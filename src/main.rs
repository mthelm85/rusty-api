use std::env;

use actix_cors::Cors;
use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };
use anyhow::Result;
use mongodb::{ Client, Database, bson::doc, options::ClientOptions };

mod types;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub client: Client
}

#[get("/county/{fips}")]
async fn search(data: web::Data<AppState>, info: web::Path<types::Info>) -> impl Responder {
    let collection = data.db.collection::<types::County>("counties");
    let filter = doc! { "fips": info.fips.clone() };
    // let options = FindOneOptions::builder().projection(doc! { "place_name": true }).build();
    match collection.find_one(filter, None).await {
        Ok(county) => {
            match county {
                Some(c) => HttpResponse::Ok().json(types::Response { data: Some(c) }),
                None => HttpResponse::Ok().json(types::Response { data: None })
            }
        },
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> Result<()> { 
    let mongouri = env::var("MONGODB_URI").unwrap_or("mongodb://127.0.0.1:27017/".to_string());
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let client_options = ClientOptions::parse(mongouri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("labor-data");
    let state = AppState { db, client };
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .data(state.clone())
            .service(search)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await?;
    Ok(())
}

