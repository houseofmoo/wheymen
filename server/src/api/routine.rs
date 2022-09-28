use crate::{
    actions,
    model::db::{Upsert, InsertRoutineRow, RoutineRow},
    resource::auth::Authorized,
    resource::client::DbClient,
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/get-all")]
async fn get_all_routines(auth: Authorized, db: web::Data<DbClient>) -> impl Responder {
    match actions::routine::get_all_routines(&auth.user_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/get/{routine_id}")]
async fn get_routine(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let routine_id = path.into_inner();
    if routine_id.is_empty() {
        return HttpResponse::BadRequest().body("no routine id provided".to_string());
    }

    match actions::routine::get_routine(&auth.user_id, &routine_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/insert")]
async fn insert_routine(
    auth: Authorized,
    db: web::Data<DbClient>,
    insert: web::Json<Upsert<InsertRoutineRow>>,
) -> impl Responder {
    if !auth.user_id.eq(&insert.row.user_id) {
        return HttpResponse::Unauthorized().body("invalid user id".to_string());
    }

    match actions::routine::insert_routine(&auth.user_id, &insert.row, &insert.ids, &db)
        .await
    {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/update")]
async fn update_routine(
    auth: Authorized,
    db: web::Data<DbClient>,
    update: web::Json<Upsert<RoutineRow>>,
) -> impl Responder {
    if !auth.user_id.eq(&update.row.user_id) {
        return HttpResponse::Unauthorized().body("invalid user id".to_string());
    }

    match actions::routine::update_routine(&auth.user_id, &update.row, &update.ids, &db)
        .await
    {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/delete/{routine_id}")]
async fn delete_routine(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let routine_id = path.into_inner();
    if routine_id.is_empty() {
        return HttpResponse::BadRequest().body("no routine id provided".to_string());
    }

    match actions::routine::delete_routine(&auth.user_id, &routine_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
