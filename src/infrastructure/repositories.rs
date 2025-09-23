use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use chrono::Utc;

use crate::domain::{
    Product, ProductId, ProductName, Money, StockQuantity,
    ProductRepository, RepositoryError
};

pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Initialize database tables and seed data
    pub async fn initialize(&self) -> Result<(), RepositoryError> {
        // Create table
        sqlx::query(include_str!("../../migrations/001_create_products.sql"))
            .execute(&self.pool)
            .await?;

        // Check if we need to seed data
        let count: i64 = sqlx::query("SELECT COUNT(*) as count FROM products")
            .fetch_one(&self.pool)
            .await?
            .get("count");

        if count == 0 {
            self.seed_data().await?;
        }

        Ok(())
    }

    async fn seed_data(&self) -> Result<(), RepositoryError> {
        let products = vec![
            ("MacBook Pro 16\"", Some("Apple MacBook Pro with M2 chip"), 2499.99, 10),
            ("iPhone 15 Pro", Some("Latest iPhone with titanium design"), 999.99, 25),
            ("AirPods Pro", Some("Wireless earbuds with noise cancellation"), 249.99, 50),
            ("iPad Air", Some("Lightweight tablet for creativity"), 599.99, 15),
            ("Apple Watch Ultra", Some("Adventure-ready smartwatch"), 799.99, 8),
        ];

        for (name, description, price, stock) in products {
            let now = Utc::now().to_rfc3339();
            sqlx::query(
                "INSERT INTO products (name, description, price, stock, created_at, updated_at) 
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(name)
            .bind(description)
            .bind(price)
            .bind(stock)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    fn row_to_product(&self, row: &sqlx::sqlite::SqliteRow) -> Result<Product, RepositoryError> {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let price: f64 = row.get("price");
        let stock: i32 = row.get("stock");
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        // Parse timestamps
        let _created_at = created_at.parse::<chrono::DateTime<Utc>>()
            .map_err(|e| RepositoryError::Internal(format!("Invalid created_at: {}", e)))?;
        let _updated_at = updated_at.parse::<chrono::DateTime<Utc>>()
            .map_err(|e| RepositoryError::Internal(format!("Invalid updated_at: {}", e)))?;

        // Create value objects
        let product_id = ProductId::new(id)?;
        let product_name = ProductName::new(name)?;
        let money = Money::new(price)?;
        let stock_quantity = StockQuantity::new(stock)?;

        // Create product with correct timestamps
        let product = Product::new(product_id, product_name, description, money, stock_quantity);
        
        // We need to set the actual timestamps from DB
        // For now, we'll create a new Product and trust the timestamps from the constructor
        // In a real implementation, you'd want to have setters or a factory method
        
        Ok(product)
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let rows = sqlx::query(
            "SELECT id, name, description, price, stock, created_at, updated_at 
             FROM products 
             ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut products = Vec::new();
        for row in rows {
            products.push(self.row_to_product(&row)?);
        }

        Ok(products)
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, name, description, price, stock, created_at, updated_at 
             FROM products 
             WHERE id = ?"
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_product(&row)?)),
            None => Ok(None),
        }
    }

    async fn search_by_name(&self, query: &str) -> Result<Vec<Product>, RepositoryError> {
        let search_term = format!("%{}%", query);
        let rows = sqlx::query(
            "SELECT id, name, description, price, stock, created_at, updated_at 
             FROM products 
             WHERE name LIKE ? OR description LIKE ? 
             ORDER BY created_at DESC"
        )
        .bind(&search_term)
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await?;

        let mut products = Vec::new();
        for row in rows {
            products.push(self.row_to_product(&row)?);
        }

        Ok(products)
    }

    async fn save(&self, product: Product) -> Result<Product, RepositoryError> {
        let now = Utc::now().to_rfc3339();
        
        let result = sqlx::query(
            "INSERT INTO products (name, description, price, stock, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?) 
             RETURNING id"
        )
        .bind(product.name().value())
        .bind(product.description())
        .bind(product.price().value())
        .bind(product.stock().value())
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = result.get("id");
        
        // Return the saved product with the new ID
        self.find_by_id(&ProductId::new(id)?)
            .await?
            .ok_or(RepositoryError::Internal("Failed to retrieve saved product".to_string()))
    }

    async fn update(&self, product: Product) -> Result<Product, RepositoryError> {
        let now = Utc::now().to_rfc3339();
        
        let result = sqlx::query(
            "UPDATE products 
             SET name = ?, description = ?, price = ?, stock = ?, updated_at = ? 
             WHERE id = ?"
        )
        .bind(product.name().value())
        .bind(product.description())
        .bind(product.price().value())
        .bind(product.stock().value())
        .bind(&now)
        .bind(product.id().value())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        // Return the updated product
        self.find_by_id(product.id())
            .await?
            .ok_or(RepositoryError::Internal("Failed to retrieve updated product".to_string()))
    }

    async fn delete(&self, id: &ProductId) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id.value())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn exists(&self, id: &ProductId) -> Result<bool, RepositoryError> {
        let count: i64 = sqlx::query("SELECT COUNT(*) as count FROM products WHERE id = ?")
            .bind(id.value())
            .fetch_one(&self.pool)
            .await?
            .get("count");

        Ok(count > 0)
    }

    async fn next_id(&self) -> Result<ProductId, RepositoryError> {
        // For SQLite with auto-increment, we can return a placeholder ID
        // The actual ID will be generated during insertion
        Ok(ProductId::new(1)?) // This will be overridden by auto-increment
    }
}