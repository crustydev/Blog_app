use actix_web::web;
mod article;
mod auth;
mod blogger;
mod comment;
mod path;

pub fn views_factory(app:&mut web::ServiceConfig) {
    auth::auth_factory(app);
    article::article_factory(app);
    blogger::blogger_factory(app);
    comment::comment_factory(app);
}







