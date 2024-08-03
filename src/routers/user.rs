use actix_web::{HttpResponse, Responder, web};
use crate::models::user::CreateUser;


pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            //.route("", web::post().to(create_user))
            .route("{id}", web::get().to(|user_controller: web::Data<UserController>, path: web::Path<i32>| async move {
                user_controller.get_user(path).await
            }))
    );
}


// async fn create_user(web::Json(user): web::Json<CreateUser>) -> actix_web::Result<impl Responder> {
//     Ok(web::Json(user))
// }
//
async fn get_user_by_id() -> impl Responder {
    //self.user_service.get_user_by_id(id);
    format!("{}", "Pong!")
}


struct UserController {
    //user_service: Box<dyn IUserService>,
}

impl UserController {
    pub fn new() -> Self {
        UserController {
            //user_service: Box::new(UserService::new(Box::new(UserRepo::new(pool)))),
        }
    }
    pub async fn get_user(&self, path: web::Path<i32>) -> impl Responder {
        format!("{}", path)
    }
}
