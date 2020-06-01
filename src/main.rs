use crate::graphql::schema::{create_schema, Context, Photo, Schema};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenv::{dotenv, from_filename};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;


pub mod graphql;

impl Photo {
    fn new(id: String, name: String, description: String) -> Photo {
        Photo {
            id,
            name,
            description,
        }
    }
}

/// actixからGraphQLにアクセスするためのハンドラメソッド
pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(
            &st,
            &Context {
                photos: vec![
                    Photo::new(
                        "1".to_string(),
                        "test1".to_string(),
                        "test1 photo".to_string(),
                    ),
                    Photo::new(
                        "2".to_string(),
                        "test2".to_string(),
                        "test2 photo".to_string(),
                    ),
                    Photo::new(
                        "3".to_string(),
                        "test3".to_string(),
                        "test3 photo".to_string(),
                    ),
                    Photo::new(
                        "4".to_string(),
                        "test4".to_string(),
                        "test4 photo".to_string(),
                    ),
                    Photo::new(
                        "5".to_string(),
                        "test5".to_string(),
                        "test5 photo".to_string(),
                    ),
                ],
            },
        );
        // serde_jsonで実際はエラーになる可能性があるのでOkのturbofishに指定
        // 実際これでエラーになる場合はawait?によって早期returnされてErrがreturnされる
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

/// actixからGraphiQLにアクセスするためのハンドラメソッド
pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn hello_world() -> impl Responder {
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

    // 追加
    // juniperのschemaを共有してGraphQL用のハンドラに引き渡す
    let schema = std::sync::Arc::new(create_schema());

    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
            .route("/graphiql", web::get().to(graphiql)) // 追加
            .route("/graphql", web::post().to(graphql)) // 追加
    });

    server = server.bind("127.0.0.1:3000").unwrap();
    server.run().await
}
