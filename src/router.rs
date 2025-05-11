use actix_web::{web, App, HttpResponse, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::dao::database;
use crate::handler::ping::ping;
use crate::handler::{user, post};

#[derive(OpenApi)]
#[openapi(
    info (
        title = "Blog",
        version = "1.0",
        description = "个人博客",
    ),
    paths(
        crate::handler::ping::ping,
        crate::handler::user::create,
        crate::handler::post::create,
    ),
    components(schemas(
        crate::model::request::UserCreateRequest,
        crate::model::request::PostCreateRequest,
    )),
)]
struct ApiDoc;

fn config_app(cfg: &mut web::ServiceConfig) {
    
    cfg
        .service(web::resource("/ping").route(web::get().to(ping)))
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/user")
                    .service(web::resource("/create").route(web::post().to(user::create))),
                )
                .service(
                    web::scope("/post")
                    .service(web::resource("/create").route(web::post().to(post::create)))
                )
        )
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .default_service(web::route().to(HttpResponse::NotFound));
}
pub async fn run_server() ->std::io::Result<()> {
    let database = database::Database::new().await;
    let share_data = web::Data::new(database);
    HttpServer::new(move || {
        App::new()
            .app_data(share_data.clone())
            .configure(config_app)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}