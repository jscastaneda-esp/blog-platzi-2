use rocket::serde::{Deserialize, Serialize};
use sqlx::{error::Error, PgPool, Row};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

impl Post {
    fn slugify(title: &String) -> String {
        title.replace(" ", "").to_lowercase()
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Post>, Error> {
        let rows = sqlx::query("SELECT * FROM posts").fetch_all(pool).await?;

        let result = rows
            .iter()
            .map(|row| Post {
                id: row.get::<i32, _>("id"),
                title: row.get::<String, _>("title"),
                slug: row.get::<String, _>("slug"),
                body: row.get::<String, _>("body"),
            })
            .collect::<Vec<Post>>();

        Ok(result)
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Post, Error> {
        let row = sqlx::query("SELECT * FROM posts WHERE slug = $1")
            .bind(slug)
            .fetch_one(pool)
            .await?;

        Ok(Post {
            id: row.get::<i32, _>("id"),
            title: row.get::<String, _>("title"),
            slug: row.get::<String, _>("slug"),
            body: row.get::<String, _>("body"),
        })
    }

    pub async fn create_post<'a>(pool: &PgPool, post: &NewPostHandler) -> Result<Post, Error> {
        let slug = Post::slugify(&post.title);

        let row = sqlx::query(
            "
INSERT INTO posts (title, slug, body)
VALUES ($1, $2, $3)
RETURNING id
        ",
        )
        .bind(&post.title)
        .bind(&slug)
        .bind(&post.body)
        .fetch_one(pool)
        .await?;

        let id = row.get::<i32, _>("id");

        Ok(Post {
            id: id,
            title: post.title.clone(),
            slug: slug,
            body: post.body.clone(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}
