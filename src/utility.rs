use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use sqlx::FromRow;

pub mod application;

#[derive(Debug, FromRow, Deserialize)]
pub struct Count {
    pub count: i64,
}

// generates a random string provided string length
// if the string length is less than 1, then the program panics
pub fn genarate_rand_string(str_len: usize) -> String {
    if str_len < 1 {
        panic!("The String length must be at least 1.")
    }
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(str_len)
        .map(char::from)
        .collect()
}

// generates a random string provided string length
// if the string length is less than 1, then the program returns error
pub fn _safe_genarate_rand_string(str_len: usize) -> Result<String, &'static str> {
    if str_len < 1 {
        return Err("The String length must be at least 1.");
    }
    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(str_len)
        .map(char::from)
        .collect())
}
