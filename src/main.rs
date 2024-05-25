use actix_web::{web, App, HttpServer};
use app::database::database_connection;

use crate::handlers::user_handler::{create_user, get_users};
use crate::repository::user_repository::UserRepository;
use crate::services::user_service::UserService;

mod models {
    pub mod user;
}

mod app {
    pub mod database;
}

mod utils {
    pub mod helper;
}

mod repository {
    pub mod user_repository;
}

mod handlers {
    pub mod user_handler;
}

mod services {
    pub mod user_service;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database_connection();

    HttpServer::new(move || {
        let repo = UserRepository::new(pool.clone());
        let u_service = UserService::new(repo);
        let api_group = web::scope("/api/v1")
            .app_data(web::Data::new(u_service))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user));

        App::new().service(api_group)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
