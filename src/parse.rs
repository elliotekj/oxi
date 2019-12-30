use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parse {
    url: String,
    format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    title: String,
    content: String,
    author: Option<String>,
    date_published: Option<String>,
    lead_image_url: Option<String>,
    dek: Option<String>,
    next_page_url: Option<String>,
    url: String,
    domain: String,
    excerpt: String,
    word_count: u64,
    direction: String,
    total_pages: u64,
    rendered_pages: u64,
}

pub async fn parse(params: web::Json<Parse>) -> HttpResponse {
    info!("Parsing \"{}\"", params.url);

    let url = params.url.clone();
    let mut format = String::from("html");

    if let Some(format_param) = &params.format {
        if (format_param == "markdown") | (format_param == "md") {
            format = String::from("markdown");
        } else if format_param == "text" {
            format = String::from("text");
        }
    }

    let parser_result = web::block(move || run_mercury_parser(url, format)).await;

    match parser_result {
        Ok(article) => HttpResponse::Ok().json(article),

        Err(_) => {
            let error_value = format!("Failed to parse \"{}\"", params.url);
            error!("{}", error_value);

            let error_json = json!({
                "error": "Failed to parse URL",
                "url": params.url
            });

            HttpResponse::UnprocessableEntity().json(error_json)
        }
    }
}

fn run_mercury_parser(url: String, format: String) -> Result<Article, ()> {
    let output = Command::new("mercury-parser")
        .arg(format!("--format={}", format))
        .arg(url)
        .output()
        .ok();

    if let Some(output) = output {
        let json_str = std::str::from_utf8(&output.stdout).unwrap();

        match serde_json::from_str(json_str) {
            Ok(article) => return Ok(article),
            Err(e) => {
                error!("{:?}", e);
                return Err(());
            }
        }
    }

    Err(())
}
