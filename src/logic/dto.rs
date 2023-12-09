#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct CreateJournalRequest {
    pub title: String,
    pub year: u32,
}

#[derive(serde::Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: String,
    pub journal_id: String,
}
