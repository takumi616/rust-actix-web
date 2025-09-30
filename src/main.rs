use actix_web::{App, HttpServer, Responder, web};


async fn index() -> impl Responder {
    "Hello world!"
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app")
        .route("/index", web::get().to(index)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_str = env::var("CONTAINER_APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a valid number");

    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}