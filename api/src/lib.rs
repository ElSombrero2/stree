use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Responder};

async fn app () -> impl Responder {
    NamedFile::open("./web/dist/index.html")
}

pub async fn serve () {
    HttpServer::new(|| {
        let index =  Files::new("/", "./web/dist").index_file("index.html");
        App::new()
            .route("/test", web::get().to(app))
            .route("/test-2", web::get().to(app))
            .service(index)
    }).bind(("0.0.0.0", 3000))
    .unwrap().run().await.unwrap();
}
