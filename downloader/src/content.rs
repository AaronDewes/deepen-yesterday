use deepen_api::models::{PostUserContentImpressions, UpdateUserContent, UserContent};

use minio::s3::args::PutObjectArgs;
use nintendo_yesterday_db::entities::prelude::*;
use nintendo_yesterday_db::entities::*;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;

use crate::auth::Client;
use crate::storage::{AdditionalOptions, download_file_to_minio, download_m3u8_as_mp4_to_minio};
use crate::tags::import_tag;

async fn import_and_rewrite_content_html(
    html_url: &str,
    token: &str,
    storage: &minio::s3::client::Client,
    content_id: &str,
) {
    let local_set = tokio::task::LocalSet::new();
    let client = reqwest::Client::new();
    let base_url = url::Url::parse(html_url).unwrap();
    println!("Downloading HTML from {}", html_url);
    let cookie = format!("__token__={}", token);
    println!("Cookie is {}", cookie);
    let html_resp = client
        .get(html_url)
        .header("User-Agent", "Mozilla/5.0 (Linux; Android 13; sdk_gphone64_x86_64 Build/TE1A.240213.009; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/109.0.5414.123 Mobile Safari/537.36")
        .header("cookie", cookie.clone())
        .header("x-requested-with", "com.nintendo.znsa")
        .send()
        .await
        .expect("Failed to download HTML");
    if html_resp.status() != reqwest::StatusCode::OK {
        panic!("Failed to download HTML: {}", html_resp.status());
    }
    let html = html_resp.text().await.expect("Failed to read HTML");
    let doc = dom_query::Document::from(html);
    for node in doc.select("link").iter() {
        if let Some(href) = node.attr("href") {
            if href == "https://www.nintendo.com/jp/app/files/template/assets/css/style.css".into()
            {
                node.set_attr("href", "/nintendo-content.css");
            } else if href.starts_with("./assets/css") {
                let src_url = base_url.join(&href.to_string()).unwrap();
                let src_path = src_url.path().trim_start_matches('/');
                // Download to minio
                let asset_object_inner = format!("contents/{}", src_path);
                download_file_to_minio(
                    &src_url.to_string(),
                    "assets",
                    &asset_object_inner,
                    storage,
                    AdditionalOptions::new(cookie.clone(), "text/css".to_string()),
                )
                .await;
                node.set_attr("href", &format!("/assets/contents/{}", src_path));
            } else {
                println!("Unknown link: {}", href);
                todo!()
            }
        }
    }
    for img in doc.select("img").iter() {
        if let Some(src) = img.attr("src") {
            let src_url = base_url.join(&src.to_string()).unwrap();
            let src_path = src_url.path().trim_start_matches('/');
            // Download to minio
            let asset_object_inner = format!("contents/{}", src_path);
            download_file_to_minio(
                &src_url.to_string(),
                "assets",
                &asset_object_inner,
                storage,
                AdditionalOptions::cookie(cookie.clone()),
            )
            .await;
            // Rewrite the URL
            img.set_attr("src", &format!("/assets/contents/{}", src_path));
        }
    }
    for script in doc.select("script").iter() {
        if let Some(src) = script.attr("src") {
            if src == "/webview/common/js/for-parent.js".into() {
                script.remove();
            } else if src
                == "https://www.nintendo.com/jp/app/files/template/assets/js/common.js".into()
            {
                script.set_attr("src", "/nintendo-common.js");
            } else if src.starts_with("./assets/js") {
                let src_url = base_url.join(&src.to_string()).unwrap();
                let src_path = src_url.path().trim_start_matches('/');
                // Download to minio
                let asset_object_inner = format!("contents/{}", src_path);
                download_file_to_minio(
                    &src_url.to_string(),
                    "assets",
                    &asset_object_inner,
                    storage,
                    AdditionalOptions::new(cookie.clone(), "application/javascript".to_string()),
                )
                .await;
                script.set_attr("src", &format!("/assets/contents/{}", src_path));
            }
        }
    }
    let css_url_regex =
        regex::Regex::new(r#"url\(['"]?([^'")]+)['"]?\)"#).expect("Failed to compile regex");
    for style in doc.select("style").iter() {
        let text = style.text().to_string();
        let new_text = css_url_regex.replace_all(&text, |caps: &regex::Captures| {
            let url = caps.get(1).unwrap().as_str();
            let cookie_clone = cookie.clone();
            if url.starts_with("https://prod-znsa.de4taiqu.srv.nintendo.net/") {
                let storage = storage.clone();
                let url_clone = url.to_string();
                // We fetch this, and replace it with the same path in /assets/ locally
                local_set.spawn_local(async move {
                    let src_url = url::Url::parse(&url_clone).unwrap();
                    let src_path = src_url.path().trim_start_matches('/');
                    // Download to minio
                    let asset_object_inner = format!("contents/{}", src_path);
                    download_file_to_minio(
                        &src_url.to_string(),
                        "assets",
                        &asset_object_inner,
                        &storage,
                        AdditionalOptions::cookie(cookie_clone),
                    )
                    .await;
                });
                return format!("url('/assets/contents/{}')", url.trim_start_matches("https://prod-znsa.de4taiqu.srv.nintendo.net/"));
            }
            println!("Unknown style URL: {}", url);
            format!("url('{}')", url)
        });
        style.set_text(&new_text);
    }
    let html = doc.try_html().expect("Failed to get HTML").to_string();
    let asset_object_inner = format!("contents/{}/content.html", content_id);
    let html_len = html.len();
    let mut html_stream = std::io::Cursor::new(html);
    let mut upload_args = PutObjectArgs::new(
        "assets",
        &asset_object_inner,
        &mut html_stream,
        Some(html_len),
        None,
    )
    .expect("Failed to create PutObjectArgs");
    upload_args.content_type = "text/html";
    storage
        .put_object(&mut upload_args)
        .await
        .expect("Failed to upload file to MinIO");
    local_set.await;
}

async fn import_content_inner(
    content: &UserContent,
    locale: &str,
    client: &mut Client,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let exists = ContentItem::find_by_id(Uuid::parse_str(&content.content.id).unwrap())
        .count(db)
        .await
        .expect("Failed to check if content exists");
    if exists > 0 {
        tracing::debug!("Content {} already exists, skipping", content.content.id);
        return;
    }
    tracing::info!("Inserting content {}", content.content.id);

    let mut out_images = vec![];
    for image in &content.content.content_image_urls {
        let image_url: url::Url = image.parse().unwrap();
        let image_filename = image_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap();
        let asset_object_inner = format!("contents/{}/{}", content.id, image_filename);
        download_file_to_minio(
            &image,
            "assets",
            &asset_object_inner,
            storage,
            Default::default(),
        )
        .await;
        out_images.push(asset_object_inner);
    }
    let mut out_movie = None;
    if !content.content.content_movie_url.is_empty() {
        let movie_url: url::Url = content.content.content_movie_url.parse().unwrap();
        let movie_filename = movie_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap();
        let asset_object_inner = format!("contents/{}/{}.mp4", content.id, movie_filename);
        download_m3u8_as_mp4_to_minio(
            &content.content.content_movie_url,
            "assets",
            &asset_object_inner,
            &content.content.akamai_header_token.as_ref().unwrap(),
            storage,
        )
        .await
        .expect("Failed to download movie");
        out_movie = Some(asset_object_inner);
    }
    let mut out_thumbnail = None;
    if !content.content.thumbnail_url.is_empty() {
        let thumbnail_url: url::Url = content.content.thumbnail_url.parse().unwrap();
        let thumbnail_filename = thumbnail_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap();
        let asset_object_inner = format!("contents/{}/{}", content.id, thumbnail_filename);
        download_file_to_minio(
            &content.content.thumbnail_url,
            "assets",
            &asset_object_inner,
            storage,
            Default::default(),
        )
        .await;
        out_thumbnail = Some(asset_object_inner);
    }
    let mut out_icon = None;
    if !content.content.icon_url.is_empty() {
        let icon_url: url::Url = content.content.icon_url.parse().unwrap();
        let icon_filename = icon_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap();
        let asset_object_inner = format!("contents/{}/{}", content.id, icon_filename);
        download_file_to_minio(
            &content.content.icon_url,
            "assets",
            &asset_object_inner,
            storage,
            Default::default(),
        )
        .await;
        out_icon = Some(asset_object_inner);
    }
    let mut out_content_body_url = None;
    if !content.content.content_body_url.is_empty() {
        import_and_rewrite_content_html(
            &content.content.content_body_url,
            &content.content.akamai_token,
            storage,
            &content.id,
        )
        .await;
        out_content_body_url = Some(format!("/assets/contents/{}/content.html", content.id));
    }
    let raw_data = serde_json::to_value(&content).unwrap();
    let model = content_item::ActiveModel {
        id: Set(Uuid::parse_str(&content.id).unwrap()),
        title: Set(content.content.title.clone()),
        category: Set(content.content.category.clone()),
        category_color: Set(Some(content.content.category_color.clone())),
        content_url: Set(Some(content.content.content_url.clone())),
        content_body_url: Set(out_content_body_url),
        title_color: Set(Some(content.content.title_color.try_into().unwrap())),
        linked_event_schedule_id: Set(if content.content.link_event_schedule_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&content.content.link_event_schedule_id).unwrap())
        }),
        is_news: Set(content.content.is_news),
        is_premiere: Set(content.content.is_premiere),
        note: Set(if content.content.note.is_empty() {
            None
        } else {
            Some(content.content.note.clone())
        }),
        panel_width: Set(content.content.panel_size_width as i32),
        panel_height: Set(content.content.panel_size_height as i32),
        content_images: Set(Some(out_images)),
        content_movie: Set(out_movie),
        icon: Set(out_icon),
        thumbnail: Set(out_thumbnail),
        panel: Set(if content.content.panel_url.is_empty() {
            None
        } else {
            Some(content.content.panel_url.clone())
        }),
        imported_at: Set(chrono::Utc::now().naive_utc()),
        raw_data: Set(raw_data),
    };
    ContentItem::insert(model)
        .exec(db)
        .await
        .expect("Failed to insert content");
    for tag in content.content.tags.iter() {
        let tag_id = tag.id.clone();
        import_tag(tag.clone(), storage, db).await;
        let tag_model = content_item_tag::ActiveModel {
            content_item_id: Set(Uuid::parse_str(&content.id).unwrap()),
            tag_id: Set(tag_id),
            id: Set(Uuid::new_v4()),
        };
        tag_model
            .insert(db)
            .await
            .expect("Failed to insert content item tag");
    }
    if content.content.is_news {
        // TODO
    } else {
        deepen_api::apis::contents_api::update_user_content(
            client.get_config(locale).await,
            locale,
            &content.content.id,
            Some("android"),
            Some("1.0.4"),
            None,
            None,
            Some(UpdateUserContent {
                read_status: Some(Some(1)),
                include_watch_list: None,
                evaluation: None,
                is_event_schedule_subscribe: None,
            }),
        )
        .await
        .expect("Failed to mark content as read!");
        deepen_api::apis::contents_api::update_user_content(
            client.get_config(locale).await,
            locale,
            &content.content.id,
            Some("android"),
            Some("1.0.4"),
            None,
            None,
            Some(UpdateUserContent {
                read_status: None,
                include_watch_list: None,
                evaluation: Some(Some(2)),
                is_event_schedule_subscribe: None,
            }),
        )
        .await
        .expect("Failed to mark content as read!");
    }
}


async fn import_content(
    content_id: &str,
    locale: &str,
    client: &mut Client,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    let exists = ContentItem::find_by_id(Uuid::parse_str(&content_id).unwrap())
        .count(db)
        .await
        .expect("Failed to check if content exists");
    if exists > 0 {
        tracing::debug!("Content {} already exists, skipping", content_id);
        return;
    }
    tracing::info!("Fetching & inserting content {}", content_id);
    let content = deepen_api::apis::contents_api::get_content(
        client.get_config(locale).await,
        locale,
        content_id,
        Some("android"),
        Some("1.0.4"),
        None,
        None,
    )
    .await;
    let content = match content {
        Ok(content) => {
            println!("Content {} found", content_id);
            content.user_content
        }
        Err(e) => {
            eprintln!("Content {} not found: {}", content_id, e);
            return;
        }
    };
    deepen_api::apis::contents_api::post_user_content_impressions(
        client.get_config(locale).await,
        locale,
        &content_id,
        Some("android"),
        Some("1.0.4"),
        None,
        None,
        Some(PostUserContentImpressions {
            event_time_stamp: chrono::Utc::now().timestamp(),
        }),
    )
    .await
    .expect("Failed to post user content impressions");
    import_content_inner(&content, locale, client, storage, db).await;
}

pub async fn import_personalized_contents(
    client: &mut Client,
    locale: &str,
    storage: &minio::s3::client::Client,
    db: &sea_orm::DatabaseConnection,
) {
    for skin_ip in [None, Some(1), Some(2), Some(3), Some(4), Some(5)] {
        let personalized_contents = deepen_api::apis::contents_api::get_personalized_contents(
            client.get_config(locale).await,
            locale,
            Some("android"),
            Some("1.0.4"),
            None,
            None,
            Some(true),
            skin_ip,
        )
        .await
        .expect("Failed to get personalized contents");
        if personalized_contents.contents.is_empty() {
            println!("No personalized contents found");
            return;
        }
        for content in personalized_contents.contents {
            import_content(&content.content.id, locale, client, storage, db).await;
        }
    }
}
