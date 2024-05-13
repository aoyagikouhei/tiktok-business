use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::collections::HashMap;
use tiktok_business::{
    apis::{get_business_comment_list, get_business_get, get_business_video_list},
    oauth::{TiktokOauth, TiktokScope},
    responses::{account::AccountField, video::VideoField},
};
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
    if let Some(token_data) = res.0.data {
        let res = get_business_get::Api::new(&token_data.open_id, AccountField::all())
            .execute(&token_data.access_token)
            .await
            .unwrap();
        println!("{:?}", res);
        let res = get_business_video_list::Api::new(&token_data.open_id, VideoField::all())
            .execute(&token_data.access_token)
            .await
            .unwrap();
        println!("{:?}", res);
        let video_id = res.body.data.as_ref().unwrap().videos.as_ref().unwrap().get(0).unwrap().item_id.as_ref().unwrap();
        let res = get_business_comment_list::Api::new(&token_data.open_id, video_id)
            .execute(&token_data.access_token)
            .await
            .unwrap();
        println!("{:?}", res);
    }

    "success".into_response()
}
