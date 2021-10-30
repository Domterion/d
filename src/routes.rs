use std::env;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use reqwest::StatusCode;

use crate::models;

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

pub async fn send_message(
    _req: HttpRequest,
    data: web::Data<models::AppState>,
    message: web::Json<models::Message>,
) -> Result<HttpResponse, Error> {
    if message.username.len() > 32 || message.username.is_empty() {
        let response = models::Response {
            success: false,
            message: "Username is greater than 32 characters or 0.".to_string(),
        };

        return Ok(HttpResponse::PayloadTooLarge().json(response));
    }

    if message.content.len() > 2000 || message.content.is_empty() {
        let response = models::Response {
            success: false,
            message: "Message content greater than 2000 characters or 0.".to_string(),
        };

        return Ok(HttpResponse::PayloadTooLarge().json(response));
    }

    let mut sent = data.sent.lock().unwrap();

    let content = &message.0;

    let payload = models::Payload {
        username: env::var("USERNAME").unwrap(),
        content: "`📧`".to_string(),
        embeds: vec![models::Embed {
            author: models::EmbedAuthor {
                name: content.username.to_owned(),
            },
            description: content.content.to_owned(),
            color: env::var("EMBED_COLOR").unwrap(),
            footer: models::EmbedFooter {
                text: format!("Messages sent this session: {}", *sent + 1),
            },
        }],
    };

    let _ = data
        .client
        .post(&env::var("WEBHOOK_URL").unwrap())
        .json(&payload)
        .send()
        .await
        .unwrap();

    let response = models::Response {
        success: true,
        message: "Message sent!".to_string(),
    };

    *sent += 1;

    Ok(HttpResponse::Ok().json(response))
}
