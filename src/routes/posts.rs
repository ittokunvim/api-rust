use crate::db::post::NewModel as NewPost;
use crate::db::{Mutation, Query};

use crate::AppState;
use crate::errors::CustomError;

use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};
use serde_json::to_string_pretty;

#[get("/posts")]
async fn list(data: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    let conn = &data.conn;

    let posts = Query::find_posts_in_page(conn, 1, 10).await?;
    let posts = to_string_pretty(&posts).unwrap();

    Ok(HttpResponse::Ok().body(posts))
}

#[post("/posts")]
async fn create(data: web::Data<AppState>, post_form: web::Json<NewPost>) -> Result<HttpResponse, CustomError> {
    let conn = &data.conn;
    let form = post_form.into_inner();

    let post = Mutation::create_post(conn, form).await?;
    let post = to_string_pretty(&post).unwrap();

    Ok(HttpResponse::Created().body(post))
}

#[get("/posts/{id}")]
async fn detail(data: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let conn = &data.conn;
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();

    let post = Query::find_post_by_id(conn, id).await?;
    let post = to_string_pretty(&post).unwrap();

    Ok(HttpResponse::Ok().body(post))
}

#[patch("/posts/{id}")]
async fn update(
    data: web::Data<AppState>,
    req: HttpRequest,
    post_form: web::Json<NewPost>,
) -> Result<HttpResponse, CustomError> {
    let conn = &data.conn;
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();
    let form = post_form.into_inner();

    let post = Mutation::update_post_by_id(conn, id, form).await?;
    let post = to_string_pretty(&post).unwrap();

    Ok(HttpResponse::Ok().body(post))
}

#[delete("/posts/{id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let conn = &data.conn;
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();

    let post = Mutation::delete_post_by_id(conn, id).await?;
    let post = to_string_pretty(&post).unwrap();

    Ok(HttpResponse::Ok().body(post))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(detail);
    cfg.service(update);
    cfg.service(delete);
}
