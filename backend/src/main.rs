use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};
use actix_web::{rt, web, App, HttpResponse, HttpServer};

struct AppState {
    request_count: Arc<Mutex<u64>>,
    request_count_sync: Arc<Mutex<u64>>
}

fn index(state: web::Data<AppState>) -> HttpResponse {
    let mut request_count = state.request_count.lock().unwrap();
    *request_count += 1;
    HttpResponse::NoContent().finish()
}

fn rps(state: web::Data<AppState>) -> HttpResponse {
    let request_count_sync = state.request_count_sync.lock().unwrap().clone();
    HttpResponse::Ok().body(format!("{}", request_count_sync))
}

fn main() {
    let request_count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let request_count_sync: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let request_count_clone = Arc::clone(&request_count);
    let request_count_sync_clone = Arc::clone(&request_count_sync);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));

            let mut request_count = request_count_clone.lock().unwrap();
            let mut request_count_sync = request_count_sync_clone.lock().unwrap();

            *request_count_sync = *request_count;
            *request_count = 0;
        }
    });

    rt::System::new("selfstat").block_on(async {
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    request_count: Arc::clone(&request_count),
                    request_count_sync: Arc::clone(&request_count_sync)
                })
                .route("/", web::get().to(index))
                .route("/rps", web::get().to(rps))
        })
        .bind("127.0.0.1:1337")
        .unwrap()
        .run()
        .await
        .unwrap();
    });
}