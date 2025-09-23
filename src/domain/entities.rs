use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Product Entity - Core business entity
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: Option<String>,
    price: Money,
    stock: StockQuantity,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        description: Option<String>,
        price: Money,
        stock: StockQuantity,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            price,
            stock,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(
        &mut self,
        name: Option<ProductName>,
        description: Option<Option<String>>,
        price: Option<Money>,
        stock: Option<StockQuantity>,
    ) -> Result<(), DomainError> {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(price) = price {
            self.price = price;
        }
        if let Some(stock) = stock {
            self.stock = stock;
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &ProductId { &self.id }
    pub fn name(&self) -> &ProductName { &self.name }
    pub fn description(&self) -> &Option<String> { &self.description }
    pub fn price(&self) -> &Money { &self.price }
    pub fn stock(&self) -> &StockQuantity { &self.stock }
    pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    pub fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
}

/// Product ID Value Object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductId(i64);

impl ProductId {
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if value <= 0 {
            return Err(DomainError::InvalidProductId);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for ProductId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// Product Name Value Object
#[derive(Debug, Clone, PartialEq)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::InvalidProductName("Product name cannot be empty".to_string()));
        }
        if value.len() > 255 {
            return Err(DomainError::InvalidProductName("Product name too long".to_string()));
        }
        Ok(Self(value.trim().to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ProductName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Money Value Object
#[derive(Debug, Clone, PartialEq)]
pub struct Money(f64);

impl Money {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value < 0.0 {
            return Err(DomainError::InvalidMoney("Price cannot be negative".to_string()));
        }
        if value > 999999.99 {
            return Err(DomainError::InvalidMoney("Price too high".to_string()));
        }
        Ok(Self((value * 100.0).round() / 100.0)) // Round to 2 decimal places
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Money {
    type Error = DomainError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Stock Quantity Value Object
#[derive(Debug, Clone, PartialEq)]
pub struct StockQuantity(i32);

impl StockQuantity {
    pub fn new(value: i32) -> Result<Self, DomainError> {
        if value < 0 {
            return Err(DomainError::InvalidStock("Stock cannot be negative".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    #[allow(dead_code)]
    pub fn is_available(&self) -> bool {
        self.0 > 0
    }

    #[allow(dead_code)]
    pub fn decrease(&mut self, amount: i32) -> Result<(), DomainError> {
        if amount < 0 {
            return Err(DomainError::InvalidStock("Decrease amount cannot be negative".to_string()));
        }
        if self.0 < amount {
            return Err(DomainError::InsufficientStock);
        }
        self.0 -= amount;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn increase(&mut self, amount: i32) -> Result<(), DomainError> {
        if amount < 0 {
            return Err(DomainError::InvalidStock("Increase amount cannot be negative".to_string()));
        }
        self.0 += amount;
        Ok(())
    }
}

impl TryFrom<i32> for StockQuantity {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Domain Errors
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid product ID")]
    InvalidProductId,
    #[error("Invalid product name: {0}")]
    InvalidProductName(String),
    #[error("Invalid money value: {0}")]
    InvalidMoney(String),
    #[error("Invalid stock value: {0}")]
    InvalidStock(String),
    #[allow(dead_code)]
    #[error("Insufficient stock available")]
    InsufficientStock,
    #[allow(dead_code)]
    #[error("Product not found")]
    ProductNotFound,
}

/// Product Domain Events
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProductEvent {
    ProductCreated {
        product_id: ProductId,
        name: ProductName,
        price: Money,
    },
    ProductUpdated {
        product_id: ProductId,
        changes: Vec<String>,
    },
    ProductDeleted {
        product_id: ProductId,
    },
    StockChanged {
        product_id: ProductId,
        old_stock: StockQuantity,
        new_stock: StockQuantity,
    },
}