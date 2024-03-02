#[derive(serde::Deserialize, Debug)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

pub struct Query {
    // todo
}
