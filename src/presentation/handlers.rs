use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
    Form,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::application::{
    ProductService, CreateProductRequest, UpdateProductRequest, 
    ProductResponse, SearchProductsQuery, ApiResponse, ApplicationError
};
use crate::presentation::templates::{
    products_page, product_detail_page, product_list_partial, product_card
};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub product_service: Arc<ProductService>,
}

// ============================================================================
// HTML Handlers for Browser Interface
// ============================================================================

pub async fn home_page(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    match state.product_service.get_all_products().await {
        Ok(products) => {
            let html = products_page(&products);
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn product_detail_page_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, StatusCode> {
    match state.product_service.get_product_by_id(id).await {
        Ok(product) => {
            let html = product_detail_page(&product);
            Ok(Html(html))
        }
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ============================================================================
// HTMX Handlers for Dynamic Updates
// ============================================================================

#[derive(Deserialize)]
pub struct HtmxSearchQuery {
    search: Option<String>,
}

pub async fn htmx_products_list(
    State(state): State<AppState>,
    Query(params): Query<HtmxSearchQuery>,
) -> Result<Html<String>, StatusCode> {
    let query = SearchProductsQuery {
        query: params.search,
        limit: None,
        offset: None,
    };

    match state.product_service.search_products(query).await {
        Ok(products) => {
            let html = product_list_partial(&products);
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn htmx_create_product(
    State(state): State<AppState>,
    Form(form): Form<CreateProductRequest>,
) -> Result<Html<String>, StatusCode> {
    match state.product_service.create_product(form).await {
        Ok(product) => {
            let html = product_card(&product);
            Ok(Html(html))
        }
        Err(ApplicationError::DomainError(_)) => Err(StatusCode::BAD_REQUEST),
        Err(ApplicationError::ValidationError(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn htmx_update_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateProductRequest>,
) -> Result<Html<String>, StatusCode> {
    match state.product_service.update_product(id, form).await {
        Ok(product) => {
            let html = product_card(&product);
            Ok(Html(html))
        }
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(ApplicationError::DomainError(_)) => Err(StatusCode::BAD_REQUEST),
        Err(ApplicationError::ValidationError(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn htmx_delete_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, StatusCode> {
    match state.product_service.delete_product(id).await {
        Ok(true) => Ok(Html(String::new())), // Empty response removes the element
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ============================================================================
// REST API Handlers for JSON Interface  
// ============================================================================

pub async fn api_get_products(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, StatusCode> {
    match state.product_service.get_all_products().await {
        Ok(products) => Ok(Json(ApiResponse::success(products))),
        Err(err) => {
            let error_msg = format!("Failed to retrieve products: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
    }
}

pub async fn api_get_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ProductResponse>>, StatusCode> {
    match state.product_service.get_product_by_id(id).await {
        Ok(product) => Ok(Json(ApiResponse::success(product))),
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            let error_msg = format!("Failed to retrieve product: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
    }
}

pub async fn api_create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, StatusCode> {
    match state.product_service.create_product(request).await {
        Ok(product) => Ok(Json(ApiResponse::success(product))),
        Err(ApplicationError::DomainError(err)) => {
            let error_msg = format!("Invalid product data: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
        Err(ApplicationError::ValidationError(err)) => {
            Ok(Json(ApiResponse::validation_error(vec![err])))
        }
        Err(err) => {
            let error_msg = format!("Failed to create product: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
    }
}

pub async fn api_update_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, StatusCode> {
    match state.product_service.update_product(id, request).await {
        Ok(product) => Ok(Json(ApiResponse::success(product))),
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(ApplicationError::DomainError(err)) => {
            let error_msg = format!("Invalid product data: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
        Err(ApplicationError::ValidationError(err)) => {
            Ok(Json(ApiResponse::validation_error(vec![err])))
        }
        Err(err) => {
            let error_msg = format!("Failed to update product: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
    }
}

pub async fn api_delete_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.product_service.delete_product(id).await {
        Ok(true) => Ok(Json(ApiResponse::success("Product deleted successfully".to_string()))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(ApplicationError::ProductNotFound) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            let error_msg = format!("Failed to delete product: {}", err);
            Ok(Json(ApiResponse::error(error_msg)))
        }
    }
}