# E-commerce CRUD Application - Clean Architecture with Rust

A modern e-commerce product management system demonstrating **Clean Architecture** and **Domain-Driven Design** principles using Rust, Axum, SQLite, and HTMX.

## 🏗️ Architecture Overview

This application implements Clean Architecture with four distinct layers:

```
┌─────────────────────────────────────────┐
│           Presentation Layer            │  ← HTTP handlers, templates, routes
├─────────────────────────────────────────┤
│           Application Layer             │  ← Use cases, DTOs, services
├─────────────────────────────────────────┤
│          Infrastructure Layer           │  ← Database, external services
├─────────────────────────────────────────┤
│             Domain Layer                │  ← Business logic, entities, rules
└─────────────────────────────────────────┘
```

### Domain-Driven Design Patterns

- **Entities**: Core business objects with identity (`Product`)
- **Value Objects**: Immutable objects that describe aspects (`ProductId`, `ProductName`, `Money`, `StockQuantity`)
- **Repository Pattern**: Abstraction for data access
- **Dependency Injection**: Loose coupling between layers
- **Domain Events**: Future extensibility for business events

## ✨ Features

- **Clean Architecture**: Proper separation of concerns across layers
- **Domain-Driven Design**: Rich domain model with value objects
- **Full CRUD Operations**: Create, Read, Update, Delete products
- **REST API**: JSON endpoints following REST principles
- **HTMX Frontend**: Dynamic interactions without heavy JavaScript
- **Template Separation**: Dedicated template modules for presentation
- **SQLite Database**: Lightweight, embedded database with migrations
- **Real-time Search**: Filter products dynamically with HTMX
- **Responsive Design**: TailwindCSS with modern UI patterns
- **Dependency Injection**: Clean separation and testability
- **Error Handling**: Comprehensive error types across all layers

## 🛠️ Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: SQLite with SQLx async ORM
- **Frontend**: HTMX + TailwindCSS
- **Architecture**: Clean Architecture + Domain-Driven Design
- **Async Runtime**: Tokio
- **Error Handling**: thiserror for structured errors
- **Serialization**: serde for JSON handling

## 📋 Domain Model

### Product Entity
```rust
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: Option<String>,
    price: Money,
    stock: StockQuantity,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### Value Objects
```rust
pub struct ProductId(i64);
pub struct ProductName(String);
pub struct Money(f64);
pub struct StockQuantity(i32);
```

## 🚀 API Endpoints

### REST API (JSON)
- `GET /api/products` - List all products with optional search
- `GET /api/products/{id}` - Get single product by ID
- `POST /api/products` - Create new product
- `PUT /api/products/{id}` - Update existing product
- `DELETE /api/products/{id}` - Delete product
- `GET /health` - Health check endpoint

### HTML Routes
- `GET /` - Product catalog page (full HTML)
- `GET /products/{id}` - Product detail page

### HTMX Endpoints (Partial HTML)
- `GET /htmx/products` - Product list partial with search support
- `POST /htmx/products` - Create product (returns new product card)
- `PUT /htmx/products/{id}` - Update product (returns updated card)
- `DELETE /htmx/products/{id}` - Delete product (returns empty response)

## 📁 Project Structure

```
ecommerce-app/
├── Cargo.toml                      # Dependencies and project configuration
├── src/
│   ├── main.rs                     # Application entry point & DI setup
│   ├── domain/                     # 🎯 Domain Layer
│   │   ├── mod.rs                  # Domain module exports
│   │   ├── entities.rs             # Product entity & value objects
│   │   └── repositories.rs         # Repository trait definitions
│   ├── application/                # 🔧 Application Layer
│   │   ├── mod.rs                  # Application module exports
│   │   ├── dtos.rs                 # Request/Response DTOs
│   │   └── services.rs             # Product service & use cases
│   ├── infrastructure/             # 🏗️ Infrastructure Layer
│   │   ├── mod.rs                  # Infrastructure module exports
│   │   ├── database.rs             # Database connection setup
│   │   └── repositories.rs         # SQLite repository implementation
│   └── presentation/               # 🌐 Presentation Layer
│       ├── mod.rs                  # Presentation module exports
│       ├── handlers.rs             # HTTP request handlers
│       ├── routes.rs               # Route configuration
│       └── templates/              # 📄 Template modules
│           ├── mod.rs              # Template module exports
│           ├── product_templates.rs # Product-specific templates
│           └── page_templates.rs   # Page layout templates
├── migrations/
│   └── 001_create_products.sql     # Database schema migration
├── static/
│   └── css/                        # Static assets (future use)
└── README.md                       # This documentation
```

## 🏛️ Layer Responsibilities

### 🎯 Domain Layer (`src/domain/`)
- **Purpose**: Core business logic and rules
- **Contains**: Entities, Value Objects, Domain Services, Repository Interfaces
- **Dependencies**: None (pure business logic)
- **Key Files**:
  - `entities.rs`: Product entity, value objects (ProductId, ProductName, Money, StockQuantity)
  - `repositories.rs`: ProductRepository trait, domain errors

### 🔧 Application Layer (`src/application/`)
- **Purpose**: Orchestrate business use cases
- **Contains**: Services, DTOs, Application-specific logic
- **Dependencies**: Domain layer only
- **Key Files**:
  - `services.rs`: ProductService with business use cases
  - `dtos.rs`: Request/Response DTOs, API contracts

### 🏗️ Infrastructure Layer (`src/infrastructure/`)
- **Purpose**: External concerns (database, external APIs)
- **Contains**: Repository implementations, database setup
- **Dependencies**: Domain and Application layers
- **Key Files**:
  - `repositories.rs`: SqliteProductRepository implementing domain interface
  - `database.rs`: Database connection and configuration

### 🌐 Presentation Layer (`src/presentation/`)
- **Purpose**: User interface and HTTP concerns
- **Contains**: Controllers, Route handlers, Templates
- **Dependencies**: Application layer services
- **Key Files**:
  - `handlers.rs`: HTTP request/response handling
  - `routes.rs`: Route configuration and middleware
  - `templates/`: Separated template generation modules

## 🎨 Template Architecture

Templates are now properly separated from handlers for better maintainability:

### Product Templates (`templates/product_templates.rs`)
- `products_page()`: Full product catalog page
- `product_card()`: Individual product card component
- `product_list_partial()`: HTMX partial updates

### Page Templates (`templates/page_templates.rs`)
- `base_layout()`: Common page structure
- `navigation()`: Site navigation component
- `product_detail_page()`: Individual product pages
- `error_page()`: Error handling pages

## 🔄 Dependency Flow

```
main.rs
  ↓ (creates)
Infrastructure Layer (SqliteProductRepository)
  ↓ (implements)
Domain Layer (ProductRepository trait)
  ↓ (injected into)
Application Layer (ProductService)
  ↓ (used by)
Presentation Layer (Handlers)
  ↓ (generates)
Templates (HTML responses)
```

## 🚀 Quick Start

### Prerequisites
- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Git**: For cloning the repository

### Installation & Running

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ecommerce-app
   ```

2. **Ensure database file exists** (auto-created if missing):
   ```bash
   # The products.db file is already present in the project root
   ls -la products.db
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run the application**:
   ```bash
   cargo run
   # Or run the binary directly:
   ./target/debug/ecommerce-crud
   ```

5. **Verify the application is running**:
   ```bash
   # Check if the server is responding
   curl http://localhost:3000/health
   ```

6. **Open your browser**: 
   - **Main App**: http://localhost:3000
   - **API Endpoints**: http://localhost:3000/api/products
   - **Health Check**: http://localhost:3000/health

### Development Commands

```bash
# Build in development mode
cargo build

# Run with debug logging
RUST_LOG=debug cargo run

# Run with info logging
RUST_LOG=info cargo run

# Check code without building
cargo check

# Run tests (when implemented)
cargo test

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Clean build artifacts
cargo clean

# Watch for changes and rebuild (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

## 🛠️ Troubleshooting

### Common Issues

**1. Application fails to start with database error:**
```bash
# Ensure the database file exists and has proper permissions
touch products.db
chmod 644 products.db
```

**2. Port 3000 already in use:**
```bash
# Check what's using port 3000
lsof -i :3000
# Kill the process if needed
kill -9 <PID>
```

**3. Build fails with dependency errors:**
```bash
# Update Rust toolchain
rustup update
# Clean and rebuild
cargo clean && cargo build
```

**4. HTMX interactions not working:**
- Ensure JavaScript is enabled in your browser
- Check browser console for errors
- Verify HTMX CDN is accessible

### Logging and Debugging

```bash
# Enable detailed logging
export RUST_LOG=debug
cargo run

# Enable SQL query logging
export RUST_LOG=sqlx=debug
cargo run

# Check application logs
tail -f application.log  # If logging to file is configured
```

## 💾 Database

The application uses SQLite with automatic setup:

### Database File
- **Location**: `products.db` in the project root
- **Format**: SQLite 3 database file
- **Size**: Lightweight (~8KB with seed data)
- **Backup**: Simply copy the `products.db` file

### Features
- **Auto-migrations**: Database schema created on startup
- **Seed data**: Pre-populated with 5 demo products (Apple ecosystem)
- **Async operations**: All database operations are async
- **Connection pooling**: Efficient database connection management
- **ACID compliance**: Full transaction support

### Database Management

```bash
# View database file info
ls -la products.db
file products.db

# Access database directly (requires sqlite3)
sqlite3 products.db
> .tables
> SELECT * FROM products;
> .quit

# Backup database
cp products.db products_backup_$(date +%Y%m%d).db

# Reset database (deletes all data)
rm products.db
cargo run  # Will recreate with seed data
```

### Database Schema
```sql
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    price REAL NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### Seed Data
The application starts with these products:
- MacBook Pro 16" ($2,499.99)
- iPhone 15 Pro ($999.99)
- AirPods Pro ($249.99)
- iPad Air ($599.99)
- Apple Watch Ultra ($799.99)

## 🎨 Frontend Features

### HTMX Interactions
- **Add Product**: Form submission without page reload
- **Real-time Search**: Filter products as you type (300ms delay)
- **Delete Products**: Remove with confirmation dialog
- **Responsive Updates**: Seamless DOM updates via HTMX

### UI/UX Design
- **Mobile-first**: Responsive grid layout
- **Modern Styling**: TailwindCSS with custom components
- **Interactive Elements**: Hover effects and smooth transitions
- **Stock Indicators**: Color-coded stock levels
- **Clean Typography**: Proper content hierarchy

### Template Features
- **Component-based**: Reusable template functions
- **Separation of Concerns**: Templates separated from business logic
- **HTMX Integration**: Proper HTMX attributes for interactivity
- **SEO Friendly**: Semantic HTML structure

## 🔧 Development Guide

### Adding New Features

1. **Domain First**: Start with domain entities and rules
2. **Repository Pattern**: Define interfaces in domain layer
3. **Implementation**: Implement in infrastructure layer
4. **Services**: Add use cases in application layer
5. **Presentation**: Create handlers and templates

### Error Handling Strategy

The application uses structured error handling across all layers:

```rust
// Domain layer errors
pub enum DomainError {
    InvalidProductName(String),
    InvalidPrice(String),
    // ...
}

// Repository layer errors  
pub enum RepositoryError {
    DatabaseError(sqlx::Error),
    NotFound,
    // ...
}

// Application layer errors
pub enum ApplicationError {
    DomainError(DomainError),
    RepositoryError(RepositoryError),
    // ...
}
```

### Testing Strategy (Future)

```bash
# Unit tests for domain logic
cargo test domain

# Integration tests for repository
cargo test infrastructure  

# End-to-end tests for API
cargo test --test integration
```

## 📚 Example Usage

### REST API Examples

**Create Product:**
```bash
curl -X POST http://localhost:3000/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Air M3",
    "description": "Lightweight laptop with M3 chip",
    "price": 1299.99,
    "stock": 20
  }'

# Expected Response:
# {
#   "id": 6,
#   "name": "MacBook Air M3",
#   "description": "Lightweight laptop with M3 chip",
#   "price": 1299.99,
#   "stock": 20,
#   "created_at": "2025-09-23T10:30:00Z",
#   "updated_at": "2025-09-23T10:30:00Z"
# }
```

**Search Products:**
```bash
# Search by name
curl "http://localhost:3000/api/products?search=iPhone"

# List all products
curl "http://localhost:3000/api/products"

# Expected Response:
# [
#   {
#     "id": 2,
#     "name": "iPhone 15 Pro",
#     "description": "Latest iPhone with A17 Pro chip",
#     "price": 999.99,
#     "stock": 15,
#     "created_at": "2025-09-23T08:00:00Z",
#     "updated_at": "2025-09-23T08:00:00Z"
#   }
# ]
```

**Get Single Product:**
```bash
curl "http://localhost:3000/api/products/1"

# Expected Response:
# {
#   "id": 1,
#   "name": "MacBook Pro 16\"",
#   "description": "Powerful laptop for professionals",
#   "price": 2499.99,
#   "stock": 10,
#   "created_at": "2025-09-23T08:00:00Z",
#   "updated_at": "2025-09-23T08:00:00Z"
# }
```

**Update Product:**
```bash
curl -X PUT http://localhost:3000/api/products/1 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Pro 16\" Updated",
    "description": "Updated description",
    "price": 2599.99,
    "stock": 8
  }'

# Expected Response: Updated product JSON
```

**Delete Product:**
```bash
curl -X DELETE http://localhost:3000/api/products/1

# Expected Response: 204 No Content
```

**Health Check:**
```bash
curl http://localhost:3000/health

# Expected Response:
# {
#   "status": "ok",
#   "timestamp": "2025-09-23T10:30:00Z"
# }
```

### Frontend Usage

**Main Product Catalog:**
- Visit: http://localhost:3000
- Features: Search, view products, responsive design

**Add New Product:**
- Click "Add Product" button
- Fill form and submit via HTMX
- New product appears without page reload

**Real-time Search:**
- Type in search box
- Results filter automatically (300ms delay)
- Uses HTMX for seamless updates

## 🏗️ Architecture Benefits

### Clean Architecture Advantages
- **Testability**: Each layer can be tested independently
- **Maintainability**: Clear separation of concerns
- **Flexibility**: Easy to change infrastructure without affecting business logic
- **Scalability**: Well-organized code structure for team development

### Domain-Driven Design Benefits
- **Business Focus**: Code reflects business terminology
- **Type Safety**: Value objects prevent invalid states
- **Rich Model**: Business logic lives in domain entities
- **Ubiquitous Language**: Consistent terminology across team

## 🔮 Future Enhancements

### Planned Features
- [ ] User authentication and authorization
- [ ] Product categories and tagging
- [ ] Inventory management with alerts
- [ ] Order management system
- [ ] Payment integration
- [ ] File upload for product images
- [ ] Advanced search and filtering
- [ ] Caching layer (Redis)
- [ ] Metrics and monitoring
- [ ] Comprehensive test suite

### Architecture Improvements
- [ ] Domain events for audit logging
- [ ] CQRS pattern for read/write separation
- [ ] Event sourcing for complete audit trail
- [ ] API versioning strategy
- [ ] Rate limiting and security middleware
- [ ] OpenAPI/Swagger documentation
- [ ] Docker containerization
- [ ] CI/CD pipeline setup

## 📊 Project Status

**Current Version**: 1.0.0-alpha  
**Development Stage**: Active Development  
**Last Updated**: September 2025

### Recent Changes
- ✅ Initial Clean Architecture implementation
- ✅ SQLite database integration with seed data
- ✅ HTMX frontend with responsive design
- ✅ Complete REST API with error handling
- ✅ Template-based HTML rendering
- ✅ Real-time search functionality

### Known Issues
- [ ] No input validation on frontend forms
- [ ] Limited error messages in UI
- [ ] No pagination for large product lists
- [ ] Missing comprehensive test coverage

## 🧪 Testing

### Manual Testing Checklist

**API Endpoints:**
- [ ] GET /api/products - List products
- [ ] GET /api/products/{id} - Get single product
- [ ] POST /api/products - Create product
- [ ] PUT /api/products/{id} - Update product
- [ ] DELETE /api/products/{id} - Delete product
- [ ] GET /health - Health check

**Frontend Features:**
- [ ] Product listing displays correctly
- [ ] Search functionality works
- [ ] Add product form submits
- [ ] Product cards display properly
- [ ] Responsive design on mobile
- [ ] HTMX interactions work without page reload

**Error Scenarios:**
- [ ] Invalid product ID returns 404
- [ ] Malformed JSON returns 400
- [ ] Empty product name validation
- [ ] Negative price/stock validation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Follow clean architecture principles
4. Write tests for new functionality
5. Update documentation
6. Submit a pull request

### Code Style Guidelines
- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Document public APIs
- Keep layers properly separated

## 📄 License

This project is open source and available under the MIT License.

---

**Built with ❤️ using Clean Architecture principles in Rust**

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is open source and available under the MIT License.