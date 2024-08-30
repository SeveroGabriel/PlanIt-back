use mongodb::bson::oid::ObjectId;
use mongodb::error::Error;

use thiserror::Error;

use super::models::{Category, Color};
use super::repository::CategoryRepository;

#[derive(Error, Debug)]
pub enum CategoryServiceError {
    #[error("Category already exists")]
    CategoryAlreadyExists,

    #[error("Database error occurred: {0}")]
    DatabaseError(#[from] Error),
}

pub struct CategoryService {
    repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(repository: CategoryRepository) -> Self {
        CategoryService { repository }
    }

    pub async fn create_category_for_user(
        &self,
        &user_id: &ObjectId,
        title: String,
        color: Color,
    ) -> Result<ObjectId, CategoryServiceError> {
        if let Some(_existing_category) = self
            .repository
            .get_category_by_title(&user_id, &title)
            .await?
        {
            return Err(CategoryServiceError::CategoryAlreadyExists);
        }
        let new_category = Category {
            id: None,
            user_id,
            title,
            color,
        };

        let result = self.repository.create_category(new_category).await?;
        Ok(result)
    }

    pub async fn get_all_user_categories(
        &self,
        &user_id: &ObjectId,
    ) -> Result<Vec<Category>, Error> {
        self.repository.get_all_user_categories(&user_id).await
    }
}
