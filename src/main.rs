#[macro_use]
extern crate rocket;

mod models;

use dotenvy::dotenv;
use models::{NewPostHandler, Post};
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::sqlx;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};

#[derive(Database)]
#[database("pg_db")]
pub struct Db(sqlx::PgPool);

#[get("/")]
async fn index(db: &Db) -> Result<Template, Status> {
    match Post::find_all(&db.0).await {
        Ok(posts) => Ok(Template::render("index", context! { posts: posts })),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/blog/<blog_slug>")]
async fn get_post(db: &Db, blog_slug: String) -> Result<Template, Status> {
    match Post::find_by_slug(&db.0, &blog_slug).await {
        Ok(post) => Ok(Template::render("post", context! { post: post })),
        Err(sqlx::error::Error::RowNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/new_post", data = "<item>")]
async fn new_post(db: &Db, item: Json<NewPostHandler>) -> Result<Json<Post>, Status> {
    match Post::create_post(&db.0, &item).await {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index, get_post, new_post])
        .attach(Template::fairing())
        .attach(Db::init())
}
