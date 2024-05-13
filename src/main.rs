use actix_web::{web, App, HttpResponse, HttpServer};
use mysql::{prelude::Queryable, Pool};

mod models {
    pub mod user;
}

mod app {
    pub mod database;
}

mod utils {
    pub mod helper;
}

use app::database::database_connection;
use models::user::User;
use models::user::UserResponse;
use utils::helper::json_response;
extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

async fn get_users() -> HttpResponse {
    let pool = database_connection();
    let mut conn = pool.get_conn().unwrap();

    let users: Vec<User> = conn
        .query_map(
            "SELECT id, name, email, password FROM users",
            |(id, name, email, password)| User {
                id,
                name,
                email,
                password,
            },
        )
        .unwrap();

    let users_response: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
        .collect();

    json_response(
        "success",
        200,
        "Successfully get users",
        Some(users_response),
    )
}

async fn create_user(user: web::Json<User>) -> HttpResponse {
    let pool = database_connection();
    let mut conn = pool.get_conn().unwrap();
    let password = user.password.clone().unwrap_or_else(|| "".to_string());
    let pass_hash = hash(password, DEFAULT_COST).unwrap();
    conn.exec_drop(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        (user.name.clone(), user.email.clone(), pass_hash),
    )
    .unwrap();

    json_response("success", 201, "created user successfully", None::<User>)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let api_group = web::scope("/api/v1")
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user));

        App::new().service(api_group)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
