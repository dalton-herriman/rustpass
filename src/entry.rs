use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}
