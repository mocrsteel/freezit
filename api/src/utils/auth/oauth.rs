use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl,
    Scope, TokenUrl,
};
use dotenvy::dotenv;
use std::env;
use super::providers::*;

pub fn get_oauth_link() {
    dotenv().ok();

    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET should be set"),
    );
    let google_client_id =
        ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID should be set"));
    let redirect_url =
        RedirectUrl::new(env::var("GOOGLE_REDIRECT_URL").expect("REDIRECT_URL should be set"))
            .expect("Invalid redirect URL");
    let auth_url =
        AuthUrl::new(GOOGLE_AUTH_URL.to_string()).expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).expect("Invalid token endpoint URL");
    let revocation_url = RevocationUrl::new(GOOGLE_REVOCATION_URL.to_string())
        .expect("Invalid revocation endpoint URL");

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
}
