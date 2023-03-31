use crate::api_error::ApiError;
use crate::db;
use crate::schema::posts;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, PartialEq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Validate, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostParams {
    #[validate(custom = "validate_title")]
    pub title: String,
    #[validate(custom = "validate_body")]
    pub body: String,
    pub updated_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = &mut db::connection()?;
        let posts = posts::table.load::<Post>(conn)?;
        Ok(posts)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = posts::table.filter(posts::id.eq(id)).first(conn)?;
        Ok(post)
    }

    pub fn create(post: PostParams) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn update(id: Uuid, post: PostParams) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = PostParams::from(post);
        let post = diesel::update(posts::table)
            .filter(posts::id.eq(id))
            .set(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn delete(id: Uuid) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = diesel::delete(posts::table)
            .filter(posts::id.eq(id))
            .get_result(conn)?;
        Ok(post)
    }
}

impl From<PostParams> for Post {
    fn from(post: PostParams) -> Self {
        Post {
            id: Uuid::new_v4(),
            title: post.title,
            body: post.body,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl PostParams {
    fn from(post: PostParams) -> Self {
        PostParams {
            title: post.title,
            body: post.body,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PostFindAll {
    pub total_count: usize,
    pub posts: Vec<Post>,
}

impl PostFindAll {
    pub fn new(total_count: usize, posts: Vec<Post>) -> Self {
        PostFindAll { total_count, posts }
    }
}

fn validate_title(title: &str) ->Result<(), ValidationError> {
    if title.is_empty() {
        return Err(ValidationError::new("title is required"));
    } else if title.len() > 256 {
        return Err(ValidationError::new("title is too long"));
    }

    Ok(())
}

fn validate_body(body: &str) ->Result<(), ValidationError> {
    if body.is_empty() {
        return Err(ValidationError::new("body is required"));
    } else if body.len() > 65536 {
        return Err(ValidationError::new("body is too long"));
    }

    Ok(())
}