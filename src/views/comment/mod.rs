use super::path::Path;
use actix_web::web;
mod create;
mod delete;
mod edit;

pub fn comment_factory(app:&mut web::ServiceConfig) {
    let base_path: Path = Path{prefix: String::from("/comment"),
                                backend: true};

    app.route(&base_path.define(String::from("/{comment_unique_id}/edit")),
                web::put().to(edit::edit))
        .route(&base_path.define(String::from("/{comment_unique_id}/delete")),
                web::post().to(delete::delete))
        .route("/articles/{article_unique_id}/create_comment", web::post()
                .to(create::create));
}