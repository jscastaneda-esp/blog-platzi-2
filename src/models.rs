use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

impl Post {
    fn slugify(title: &String) -> String {
        title.replace(" ", "to").to_lowercase()
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
        posts::table.load::<Post>(conn)
    }

    pub fn find_by_slug(
        conn: &mut PgConnection,
        slug: &str,
    ) -> Result<Post, diesel::result::Error> {
        posts::table
            .filter(posts::slug.eq(slug))
            .first::<Post>(conn)
    }

    pub fn create_post<'a>(
        conn: &mut PgConnection,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title);

        let new_post = NewPost {
            title: &post.title,
            body: &post.body,
            slug: &slug,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}
