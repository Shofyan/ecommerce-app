mod domain;
mod application;
mod infrastructure;
mod presentation;

use std::sync::Arc;
use anyhow::Result;

use infrastructure::{create_connection_pool, SqliteProductRepository};
use application::ProductService;
use presentation::{create_router, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting E-commerce Application with Clean Architecture...");
    
    // Infrastructure Layer - Database setup
    let pool = create_connection_pool().await?;
    println!("✅ Database connection established");
    
    // Infrastructure Layer - Repository implementation
    let repository = SqliteProductRepository::new(pool);
    repository.initialize().await?;
    println!("✅ Database initialized with seed data");
    
    // Application Layer - Service with dependency injection
    let product_service = Arc::new(ProductService::new(Arc::new(repository)));
    println!("✅ Application services configured");
    
    // Presentation Layer - Web framework setup
    let app_state = AppState {
        product_service,
    };
    
    let app = create_router(app_state);
    println!("✅ Web routes configured");
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🌐 Server running on http://localhost:3000");
    println!("📚 API Documentation:");
    println!("  - Home Page: http://localhost:3000");
    println!("  - REST API: http://localhost:3000/api/products");
    println!("  - Health Check: http://localhost:3000/health");
    
    // Serve the application
    axum::serve(listener, app).await?;
    
    Ok(())
}