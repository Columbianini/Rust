use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    println!("Serving on http://localhost:3000...");
    server.bind("127.0.0.1:3000")?
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}