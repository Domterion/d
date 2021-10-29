use std::{env, sync::Mutex};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;

mod models;
mod routes;

/*

curl -X POST http://localhost:6970/send \
    -H 'Content-Type: application/json' \
    -d '{"username": "Danii", "content": "owo"}'

   */
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port = env::var("PORT")?.parse::<u16>()?;
    let data = web::Data::new(models::AppState {
        client: Client::new(),
        sent: Mutex::new(0),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("URL").unwrap())
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new()
            .app_data(data.clone())
            .wrap(cors)
            .route("/send", web::post().to(routes::send_message))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}
