use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::collections::HashMap;
use tiktok_business_api::oauth::{TiktokOauth, TiktokScope};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use url::Url;

pub const CSRF_TOKEN: &str = "csrf_token";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/oauth", get(oauth))
        .route("/", get(root))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn oauth_client() -> TiktokOauth {
    TiktokOauth::new(
        &std::env::var("CLIENT_KEY").unwrap(),
        &std::env::var("CLIENT_SECRET").unwrap(),
        &std::env::var("CALLBACK_URL").unwrap(),
        TiktokScope::tiktok_accounts(),
    )
    .unwrap()
}

async fn root(cookies: Cookies) -> impl IntoResponse {
    let oauth = oauth_client();
    let res = oauth.oauth_url(None);
    cookies.add(Cookie::new(CSRF_TOKEN, res.csrf_token.clone()));
    Html(format!("<a href='{}'>oauth<a>", res.oauth_url)).into_response()
}

async fn oauth(uri: Uri, cookies: Cookies) -> impl IntoResponse {
    let url = Url::parse(&format!("http://localhost:3000{}", uri)).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let csrf = cookies.get(CSRF_TOKEN).unwrap();
    if csrf.value() != hash_query.get("state").unwrap() {
        return "csrf token error".into_response();
    }
    let oauth = oauth_client();
    let res = oauth.token(hash_query.get("code").unwrap()).await.unwrap();
    println!("{:?}", res);
    "success".into_response()
}
