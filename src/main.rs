use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time::Duration,
};

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use tokio::task::{Id, JoinHandle};

static PORT: u16 = 8181;
static TASK_HANDLES: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static ID_GEN: AtomicUsize = AtomicUsize::new(0);

fn get_next_id() -> usize {
    ID_GEN.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug)]
struct Task {
    id: usize,
    handle: JoinHandle<()>,
}

impl Task {
    pub fn new(handle: JoinHandle<()>) -> Task {
        Task {
            id: get_next_id(),
            handle,
        }
    }
}

fn add_task(handle: JoinHandle<()>) -> usize {
    let task = Task::new(handle);
    let id = task.id;
    TASK_HANDLES.lock().unwrap().push(task);
    id
}

fn task_ids() -> Vec<usize> {
    TASK_HANDLES
        .lock()
        .unwrap()
        .iter()
        .map(|t| t.id)
        .collect::<Vec<usize>>()
}

fn remove_task(tokio_id: &Id) {
    TASK_HANDLES
        .lock()
        .unwrap()
        .retain(|t| &t.handle.id() != tokio_id);
}

fn stop_task(id: usize) {
    TASK_HANDLES
        .lock()
        .unwrap()
        .iter()
        .find(|t| t.id == id)
        .inspect(|t| t.handle.abort());
    TASK_HANDLES.lock().unwrap().retain(|t| t.id != id);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hi - i am the tasker app")
}

#[get("/task")]
async fn task_list() -> impl Responder {
    HttpResponse::Ok().json(task_ids())
}

#[put("/task/stop/{id}")]
async fn task_stop(path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();
    info!("stopping task {}", id);
    stop_task(id);
    HttpResponse::Ok()
}

#[post("/task")]
async fn task_add(req_body: String) -> impl Responder {
    let tsk = tokio::task::spawn(async { new_task().await });
    let id = add_task(tsk);
    info!("added task id {}: {}", id, req_body);
    HttpResponse::Ok().json(id)
}

async fn new_task() {
    let task_id = tokio::task::try_id().unwrap();
    info!("started task {:?}", task_id);
    for i in 0..5000 {
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
            .service(task_stop)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
