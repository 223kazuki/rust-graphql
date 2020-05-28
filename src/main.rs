use actix_web::{middleware, App, HttpServer, web, Responder};
use dotenv::{dotenv, from_filename};

/// Hello World!を出力するだけのハンドラ関数
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        // debugのときは.env.localファイルを読み込み
        from_filename(".env.local").ok();
    } else {
        dotenv().ok();
    }
    env_logger::init();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
    });

    server = server.bind("127.0.0.1:3000").unwrap();
    server.run().await
}
