use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    // получаем список всех авторов ввиде text   
    all_authors().await?;

    // получаем список всех авторов ввиде text не асинхронно
    // tokio::task::spawn_blocking(move || blocking_get().unwrap());
    
    // получаем список всех авторов ввиде json
    all_authors_json().await?;

    // добавить нового атора post-запросом
    add_author().await?;

    Ok(())
}

// get-запрос на получение списка всех авторов без асинхронности
fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("http://127.0.0.1:3000/api/v1/authors")?;

    let body = res.text()?;
    println!("Список всех авторов: ");
    println!("body = {:?}", body);
    println!("--------------------------------");

    Ok(())
}

// get-запрос на получение списка всех авторов ввиде текста
async fn all_authors() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("http://127.0.0.1:3000/api/v1/authors").await?.text().await?;
    println!("body = {:?}", body);    
    println!("--------------------------------");

    Ok(())
}

// get-запрос на получение списка всех авторов в формате Json
async fn all_authors_json() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("http://127.0.0.1:3000/api/v1/authors").await?;

    let body: Vec<Author> = res.json::<Vec<Author>>().await?;

    for author in body {
        println!("name = {:?}, country = {:?}", author.name, author.country);
    }
    
    println!("--------------------------------");
    
    Ok(())
}

// post-запрос на добавление автора
async fn add_author() -> Result<(), Box<dyn std::error::Error>> {

    // let mut author = HashMap::new();
    // author.insert("author_name", "name_author");
    let author = Author {
        authors_id: None,
        name: "SSSSSSSSSSSSSSSSSS".to_string(),
        country: 1,
    };
    

    let client = reqwest::Client::new();
    // let res = client.post("http://localhost:3000/api/v1/author").header(reqwest::header::CONTENT_TYPE, "application/json").json(&author).send().await?;
    let res = client.post("http://localhost:3000/api/v1/author").json(&author).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!(" Автор принят")
        }
        status => {
            println!("Статус код = {}", status)
        }
    }

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Author {
    authors_id: Option<i32>,
    name: String,
    country: i32,
}