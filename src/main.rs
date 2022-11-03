#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use dotenvy::dotenv;
use models::{NewPostHandler, Post};
use rocket::{http::Status, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use rocket_sync_db_pools::database;

#[database("pg_db")]
struct Db(rocket_sync_db_pools::diesel::PgConnection);

#[get("/")]
async fn index(db: Db) -> Result<Template, Status> {
    match db.run(|conn| Post::find_all(conn)).await {
        Ok(posts) => Ok(Template::render("index", context! { posts: posts })),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/blog/<blog_slug>")]
async fn get_post(db: Db, blog_slug: String) -> Result<Template, Status> {
    match db
        .run(move |conn| Post::find_by_slug(conn, &blog_slug))
        .await
    {
        Ok(post) => Ok(Template::render("post", context! { post: post })),
        Err(err) => {
            if diesel::result::Error::NotFound == err {
                return Err(Status::NotFound);
            }

            Err(Status::InternalServerError)
        }
    }
}

#[post("/new_post", data = "<item>")]
async fn new_post(db: Db, item: Json<NewPostHandler>) -> Result<Json<Post>, Status> {
    match db.run(move |conn| Post::create_post(conn, &item)).await {
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
        .attach(Db::fairing())
}
