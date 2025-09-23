use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::Product;

/// Request DTO for creating a new product
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

/// Request DTO for updating a product
#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

/// Response DTO for product data
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id().value(),
            name: product.name().value().to_string(),
            description: product.description().clone(),
            price: product.price().value(),
            stock: product.stock().value(),
            created_at: *product.created_at(),
            updated_at: *product.updated_at(),
        }
    }
}

/// Search query DTO
#[derive(Debug, Deserialize)]
pub struct SearchProductsQuery {
    pub query: Option<String>,
    #[allow(dead_code)]
    pub limit: Option<usize>,
    #[allow(dead_code)]
    pub offset: Option<usize>,
}

/// Generic API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            errors: None,
        }
    }

    pub fn validation_error(errors: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some("Validation failed".to_string()),
            errors: Some(errors),
        }
    }
}

/// Pagination response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    #[allow(dead_code)]
    pub fn new(
        items: Vec<T>,
        total: usize,
        page: usize,
        per_page: usize,
    ) -> Self {
        let has_next = (page * per_page) < total;
        let has_prev = page > 1;

        Self {
            items,
            total,
            page,
            per_page,
            has_next,
            has_prev,
        }
    }
}