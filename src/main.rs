use actix_web::{web, App, Error, HttpResponse, HttpServer};
use chrono::Local;
async fn index() -> Result<HttpResponse, Error> {
    // 現在時刻を表示
    let now = Local::now();
    println!("GET at: {}", &now);
    // localhost:3000/にリクエストがあったらHello Worldの文字列を返す
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello World"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("start with Rust!");
    // サーバーを立ち上げる
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        // 3000番ポートをバインド
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        // 実行
        .run()
        // 非同期関数には.awaitをつけられる
        .await
}
