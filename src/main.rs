use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };
use mongodb::{ Client, Database, bson::doc, options::ClientOptions };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone)]
struct AppState {
    pub db: Database,
    pub client: Client
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Naic {
    code: u32,
    title: String,
}

#[derive(Deserialize)]
struct Info {
    code: u32
}

#[get("/search/{code}")]
async fn search(data: web::Data<AppState>, info: web::Path<Info>) -> impl Responder {
    let collection = data.db.collection::<Naic>("Descriptions");
    match collection.find_one(doc! { "Code": info.code }, None).await {
        Ok(naic) => {
            match naic {
                Some(code) => HttpResponse::Ok().body(format!("Description: {}", code.title)),
                None => HttpResponse::Ok().body("NAIC not found")
            }
        },
        Err(e) => HttpResponse::Ok().body(format!("Error: {}", e))
    }
    
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let client_options = ClientOptions::parse("mongodb://127.0.0.1:27017/").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("NAICS");
    let state = AppState { db, client };
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(search)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}

