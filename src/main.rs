
//use axum::Extension;
//use axum::extract::Path;
use axum::{
    routing::{get, post, put, delete},
    http::{StatusCode, header, HeaderMap},
    Router,
    response::IntoResponse,
    Json,    
    Extension,
    extract::{Path, Query}, 
};

use sqlx::postgres::{PgPoolOptions, PgRow, PgQueryResult};
use sqlx::{PgPool, query};

use std::net::SocketAddrV4;

use serde_json::{Value, json};

use serde::{Deserialize, Serialize};

use std::net::{Ipv4Addr, SocketAddr};

use dotenv::dotenv;

mod lib;

#[tokio::main]
async fn main(){       
    
    dotenv().ok();    

    let ip = std::env::var("IP").expect("Переменная IP не найдена"); 
    let socet: SocketAddrV4 = ip.parse().expect("Не смог распарсить IP и socet");  

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
    // POST запрос на создание автора
    .route("/api/v1/author", post(add_author))    
    // DELETE запрос на удаление автора
    .route("/api/v1/author/:author_id", delete(delete_author))    
    .layer(Extension(pool));      

    let addr = SocketAddr::from(socet);   
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); 

}

async fn hello() -> &'static str{    
    "Hello, world"
}

// POST запрос: добывить нового автора
async fn  add_author(Extension(pool): Extension<PgPool>, Json(author_name): Json<NewAuthor> ) -> Result<(StatusCode,HeaderMap, Json<NewAuthor>), CustomError>{       

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
            
    // отправляю дополнительно заголовок, для того чтобы браузер не блокировал входящий json
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

    Ok((StatusCode::CREATED, headers, Json(author_name)))                
} 


// GET запрос: получить список всех авторов
async fn get_authors(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
       
    let sql = "SELECT * FROM authors".to_string();

    let list_authors = sqlx::query_as::<_, Author>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();
       
    (StatusCode::OK, [(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], Json(list_authors))  
}

// PUT запрос: изменение имени автора по id //
async fn update_author_name(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>, Json(update_author): Json<NewAuthor>) -> Result<(StatusCode, HeaderMap), CustomError> {
  
   // открываем транзакцию
   let mut transaction = pool.begin().await.unwrap();   

    let _find: Author = sqlx::query_as("SELECT * FROM authors WHERE authors_id=$1")
       .bind(author_id)
       .fetch_one(&mut transaction)
       .await
       .map_err(|_| {
            CustomError::AuthorNotFound
       })?;
        
    let sql = "UPDATE authors SET name=$1 WHERE authors_id=$2".to_string();   

    let _ = sqlx::query(&sql)
        .bind(&update_author.author_name)
        .bind(author_id)
        .execute(&mut transaction)         
        .await
        .map_err(|_| {
             CustomError::InternalServerError
        })?; 

   // закрываем транзакцию    
   transaction.commit().await.unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());      
      
    Ok((StatusCode::OK, headers))
 }

// DELETE запрос: удаление автора по id
async fn delete_author(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<(StatusCode, HeaderMap, Json<Value>), CustomError> {

    // открываем транзакцию
    let mut transaction = pool.begin().await.unwrap();    
    
    let _find: Author = sqlx::query_as("SELECT * FROM authors WHERE authors_id=$1")
        .bind(author_id)
        .fetch_one(&mut transaction)
        .await
        .map_err(|_| {
            CustomError::AuthorNotFound            
        })?;

    let sql = "DELETE FROM authors WHERE authors_id=$1".to_string();
 
    sqlx::query(&sql)        
         .bind(author_id)
         .execute(&mut transaction)         
         .await
         .map_err(|_| {
            CustomError::InternalServerError
         })?; 
 
    // закрываем транзакцию    
    transaction.commit().await.unwrap();

    // отправляю дополнительно заголовок, для того чтобы браузер не блокировал входящий json
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

    Ok((StatusCode::OK, headers, Json(json!({"msg": "Author Deleted"}))))
 }

// GET запрос: поиск автора по имени
async fn search_author(  Extension(pool): Extension<PgPool>, Query(query): Query<NewAuthor>) ->  Result<(StatusCode, HeaderMap, Json<Vec<Author>>), CustomError> { 
    
    // sql-запрос
    let mut sql = "SELECT * FROM authors WHERE name LIKE ".to_string(); 
    // пропускаем через функцию escape_internal параметр запроса и URL, чтобы обезопаситься от SQLi
    let query_param = lib::escape_internal(&query.author_name, false); 
    // добавляем получившийся параметр запроса к SQL-запросу
    sql.push_str(&query_param);      

    let author: Vec<Author> = sqlx::query_as::<_, Author>(&sql)          
        .fetch_all(&pool)
        .await         
        .map_err(|_| {
           CustomError::InternalServerError
        })?;     
    
    // если в БД нет совпадений, то вернём ошибку об отсутствии таких авторов
    if author.is_empty() {
        return Err(CustomError::AuthorNotFound)
    }   

    // отправляю дополнительно заголовок, для того чтобы браузер не блокировал входящий json
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    
    Ok((StatusCode::OK,headers, Json(author)))
 }

// GET запрос: получение атора по id
async fn get_author_name(Path(author_id): Path<i32>, Extension(pool): Extension<PgPool>) -> Result<(HeaderMap, Json<Author>), CustomError> {
  
    let sql = "SELECT * FROM authors WHERE authors_id=$1".to_string();

    let author = sqlx::query_as::<_, Author>(&sql)
        .bind(author_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::AuthorNotFound
        })?;
    
    // отправляю дополнительно заголовок, для того чтобы браузер не блокировал входящий json
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    
    Ok((headers, Json(author)))
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

#[derive(sqlx::FromRow, Deserialize, Serialize)]
struct NewBook {
    book_name: String,
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
    
    // отправляю дополнительно заголовок, для того чтобы браузер не блокировал входящий json
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,                
                "Internal Server Error",
            ),
            Self::BadRequest=> (
                StatusCode::BAD_REQUEST,                  
                "Bad Request"
            ),
            Self::AuthorNotFound => (
                StatusCode::NOT_FOUND,                 
                "Author Not Found"
            ),
            Self::AuthorIsRepeats => (
                StatusCode::NOT_IMPLEMENTED, 
                "The author repeats"
            ),            
        };
        (status, headers, Json(json!({"error": error_message}))).into_response()
    }
}


