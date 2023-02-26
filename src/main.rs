
//use axum::Extension;
//use axum::extract::Path;
use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
    response::IntoResponse,
    Json,    
    Extension,
    extract::{Path, Query}, 
};

use sqlx::postgres::{PgPoolOptions, PgRow, PgQueryResult};
use sqlx::PgPool;



use serde_json::{Value, json};

use serde::{Deserialize, Serialize};

use std::net::SocketAddr;


#[tokio::main]
async fn main(){         

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect("postgres://postgres:2670467@localhost/my_db")
        .await
        .unwrap();

    let app = Router::new()
    // тест. Hello? world
    .route("/", get(hello))    
    // GET запрос на получение всех авторов
    .route("/api/v1/authors", get(get_authors))
    // GET запрос на поиск автора по имени
    .route("/api/v1/author/search", get(search_author))
    // GET запрос на получение книг по author_id автора
    .route("/api/v1/author/:author_id", get(get_author_name))
    // PUT запрос на изменение имени атора
    .route("/api/v1/author/:author_id", put(update_author_name))
    // DELETE запрос на удаление автора
    .route("/api/v1/author/:author_id", delete(delete_author))
    // POST запрос на создание автора
    .route("/api/v1/author", post(add_author))    
    .layer(Extension(pool));  
   
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));   
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); 

}

async fn hello() -> &'static str{    
    "Hello, world"
}

// POST запрос: добывить нового автора
async fn  add_author(Extension(pool): Extension<PgPool>, Json(author_name): Json<NewAuthor> ) -> Result<(StatusCode, Json<NewAuthor>), CustomError>{    
 
    if author_name.author_name.is_empty() {
        return Err(CustomError::BadRequest)
    }

    let sql = "INSERT INTO authors(name) VALUES ($1)".to_string();
    
    let _ = sqlx::query(&sql)
        .bind(&author_name.author_name)
        .execute(&pool)
        .await
        .map_err(|_| {
            CustomError::AuthorIsRepeats
        })?;    
            
    Ok((StatusCode::CREATED, Json(author_name)))                
} 


// GET запрос: получить список всех авторов
async fn get_authors(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
       
   let sql = "SELECT * FROM authors".to_string();

    let list_authors = sqlx::query_as::<_, Author>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(list_authors))
}

// PUT запрос: изменение имени автора по id
async fn update_author_name(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>, Json(update_author): Json<NewAuthor>) -> Result<(StatusCode, Json<NewAuthor>), CustomError> {
       
    let _find: Author = sqlx::query_as("SELECT * FROM authors WHERE authors_id=$1")
       .bind(author_id)
       .fetch_one(&pool)
       .await
       .map_err(|_| {
            CustomError::AuthorNotFound
       })?;
        
    let sql = "UPDATE authors SET name=$1 WHERE authors_id=$2".to_string();
            
    let _ = sqlx::query(&sql)
        .bind(&update_author.author_name)
        .bind(author_id)
        .execute(&pool)         
        .await
        .map_err(|_| {
             CustomError::InternalServerError
        }); 
        
    Ok((StatusCode::OK, Json(update_author)))        
    
 }

// DELETE запрос: удаление автора по id
async fn delete_author(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<(StatusCode, Json<Value>), CustomError> {
       
    let _find: Author = sqlx::query_as("SELECT * FROM authors WHERE authors_id=$1")
        .bind(author_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::AuthorNotFound            
        })?;

    let sql = "DELETE FROM authors WHERE authors_id=$1".to_string();
 
    sqlx::query(&sql)        
         .bind(author_id)
         .execute(&pool)         
         .await
         .map_err(|_| {
            CustomError::AuthorNotFound
         })?; 
 
     Ok((StatusCode::OK, Json(json!({"msg": "Author Deleted"}))))
 }

// GET запрос: поиск автора по имени
async fn search_author(  Extension(pool): Extension<PgPool>, Query(query): Query<NewAuthor>) -> Result<Json<Author>, CustomError> {
    
    let sql = "SELECT * FROM authors WHERE name=$1".to_string();   
 
    let author = sqlx::query_as::<_, Author>(&sql)        
         .bind(query.author_name)
         .fetch_one(&pool)         
         .await
         .map_err(|_| {
            CustomError::AuthorNotFound
         })?; 
 
     Ok(Json(author))
 }

// GET запрос: получение атора по id
async fn get_author_name(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<Json<Author>, CustomError> {
  
    let sql = "SELECT * FROM authors WHERE authors_id=$1".to_string();

    let author = sqlx::query_as::<_, Author>(&sql)
        .bind(author_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::AuthorNotFound
        })?;

    Ok(Json(author))
}



#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct NewAuthor {
    author_name: String,
}


#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct Author {
    authors_id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct Book {
    books_id: i32,
    fk_authors_id: i32,
    title: String,
}

// перечисление для обработки ошибок
enum CustomError {
    BadRequest,
    AuthorNotFound,
    InternalServerError,
    AuthorIsRepeats,
}

// реализуем трейт IntoResponse для enum CustomError
impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::AuthorNotFound => (StatusCode::NOT_FOUND, "Author Not Found"),
            Self::AuthorIsRepeats => (StatusCode::NOT_IMPLEMENTED, "The author repeats"),            
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}


