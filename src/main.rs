use std::{sync::Mutex, time::Duration};

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use log::info;
use tokio::task::Id;

static TASK_LIST: Mutex<Vec<Id>> = Mutex::new(Vec::new());

fn task_list_str() -> String {
    format!("{:?}", TASK_LIST.lock().unwrap())
}

fn remove_task(id: &Id) {
    TASK_LIST.lock().unwrap().retain(|t| t != id);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hi - i am the tasker app")
}

#[get("/task")]
async fn task_list() -> impl Responder {
    HttpResponse::Ok().body(task_list_str())
}

#[post("/task")]
async fn task_add(req_body: String) -> impl Responder {
    let tsk = tokio::task::spawn(async { new_task().await });

    TASK_LIST.lock().unwrap().push(tsk.id());
    //let res = tsk.await.unwrap();
    //info!("task finished {}", res);
    HttpResponse::Ok().body(req_body)
}

async fn new_task() {
    let task_id = tokio::task::try_id().unwrap();
    info!("started task {:?}", task_id);
    for i in 0..500 {
        if i % 25 == 0 {
            info!("task {:?} {}", task_id, i);
            tokio::time::sleep(Duration::from_millis(1500)).await;
        }
    }
    remove_task(&task_id);
    info!("finished task id {:?}", task_id);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(task_list)
            .service(task_add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
