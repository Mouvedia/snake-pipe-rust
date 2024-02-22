use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use crate::input::{Game, InitOptions};
use crate::render_browser::broadcast::Broadcaster;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn do_broadcast_task(
    shutdown_marker: Arc<AtomicBool>,
    broadcaster: Arc<Broadcaster>,
    lines: Box<dyn Iterator<Item = Game>>,
) {
    for line in lines {
        if shutdown_marker.load(Ordering::SeqCst) {
            break;
        }
        let msg = serde_json::to_string(&line).unwrap();
        println!("{}\r", &msg);
        broadcaster.broadcast(&msg).await;
    }
}

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[get("/init-options")]
async fn get_init_options(init_options: web::Data<InitOptions>) -> impl Responder {
    let result = serde_json::to_string(&init_options).unwrap();
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
pub async fn launch_server(
    lines: Box<dyn Iterator<Item = Game>>,
    init_options: InitOptions,
) -> std::io::Result<()> {
    let broadcaster = Broadcaster::create();
    let broadcaster_clone = broadcaster.clone();
    let rc_init_options = Arc::new(init_options);

    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .app_data(web::Data::from(Arc::clone(&rc_init_options)))
            .service(event_stream)
            .service(get_init_options)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .disable_signals()
    .run();

    let server_handle = server.handle();
    let task_shutdown_marker = Arc::new(AtomicBool::new(false));

    let server_task = actix_web::rt::spawn(server);

    let broadcast_task = actix_web::rt::spawn(do_broadcast_task(
        Arc::clone(&task_shutdown_marker),
        broadcaster_clone,
        lines,
    ));

    let shutdown = actix_web::rt::spawn(async move {
        // listen for ctrl-c
        actix_web::rt::signal::ctrl_c().await.unwrap();

        // start shutdown of tasks
        let server_stop = server_handle.stop(true);
        task_shutdown_marker.store(true, Ordering::SeqCst);

        // await shutdown of tasks
        server_stop.await;
    });

    let _ = tokio::try_join!(server_task, broadcast_task, shutdown).expect("Unable to join tasks");

    Ok(())
}
