use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::DateTime;
use chrono::Utc;


#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub role: String,          
    pub banned: i64,            
    pub created_at: String,     
    pub last_login: Option<String>, 
    pub balance: i64,
    pub minutes_balance: i64,
}

impl User {
    pub fn is_banned(&self) -> bool {
        self.banned != 0
    }
    
    pub fn get_role(&self) -> &str {
        &self.role
    }
    
    pub fn created_at_parsed(&self) -> Option<DateTime<Utc>> {
        DateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| DateTime::parse_from_rfc3339(&self.created_at))
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
    
    pub fn last_login_parsed(&self) -> Option<DateTime<Utc>> {
        self.last_login.as_ref()
            .and_then(|s| {
                DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .or_else(|_| DateTime::parse_from_rfc3339(s))
                    .ok()
            })
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UpdateBalance {
    pub balance: i64,
    pub minutes_balance: i64,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
}
