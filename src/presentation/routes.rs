use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::services::ServeDir;

use crate::presentation::handlers::{
    AppState,
    // HTML routes
    home_page, product_detail_page_handler,
    // HTMX routes
    htmx_products_list, htmx_create_product, htmx_update_product, htmx_delete_product,
    // API routes
    api_get_products, api_get_product, api_create_product, api_update_product, api_delete_product,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        
        // HTML routes for browser interface
        .route("/", get(home_page))
        .route("/products/:id", get(product_detail_page_handler))
        
        // HTMX routes for dynamic interactions
        .route("/htmx/products", get(htmx_products_list))
        .route("/htmx/products", post(htmx_create_product))
        .route("/htmx/products/:id", put(htmx_update_product))
        .route("/htmx/products/:id", delete(htmx_delete_product))
        
        // REST API routes for JSON interface
        .route("/api/products", get(api_get_products))
        .route("/api/products/:id", get(api_get_product))
        .route("/api/products", post(api_create_product))
        .route("/api/products/:id", put(api_update_product))
        .route("/api/products/:id", delete(api_delete_product))
        
        // Health check endpoint
        .route("/health", get(health_check))
        
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}