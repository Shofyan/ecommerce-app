use async_trait::async_trait;
use crate::domain::entities::{Product, ProductId, DomainError};

/// Repository trait for Product aggregate
#[async_trait]
pub trait ProductRepository: Send + Sync {
    /// Find all products
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError>;
    
    /// Find product by ID
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError>;
    
    /// Search products by name
    async fn search_by_name(&self, query: &str) -> Result<Vec<Product>, RepositoryError>;
    
    /// Save a new product
    async fn save(&self, product: Product) -> Result<Product, RepositoryError>;
    
    /// Update existing product
    async fn update(&self, product: Product) -> Result<Product, RepositoryError>;
    
    /// Delete product by ID
    async fn delete(&self, id: &ProductId) -> Result<bool, RepositoryError>;
    
    /// Check if product exists
    async fn exists(&self, id: &ProductId) -> Result<bool, RepositoryError>;
    
    /// Get next available ID (for new products)
    async fn next_id(&self) -> Result<ProductId, RepositoryError>;
}

/// Repository specific errors
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum RepositoryError {
    #[allow(dead_code)]
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Database query failed: {0}")]
    QueryFailed(String),
    #[error("Product not found")]
    NotFound,
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    #[allow(dead_code)]
    #[error("Concurrent modification detected")]
    ConcurrentModification,
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::Database(db_err) => {
                RepositoryError::ConstraintViolation(db_err.to_string())
            }
            _ => RepositoryError::QueryFailed(err.to_string()),
        }
    }
}