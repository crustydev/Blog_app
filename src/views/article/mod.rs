use actix_web::web;
mod create;
mod delete;
mod view;
mod utils;
mod edit;
mod list;
use super::path::Path;


pub fn article_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path{prefix: String::from("/my_articles"), backend: true};

    app.route(&base_path.define(String::from("/")),
                web::get().to(list::list_articles))
        .route(&base_path.define(String::from("/{unique_id}")),
                web::get().to(view::writer_article_view))
        .route(&base_path.define(String::from("/create")),
                web::put().to(create::create))
        .route(&base_path.define(String::from("/{unique_id}/delete")),
                web::post().to(delete::delete))
        .route(&base_path.define(String::from("/unique_id/edit")),
                web::put().to(edit::edit))
        .route("/articles/{unique_id}", web::get().to
                (view::reader_article_view));
}