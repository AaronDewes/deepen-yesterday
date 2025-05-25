use crate::auth::Client;

pub async fn check_announcements(client: &mut Client, locale: &str) {
    let announcements = deepen_api::apis::announcements_api::get_announcements(
        client.get_config(locale).await,
        locale,
        Some("android"),
        Some("1.0.4"),
        None,
        None,
        None,
        None,
    )
    .await
    .expect("Failed to get announcements");
    if announcements.total_count.is_some_and(|x| x > 0) {
        // Not implemented yet
        todo!();
    }
}
