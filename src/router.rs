use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::dao::database;
use crate::handler::ping::ping;
use crate::handler::{article, comment, user};
use crate::middleware::cors::CORS;

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
        crate::handler::article::create,
        crate::handler::article::modify,
        crate::handler::article::delete,
        crate::handler::article::list,
        crate::handler::article::content_path,
        crate::handler::article::content,
        crate::handler::comment::create,
        crate::handler::comment::delete,
        crate::handler::comment::modify,
        crate::handler::comment::list,
    ),
    components(schemas(
        crate::models::requests::UserCreateRequest,
        crate::models::requests::ArticleCreateRequest,
        crate::models::requests::ArticleModifyRequest,
        crate::models::requests::CommentCreateRequest,
        crate::models::requests::CommentModifyRequest,
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
                    web::scope("/article")
                    .service(web::resource("/create").route(web::post().to(article::create)))
                    .service(web::resource("/modify").route(web::put().to(article::modify)))
                    .service(web::resource("/{aid}/delete").route(web::delete().to(article::delete)))
                    .service(web::resource("/list").route(web::get().to(article::list)))
                    .service(web::resource("/{aid}/content/path").route(web::get().to(article::content_path)))
                    .service(web::resource("/{aid}/content").route(web::get().to(article::content)))
                    .service(web::resource("/{aid}/comment").route(web::get().to(comment::list)))
                )
                .service(
                    web::scope("/comment")
                    .service(web::resource("/create").route(web::post().to(comment::create)))
                    .service(web::resource("/{cid}/delete").route(web::delete().to(comment::delete)))
                    .service(web::resource("/modify").route(web::put().to(comment::modify)))
                )
        )
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .service(Files::new("/static", "./static").prefer_utf8(true))
        .default_service(web::route().to(HttpResponse::NotFound));
}
pub async fn run_server() ->std::io::Result<()> {
    let database = database::Database::new().await;
    let share_data = web::Data::new(database);
    HttpServer::new(move || {
        App::new()
            .wrap(
                CORS()
            )
            .app_data(share_data.clone())
            .configure(config_app)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}