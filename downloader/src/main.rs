use downloader::{
    self,
    announcements::check_announcements,
    auth::Client,
    calendar::import_calendars,
    content::import_personalized_contents,
    events::{import_hidden_event_schedules, import_public_event_schedules},
};
use minio::s3::{args::SetBucketPolicyArgs, http::BaseUrl};
use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs},
    creds::StaticProvider,
};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    // TODO: Proper CLI
    tracing_subscriber::fmt().init();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://nintendo:yesterday@localhost:5434/archive".to_string()
    });
    let minio_url = std::env::var("MINIO_URL").unwrap_or_else(|_| {
        "http://localhost:9000".to_string()
    });
    let minio_access_key = std::env::var("MINIO_ACCESS_KEY").unwrap_or_else(|_| {
        "minioadmin".to_string()
    });
    let minio_secret_key = std::env::var("MINIO_SECRET_KEY").unwrap_or_else(|_| {
        "minioadmin".to_string()
    });
    let db: DatabaseConnection =
        Database::connect(db_url)
            .await
            .expect("Failed to connect");
    let storage_base_url: BaseUrl = minio_url.parse().unwrap();
    let storage_creds_provider = StaticProvider::new(&minio_access_key, &minio_secret_key, None);
    let storage = minio::s3::client::Client::new(
        storage_base_url,
        Some(Box::new(storage_creds_provider)),
        None,
        None,
    )
    .expect("Failed to create MinIO client");
    if storage
        .bucket_exists(&BucketExistsArgs::new("assets").unwrap())
        .await
        .unwrap()
    {
        tracing::debug!("Bucket already exists");
    } else {
        tracing::info!("Creating bucket");
        storage
            .make_bucket(&MakeBucketArgs::new("assets").unwrap())
            .await
            .unwrap();
    }
    let config = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::assets/*"
        }
    ]
}
"#;
    storage.set_bucket_policy(&SetBucketPolicyArgs::new("assets", config).unwrap()).await.unwrap();
    let locale = "en-US";
    nintendo_yesterday_db::migrator::run(&db).await.unwrap();
    let mut client = Client::new("https://prod-server.de4taiqu.srv.nintendo.net/".to_string());
    check_announcements(&mut client, locale).await;
    //import_tags(&mut client, locale, &storage, &db).await;
    import_calendars(&mut client, locale, &storage, &db).await;
    import_hidden_event_schedules(&mut client, locale, &storage, &db).await;
    let intervals = [
        // Just for fun
        ("1900-01-01", "1969-01-01"),
        ("1968-01-01", "2000-01-01"),
        ("2000-01-01", "2025-03-27"),
        // Probably 99% of interesting events
        ("2025-03-26", "2025-04-26"),
        ("2025-04-25", "2025-05-26"),
        ("2025-05-25", "2025-06-26"),
        ("2025-06-25", "2025-07-26"),
        ("2025-07-25", "2025-08-26"),
        ("2025-08-25", "2025-09-26"),
        ("2025-09-25", "2025-10-26"),
        ("2025-10-25", "2025-11-26"),
        ("2025-11-25", "2025-12-26"),
        ("2025-12-25", "2026-01-26"),
        // Also just for fun
        ("2026-01-01", "2050-12-31"),
        ("2049-01-01", "2100-01-01"),
        ("9990-01-01", "9999-12-31"),
    ];
    for (start_time, end_time) in intervals {
        import_public_event_schedules(&mut client, locale, start_time, end_time, &storage, &db)
            .await;
    }
    import_personalized_contents(&mut client, locale, &storage, &db).await;
}
