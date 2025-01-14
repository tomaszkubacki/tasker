use std::time::Duration;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use log::info;
use simple_logger::SimpleLogger;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hi - i am the tasker app")
}

#[post("/task")]
async fn echo(req_body: String) -> impl Responder {
    let tsk = tokio::task::spawn(async { new_task().await });

    info!("started task {}", tsk.id());
    //let res = tsk.await.unwrap();
    //info!("task finished {}", res);
    HttpResponse::Ok().body(req_body)
}

async fn new_task() {
    for i in 0..500 {
        if i % 25 == 0 {
            info!("Still working {}", i);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().env().init().unwrap();

    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
