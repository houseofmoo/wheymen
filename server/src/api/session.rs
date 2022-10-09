use crate::{actions, resource::auth::Authorized, resource::client::DbClient, model::session::Session};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/get-all")]
async fn get_all_sessions(
    auth: Authorized,
    db: web::Data<DbClient>,
) -> impl Responder {
    match actions::session::get_all_sessions(&auth.user_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/get/{session_id}")]
async fn get_session(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("no session id provided".to_string());
    }

    match actions::session::get_session(&auth.user_id, &session_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/start/{routine_id}")]
async fn start_session(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let routine_id = path.into_inner();
    if routine_id.is_empty() {
        return HttpResponse::BadRequest().body("no routine id provided".to_string());
    }

    match actions::session::start_session(&auth.user_id, &routine_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/update")]
async fn update_session(
    _auth: Authorized,
    db: web::Data<DbClient>,
    session: web::Json<Session>,
) -> impl Responder {
    match actions::session::update_session(&session, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/delete/{session_id}")]
async fn delete_session(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("no session id provided".to_string());
    }

    match actions::session::delete_session(&auth.user_id, &session_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}