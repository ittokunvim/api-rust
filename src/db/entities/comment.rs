//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use sea_orm_migration::async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::prelude::Post;

use crate::errors::ErrorResponse;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub text: String,
    pub post_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(DeriveIntoActiveModel, Serialize, Deserialize)]
pub struct NewModel {
    pub text: String,
    pub post_id: i32,
}

#[derive(Serialize)]
pub struct ListModel {
    pub total_count: u64,
    pub comments: Vec<Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Post",
        from = "Column::PostId",
        to = "super::post::Column::Id"
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Post,
}

impl Related<Post> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut errors: Vec<String> = vec![];

        let text = self.text.as_ref();

        if let Err(err) = check_text(text) {
            errors.push(ErrorResponse::new(422, err));
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            let errors = errors.join(",");
            Err(DbErr::Custom(errors))
        }
    }
}

fn check_text(text: &str) -> Result<(), &str> {
    if text.is_empty() {
        Err("text cannot be empty")
    } else if text.len() > 65536 {
        Err("text cannot be longer than 65536 characters")
    } else {
        Ok(())
    }
}
