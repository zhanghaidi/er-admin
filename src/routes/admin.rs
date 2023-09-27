use crate::app::admin::*;
use actix_web::web;

pub fn register_routes(s: &mut web::ServiceConfig) {
    s.service(
        // /api/v1/
        web::scope("/v1")
            .service(admin_user::logout)
            // /api/v1/adminUser
            .service(
                web::scope("/adminUser")
                    .service(admin_user::login)
                    .service(admin_user::profile)
                    .service(admin_user::update_password)
                    .service(admin_user::update_name),
            )
            // /api/v1/carousels
            .service(
                web::scope("/carousels")
                    .service(carousels::list)
                    .service(carousels::detail)
                    .service(carousels::delete)
                    .service(carousels::create)
                    .service(carousels::update),
            )
            // /api/v1/upload
            .service(
                web::scope("/upload")
                    .service(upload::file)
                    .service(upload::files),
            )
            // /api/v1/categories
            .service(
                web::scope("/categories")
                    .service(categories::list)
                    .service(categories::create)
                    .service(categories::delete)
                    .service(categories::update)
                    .service(categories::detail),
            )
            // /api/v1/users
            .service(
                web::scope("/users")
                    .service(users::list)
                    .service(users::lock_user),
            )
            // /api/v1/goods
            .service(web::scope("/goods")
                .service(goods::list)
                .service(goods::update)
                .service(goods::detail)
            ),
    );
}
