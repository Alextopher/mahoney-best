use actix_identity::Identity;
use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, redirect},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use maud::html;

use crate::config::Config;

pub fn user_service() -> impl HttpServiceFactory {
    web::scope("/u")
        .service(login)
        .service(logout)
        .service(index)
        .service(redirect("", "/u/"))
}

#[get("/")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        html! {
            h1 { "Welcome, " (user.id().unwrap()) "!" }
            form method="post" action="/u/logout" {
                button { "Logout" }
            }
        }
    } else {
        html! {
            h1 { "Please login" }
            form method="post" action="/u/login" {
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
#[post("/login")]
async fn login(
    config: web::Data<Config>,
    request: HttpRequest,
    login: web::Form<LoginReq>,
    redirect: web::Query<Option<String>>,
) -> impl Responder {
    // user sends a username and password as json
    if config.check_admin(&login.username, &login.password) {
        return HttpResponse::Unauthorized().finish();
    }

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "admin".into()).unwrap();

    HttpResponse::Found()
        .append_header(("location", redirect.0.as_deref().unwrap_or("/u/")))
        .finish()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Found()
        .append_header(("location", "/u/"))
        .finish()
}
