use actix_web::web;
mod create;
mod delete;
mod view;
mod utils;
mod edit;
mod list;
use super::path::Path;


pub fn article_factory(app: &mut web::ServiceConfig) {
    let user_articles_path: Path = Path{prefix: String::from("/myarticles"), backend: true};
    let general_articles_path: Path = Path{prefix: String::from("/articles"), backend: true};

    app.route(&user_articles_path.define(String::from("")),
                web::get().to(list::get_user_articles))
        .route(&user_articles_path.define(String::from("/create")),
                web::put().to(create::create))
        .route(&user_articles_path.define(String::from("/{unique_id}")),
                web::get().to(view::writer_article_view))
        .route(&user_articles_path.define(String::from("/{unique_id}/delete")),
                web::post().to(delete::delete))
        .route(&user_articles_path.define(String::from("/{unique_id}/edit")),
                web::put().to(edit::edit))

                
        .route(&general_articles_path.define(String::from("{unique_id}")), web::get().to
                (view::reader_article_view))
        .route(&general_articles_path.define(String::from("")),
                 web::get().to(list::get_all_articles));
}