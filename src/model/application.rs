use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::utility::genarate_rand_string;

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub created_date: DateTime<Local>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewApplication {
    pub name: String,
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationKey {
    pub key: String,
}

#[derive(Debug)]
pub enum ApplicationErrors {
    AppAllreadyExists,
    AppDoesNotExist,
    DbError(sqlx::Error),
}

pub async fn create_new_application(pool: &PgPool, name: &str) -> Result<(), ApplicationErrors> {
    let app = get_application_by_name(pool, name).await;

    match app {
        Ok(app) => {
            println!("{:#?}", app);
            return Err(ApplicationErrors::AppAllreadyExists);
        }
        Err(err) => match err {
            ApplicationErrors::AppAllreadyExists => {
                panic!("Some error occurred. coz this is not a valid path.")
            }
            ApplicationErrors::DbError(err) => {
                println!("{}", err);
                return Err(ApplicationErrors::DbError(err));
            }
            _ => (),
        },
    }

    let key = format!("{}_{}", name, genarate_rand_string(7));

    let query = "INSERT INTO application (name, key) VALUES ($1, $2)";

    let query = sqlx::query(query).bind(name).bind(key).execute(pool).await;

    match query {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("error: {}", err);
            Err(ApplicationErrors::DbError(err))
        }
    }
}

pub async fn get_application_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Application, ApplicationErrors> {
    let query = "SELECT * FROM application WHERE name = $1";

    let query = sqlx::query_as::<_, Application>(query).bind(&name);

    let apps = query.fetch_all(pool).await;

    match apps {
        Ok(apps) => {
            // println!("{:#?}", apps);
            if apps.len() < 1 {
                Err(ApplicationErrors::AppDoesNotExist)
            } else {
                Ok(apps.get(0).unwrap().to_owned())
            }
        }
        Err(err) => {
            println!("Something went wrong with the query for retrieving the application data for the application name {}\n error : {}", name, err);
            Err(ApplicationErrors::DbError(err))
        }
    }
}
pub async fn get_application_by_key(
    pool: &PgPool,
    key: &str,
) -> Result<Application, ApplicationErrors> {
    let query = "SELECT * FROM application WHERE key = $1";

    let query = sqlx::query_as::<_, Application>(query).bind(&key);

    let apps = query.fetch_all(pool).await;

    match apps {
        Ok(apps) => {
            // println!("{:#?}", apps);
            if apps.len() < 1 {
                Err(ApplicationErrors::AppDoesNotExist)
            } else {
                Ok(apps.get(0).unwrap().to_owned())
            }
        }
        Err(err) => {
            println!("Something went wrong with the query for retrieving the application data for the application key {}\n error : {}", key, err);
            Err(ApplicationErrors::DbError(err))
        }
    }
}
