use std::sync::Arc;
use crate::domain::{
    Product, ProductId, ProductName, Money, StockQuantity, 
    ProductRepository, DomainError, RepositoryError
};
use crate::application::dtos::{
    CreateProductRequest, UpdateProductRequest, ProductResponse, SearchProductsQuery
};

/// Application service for product operations
pub struct ProductService {
    repository: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    /// Create a new product
    pub async fn create_product(
        &self,
        request: CreateProductRequest,
    ) -> Result<ProductResponse, ApplicationError> {
        // Validate input
        let name = ProductName::new(request.name)
            .map_err(ApplicationError::DomainError)?;
        let price = Money::new(request.price)
            .map_err(ApplicationError::DomainError)?;
        let stock = StockQuantity::new(request.stock)
            .map_err(ApplicationError::DomainError)?;

        // Get next ID
        let id = self.repository.next_id().await
            .map_err(ApplicationError::RepositoryError)?;

        // Create product entity
        let product = Product::new(id, name, request.description, price, stock);

        // Save to repository
        let saved_product = self.repository.save(product).await
            .map_err(ApplicationError::RepositoryError)?;

        Ok(ProductResponse::from(saved_product))
    }

    /// Get all products
    pub async fn get_all_products(&self) -> Result<Vec<ProductResponse>, ApplicationError> {
        let products = self.repository.find_all().await
            .map_err(ApplicationError::RepositoryError)?;

        Ok(products.into_iter().map(ProductResponse::from).collect())
    }

    /// Get product by ID
    pub async fn get_product_by_id(&self, id: i64) -> Result<ProductResponse, ApplicationError> {
        let product_id = ProductId::new(id)
            .map_err(ApplicationError::DomainError)?;

        let product = self.repository.find_by_id(&product_id).await
            .map_err(ApplicationError::RepositoryError)?
            .ok_or(ApplicationError::ProductNotFound)?;

        Ok(ProductResponse::from(product))
    }

    /// Update product
    pub async fn update_product(
        &self,
        id: i64,
        request: UpdateProductRequest,
    ) -> Result<ProductResponse, ApplicationError> {
        let product_id = ProductId::new(id)
            .map_err(ApplicationError::DomainError)?;

        // Find existing product
        let mut product = self.repository.find_by_id(&product_id).await
            .map_err(ApplicationError::RepositoryError)?
            .ok_or(ApplicationError::ProductNotFound)?;

        // Validate and convert updates
        let name = if let Some(name_str) = request.name {
            Some(ProductName::new(name_str).map_err(ApplicationError::DomainError)?)
        } else {
            None
        };

        let price = if let Some(price_val) = request.price {
            Some(Money::new(price_val).map_err(ApplicationError::DomainError)?)
        } else {
            None
        };

        let stock = if let Some(stock_val) = request.stock {
            Some(StockQuantity::new(stock_val).map_err(ApplicationError::DomainError)?)
        } else {
            None
        };

        // Update product
        product.update(name, Some(request.description), price, stock)
            .map_err(ApplicationError::DomainError)?;

        // Save updated product
        let updated_product = self.repository.update(product).await
            .map_err(ApplicationError::RepositoryError)?;

        Ok(ProductResponse::from(updated_product))
    }

    /// Delete product
    pub async fn delete_product(&self, id: i64) -> Result<bool, ApplicationError> {
        let product_id = ProductId::new(id)
            .map_err(ApplicationError::DomainError)?;

        // Check if product exists
        let exists = self.repository.exists(&product_id).await
            .map_err(ApplicationError::RepositoryError)?;

        if !exists {
            return Err(ApplicationError::ProductNotFound);
        }

        // Delete product
        let deleted = self.repository.delete(&product_id).await
            .map_err(ApplicationError::RepositoryError)?;

        Ok(deleted)
    }

    /// Search products
    pub async fn search_products(
        &self,
        query: SearchProductsQuery,
    ) -> Result<Vec<ProductResponse>, ApplicationError> {
        let products = match query.query {
            Some(search_term) if !search_term.trim().is_empty() => {
                self.repository.search_by_name(&search_term).await
                    .map_err(ApplicationError::RepositoryError)?
            }
            _ => {
                self.repository.find_all().await
                    .map_err(ApplicationError::RepositoryError)?
            }
        };

        Ok(products.into_iter().map(ProductResponse::from).collect())
    }

    /// Check if product exists
    #[allow(dead_code)]
    pub async fn product_exists(&self, id: i64) -> Result<bool, ApplicationError> {
        let product_id = ProductId::new(id)
            .map_err(ApplicationError::DomainError)?;

        self.repository.exists(&product_id).await
            .map_err(ApplicationError::RepositoryError)
    }
}

/// Application layer errors
#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Product not found")]
    ProductNotFound,
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[allow(dead_code)]
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[allow(dead_code)]
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[allow(dead_code)]
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ApplicationError {
    #[allow(dead_code)]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    #[allow(dead_code)]
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::AuthorizationError(message.into())
    }

    #[allow(dead_code)]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError(message.into())
    }
}