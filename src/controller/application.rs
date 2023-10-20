use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use serde_json::json;

use crate::{
    app_data::AppData,
    model::application::{
        create_new_application, get_application_by_key, ApplicationErrors, ApplicationKey,
        NewApplication,
    },
};

pub fn application_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/application")
        .service(create_application)
        .service(get_application);

    config.service(scope);
}

#[post("/")]
pub async fn create_application(
    data: web::Data<AppData>,
    // req_user: Option<ReqData<Claims>>,
    new_application: web::Json<NewApplication>,
) -> impl Responder {
    // let user_id = req_user.unwrap().id;

    let app = create_new_application(&data.pg_conn, &new_application.name).await;

    match app {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => match err {
            ApplicationErrors::AppAllreadyExists => HttpResponse::BadRequest(),
            ApplicationErrors::DbError(err) => {
                println!("{}", err);
                HttpResponse::InternalServerError()
            }
            ApplicationErrors::AppDoesNotExist => HttpResponse::InternalServerError(),
        }
        .into(),
    }
}

#[get("/")]
pub async fn get_application(
    data: web::Data<AppData>,
    req_user: Option<ReqData<ApplicationKey>>,
) -> impl Responder {
    println!("get_application");

    let app = get_application_by_key(&data.pg_conn, &req_user.unwrap().key).await;

    match app {
        Ok(app) => {
            println!("{:#?}", app);
            HttpResponse::Ok().json(json!(app))
        }
        Err(err) => match err {
            ApplicationErrors::AppAllreadyExists => HttpResponse::InternalServerError(),
            ApplicationErrors::DbError(err) => {
                println!("{}", err);
                HttpResponse::InternalServerError()
            }
            ApplicationErrors::AppDoesNotExist => HttpResponse::BadRequest(),
        }
        .into(),
    }
}
