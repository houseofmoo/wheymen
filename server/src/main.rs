mod actions;
mod api;
mod model;
mod resource;

use actix_files::Files;
use actix_web::{
    guard::{self, GuardContext},
    web, App, HttpServer,
};
use dotenv::dotenv;
use model::env_var::EnvVar;
use std::env;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8080;

fn validate_host(ctx: &GuardContext) -> bool {
    match ctx.head().headers().get("host") {
        Some(host) => match host.to_str() {
            Ok(host_name) => match host_name {
                "localhost:8080" => true,
                "localhost" => true,
                "0.0.0.0:8080" => true,
                "0.0.0.0" => true,
                "192.168.50.215:8080" => true,
                "192.168.50.215" => true,
                "database_c:8000" => true,
                "wheymen.net" => true,
                _ => false,
            },
            Err(_) => false,
        },
        None => false,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let vars = get_env_vars();
    let decoder = resource::auth::JwtDecoder::new(vars.jwt_token);
    println!("listening on {}:{}", HOST, PORT);

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
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::fn_guard(validate_host))
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
                    )
                    .service(
                        web::scope("/sessions")
                            .service(api::session::get_all_sessions)
                            .service(api::session::get_session)
                            .service(api::session::start_session)
                            .service(api::session::update_session)
                            .service(api::session::delete_session)
                    ),
            )
            .service(
                Files::new("/", "./dist")
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind((HOST, PORT))?
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
