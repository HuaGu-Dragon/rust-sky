# rust-sky

A high-performance restaurant management system built in Rust, inspired by the "Take-out" (Sky Take-out) domain. This system provides comprehensive APIs for both administrative management and customer-facing operations.

## 🚀 Features

### Admin Management
- **Employee Management**: Staff account CRUD operations and status management
- **Menu Management**: Categories, dishes, and setmeals with full lifecycle control
- **Shop Control**: Global shop status management (Open/Closed)
- **File Upload**: Image handling for menu items with date-based storage

### Customer Operations
- **WeChat Authentication**: User login and profile management
- **Menu Browsing**: Category-based dish exploration
- **Shopping Cart**: Real-time cart management
- **Address Book**: Delivery address management

## 🏗️ Architecture

### Workspace Structure
The project is organized as a Cargo workspace with two main crates:

| Crate | Purpose | Key Technologies |
|-------|---------|------------------|
| `rust-sky` | Core application logic, HTTP server | `axum`, `tokio`, `tower-http` |
| `sky-pojo` | Data models, DTOs, VOs | `sea-orm`, `serde`, `rust_decimal` |

### Technology Stack
- **Web Framework**: `axum` for routing and middleware
- **Database**: PostgreSQL with `SeaORM` ORM
- **Caching**: Redis for cache-aside pattern optimization
- **Async Runtime**: `tokio` for asynchronous execution
- **Memory Management**: `mimalloc` for improved performance

## 📁 Project Structure

```
rust-sky/
├── src/                     # Main application
│   ├── controller/          # HTTP handlers
│   │   ├── admin/           # Admin endpoints
│   │   └── user/            # User endpoints
│   ├── server/              # Business logic layer
│   ├── config/              # Configuration management
│   └── app.rs               # Application state
├── sky-pojo/                # Data models
│   ├── src/
│   │   ├── entities/        # SeaORM entities
│   │   ├── dto/             # Data transfer objects
│   │   └── vo/              # View objects
│   └── Cargo.toml
├── Cargo.toml               # Workspace configuration
└── readme.md
```

## 🛠️ Getting Started

### Prerequisites
- Rust 1.85+
- PostgreSQL 14+
- Redis 6+

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/HuaGu-Dragon/rust-sky.git
   cd rust-sky
   ```

2. **Configure environment**
   - Create or update an `application.toml` configuration file for the service
   - Configure database and Redis connection settings in `application.toml`

3. **Run database migrations**
   ```bash
   psql -d your_database -f sky.sql
   ```

4. **Start the application**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080`

## 📚 API Documentation

### Admin API
All admin endpoints are under `/admin` prefix and require JWT authentication:

- **Employee Management**: `/admin/employee`
- **Category Management**: `/admin/category` 
- **Dish Management**: `/admin/dish`
- **Setmeal Management**: `/admin/setmeal`
- **Shop Status**: `/admin/shop`
- **File Upload**: `/admin/common/upload`

### Authentication Flow
The system uses JWT tokens with `AdminId` claims for securing admin endpoints.

### Example API Usage
See `test.http` for comprehensive API examples.

## 🔧 Key Features

### Caching Strategy
The system implements a cache-aside pattern for dish listings, checking Redis cache before querying PostgreSQL.

### File Upload
Images are stored in a date-based hierarchy (`./upload/YYYY/MM/DD/`) with SHA-1 hashed filenames to prevent collisions.

### Transaction Management
Setmeal operations handle complex transactions across multiple tables (setmeal and setmeal_dish association tables).

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## Notes

This README is generated based on the current codebase structure and wiki documentation. For the most up-to-date information, please refer to the wiki pages and source code directly.

Wiki pages you might want to explore:
- [Project Overview (HuaGu-Dragon/rust-sky)](https://deepwiki.com/HuaGu-Dragon/rust-sky)

### Citations

**File:** test.http (L61-139)
```text
POST http://localhost:8080/admin/category
Content-Type: application/json
Authorization: Bearer {{token}}

{
    "name": "bun",
    "type": 1,
    "sort": "10"
}

### Get Category Page (Requires Auth)
GET http://localhost:8080/admin/category/page?page=1&pageSize=10
Authorization: Bearer {{token}}

### Update Category (Requires Auth)
PUT http://localhost:8080/admin/category
Content-Type: application/json
Authorization: Bearer {{token}}

{
    "id": 27,
    "name": "bun-updated",
    "sort": "10"
}

Authorization: Bearer {{token}}

### Delete Category by ID (Requires Auth)
DELETE http://localhost:8080/admin/category?id=13
Authorization: Bearer {{token}}

### Dish List

GET http://localhost:8080/admin/category/list?type=1
Authorization: Bearer {{token}}

### Query Dish Page (Requires Auth)
GET http://localhost:8080/admin/dish/page?page=1&pageSize=10&status=1
Authorization: Bearer {{token}}

### Delete Dish by ID (Requires Auth)
DELETE http://localhost:8080/admin/dish?ids=46
Authorization: Bearer {{token}}

### Query Dish by ID (Requires Auth)
GET http://localhost:8080/admin/dish/53
Authorization: Bearer {{token}}

### Update Meal (Requires Auth)
PUT http://localhost:8080/admin/setmeal
Content-Type: application/json
Authorization: Bearer {{token}}

{
    "categoryId": 15,
    "categoryName": "商务套餐",
    "description": "test update",
    "id": 33,
    "idType": 15,
    "name": "test",
    "price": "100",
    "setmealDishes": [
        {
            "dishId": 50,
            "name": "馒头",
            "copies": 3,
            "price": 1
        }
    ],
    "status": 0
}

### Query Shop Status (Requires Auth)
GET http://localhost:8080/admin/shop/status
Authorization: Bearer {{token}}

### Update Shop Status (Requires Auth)
PUT http://localhost:8080/admin/shop/0
Authorization: Bearer {{token}}
```

**File:** src/controller/admin/dish.rs (L81-103)
```rust
async fn list(
    AdminId(_id): AdminId,
    State(AppState { db, mut redis }): State<AppState>,
    Query(DishQueryId { category_id }): Query<DishQueryId>,
) -> ApiReturn<Vec<DishDetailVO>> {
    let key = format!("dish_{category_id}");
    if let Ok(Some(cached)) = redis.get(&key).await
        && let Ok(dishes) = serde_json::from_str::<Vec<DishDetailVO>>(&cached)
    {
        Ok(ApiResponse::success(dishes))
    } else {
        let dishes = server::dish::list(db, category_id).await?;
        redis
            .set(
                key,
                serde_json::to_string(&dishes).map_err(|_| ApiError::Internal)?,
            )
            .await
            .map_err(|_| ApiError::Internal)?;

        Ok(ApiResponse::success(dishes))
    }
}
```

**File:** src/controller/admin/setmeal.rs (L22-28)
```rust
pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save).delete(delete_meal).put(update))
        .route("/{id}", get(get_meal))
        .route("/status/{status}", post(status))
        .route("/page", get(page))
}
```
