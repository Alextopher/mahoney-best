use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{
    dev::HttpServiceFactory,
    error::ErrorInternalServerError,
    post,
    web::{self, redirect},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use maud::{html, DOCTYPE};

use crate::config::Config;

pub fn user_service() -> impl HttpServiceFactory {
    web::scope("/u")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(logout)
        .service(web::resource("/").route(web::get().to(index)))
        .service(redirect("", "/u/"))
}

async fn index(user: Option<Identity>, query: web::Query<RedirectQuery>) -> impl Responder {
    if let Some(user) = user {
        html! {
            (DOCTYPE)
            h1 { "Welcome, " (user.id().unwrap()) "!" }
            form method="post" action="/u/logout" {
                button { "Logout" }
            }
        }
    } else {
        html! {
            (DOCTYPE)
            h1 { "Please login" }
            form method="post" action=(format!("/u/login?redirect={}", query.redirect())) {
                input type="text" name="username" placeholder="Username";
                input type="password" name="password" placeholder="Password";
                button { "Login" }
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

/// The login endpoint is a POST request that takes a username and password as json in the request body.
///
/// A query parameter is also accepted, including a redirect URL to send the user to after a successful login.
///
/// If the username and password are correct, then the user's identity is attached to the active session.
async fn login(
    request: HttpRequest,
    config: web::Data<Arc<Config>>,
    login: web::Form<LoginReq>,
    query: web::Query<RedirectQuery>,
) -> actix_web::Result<impl Responder> {
    // user sends a username and password as json
    if config.check_admin(&login.username, &login.password) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "admin".into()).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found()
        .append_header(("location", query.redirect()))
        .finish())
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Found()
        .append_header(("location", "/u/"))
        .finish()
}

#[derive(Debug, serde::Deserialize)]
struct RedirectQuery {
    redirect: Option<String>,
}

impl RedirectQuery {
    fn redirect(&self) -> String {
        self.redirect.clone().unwrap_or_else(|| "/u/".to_string())
    }
}
