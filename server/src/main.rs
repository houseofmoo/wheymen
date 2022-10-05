mod actions;
mod api;
mod model;
mod resource;

use actix_files::Files;
use actix_web::{
    guard::{self},
    web, App, HttpServer,
};
use dotenv::dotenv;
use model::env_var::EnvVar;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 8080;
   
    let vars = get_env_vars();
    let decoder = resource::auth::JwtDecoder::new(vars.jwt_token);
    
    println!("listening on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(resource::client::DbClient::new(
                vars.db_url.clone(),
                vars.db_user.clone(),
                vars.db_pass.clone(),
            )))
            .app_data(web::Data::new(decoder.clone()))
            .service(
                web::scope("/api")
                    .guard(guard::Host(host))
                    .guard(guard::Header("content-type", "application/json"))
                    .service(
                        web::scope("/routines")
                            .service(api::routine::get_all_routines)
                            .service(api::routine::get_routine)
                            .service(api::routine::insert_routine)
                            .service(api::routine::update_routine)
                            .service(api::routine::delete_routine),
                    )
                    .service(
                        web::scope("/workouts")
                            .service(api::workout::get_all_workouts)
                            .service(api::workout::get_all_unrelated_workouts)
                            .service(api::workout::get_workout)
                            .service(api::workout::insert_workout)
                            .service(api::workout::update_workout)
                            .service(api::workout::delete_workout),
                    ),
            )
            .service(
                Files::new("/", "./dist")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind((host, port))?
    .run()
    .await
}

fn get_env_vars() -> EnvVar {
    match dotenv() {
        Ok(_) => (),
        Err(e) => panic!(".env file mssing: {}", e.to_string()),
    };

    EnvVar {
        jwt_token: match env::var("JWT_TOKEN") {
            Ok(key) => key,
            Err(e) => panic!("Missing JWT_TOKEN environment variable: {}", e.to_string()),
        },
        db_url: match env::var("DB_URL") {
            Ok(key) => key,
            Err(e) => panic!("Missing DB_URL environment variable: {}", e.to_string()),
        },
        db_user: match env::var("DB_USER") {
            Ok(key) => key,
            Err(e) => panic!("Missing DB_USER environment variable: {}", e.to_string()),
        },
        db_pass: match env::var("DB_PASS") {
            Ok(key) => key,
            Err(e) => panic!("Missing DB_PASS environment variable: {}", e.to_string()),
        },
    }
}
