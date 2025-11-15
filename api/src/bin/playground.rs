//! Playground file.
use axum::{Router, routing::get, response::IntoResponse, Json};
use dotenvy::dotenv;

use oauth2::{basic::BasicClient, StandardRevocableToken, TokenResponse};
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenUrl};

use std::env;
use axum::response::Redirect;

// The following ones should be constants in the auth helpers since they are fixed for
// the Google OAuth2 API.
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const GOOGLE_REVOCATION_URL: &str = "https://oauth2.googleapis.com/revoke";
const GOOGLE_CERT_URL: &str = "https://www.googleapis.com/oauth2/v1/certs";

async fn hello() -> impl IntoResponse {
    String::from("Hello Yannick!")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET should be set"),
    );
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID should be set"),
    );
    let redirect_url = RedirectUrl::new(
        env::var("GOOGLE_REDIRECT_URL").expect("REDIRECT_URL should be set"),
    ).expect("Invalid redirect URL");
    let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).expect("Invalid token endpoint URL");
    let revocation_url = RevocationUrl::new(GOOGLE_REVOCATION_URL.to_string()).expect("Invalid revocation endpoint URL");

    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
        .set_revocation_url(revocation_url);

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's profile.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!("Open this URL in your browser:\n{authorize_url}\n");

    let app = Router::new()
        .route("/api/auth/login", get(|| async { Json(authorize_url)}))
        .route("/api", get(hello))
        .route("/", get(|| async { Redirect::permanent("/api") }))
        .route("/api/auth/callback/google", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
    // let redis_url = String::from("redis://127.0.0.1/");

}