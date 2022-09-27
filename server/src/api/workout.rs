use crate::{
    actions,
    model::db::InsertWorkoutRow,
    resource::{auth::Authorized, client::DbClient},
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/get-all")]
async fn get_all_workouts(auth: Authorized, db: web::Data<DbClient>) -> impl Responder {
    match actions::workout::get_all_workouts(&auth.user_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/get-all/unrelated/{routine_id}")]
async fn get_all_unrelated_workouts(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let routine_id = path.into_inner();
    if routine_id.is_empty() {
        return HttpResponse::BadRequest().body("no workout id provided".to_string());
    }

    match actions::workout::get_all_unrelated_workouts(&auth.user_id, &routine_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/get/{workout_id}")]
async fn get_workout(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let workout_id = path.into_inner();
    if workout_id.is_empty() {
        return HttpResponse::BadRequest().body("no workout id provided".to_string());
    }

    match actions::workout::get_workout(&auth.user_id, &workout_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/insert")]
async fn insert_workout(
    auth: Authorized,
    db: web::Data<DbClient>,
    workout_row: web::Json<InsertWorkoutRow>,
) -> impl Responder {
    let workout_row = workout_row.into_inner();
    if !auth.user_id.eq(&workout_row.user_id) {
        return HttpResponse::Unauthorized().body("invalid user id".to_string());
    }

    match actions::workout::insert_workout(&workout_row, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/delete/{workout_id}")]
async fn delete_workout(
    auth: Authorized,
    db: web::Data<DbClient>,
    path: web::Path<String>,
) -> impl Responder {
    let workout_id = path.into_inner();
    if workout_id.is_empty() {
        return HttpResponse::BadRequest().body("no workout id provided".to_string());
    }

    match actions::workout::delete_workout(&auth.user_id, &workout_id, &db).await {
        Ok(r) => match r {
            Some(r) => HttpResponse::Ok().json(r),
            None => HttpResponse::NoContent().body(""),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
