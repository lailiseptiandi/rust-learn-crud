use crate::services::user_service::UserService;
use crate::{models::user::User, utils::helper::json_response};
use actix_web::{web, HttpResponse, Responder};

pub async fn get_users(service: web::Data<UserService>) -> impl Responder {
    match service.get_all_users() {
        Ok(users) => json_response("success", 200, "Successfully get users", Some(users)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(service: web::Data<UserService>, item: web::Json<User>) -> impl Responder {
    match service.create_user(item.into_inner()) {
        Ok(_) => json_response("success", 201, "Successfully create user", Some(())),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
