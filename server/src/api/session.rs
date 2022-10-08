use crate::{actions, resource::auth::Authorized, resource::client::DbClient};
use actix_web::{post, web, HttpResponse, Responder};

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

#[post("/continue/{session_id}")]
async fn continue_sesion(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("no session id provided".to_string());
    }

    match actions::routine::delete_routine(&auth.user_id, &session_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/update/{session_id}")]
async fn update_session(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("no session id provided".to_string());
    }

    match actions::routine::delete_routine(&auth.user_id, &session_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/complete/{session_id}")]
async fn complete_session(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let session_id = path.into_inner();
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("no session id provided".to_string());
    }

    match actions::routine::delete_routine(&auth.user_id, &session_id, &db).await {
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

    match actions::routine::delete_routine(&auth.user_id, &session_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
