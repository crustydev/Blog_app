use actix_web::web;
mod create;
mod delete;
mod get;
use super::path::Path;



pub fn blogger_factory(app:&mut web::ServiceConfig) {
    let base_path: Path = Path {prefix: String::from("/blogger"), 
                                    backend: true};

    app.route(&base_path.define(String::from("/create")),
                 web::post().to(create::create))
        .route(&base_path.define(String::from("/delete")),
                 web::post().to(delete::delete))
        .route(&base_path.define(String::from("/profile/{username}}")),
                web::get().to(get::return_details));
}
