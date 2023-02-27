// use axum::response::Redirect;
// use dotenvy::dotenv;
// use serde::{Deserialize, Serialize};
// use std::env;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct QueryString {
//     client_id: String,
//     redirect_uri: String,
//     response_type: String,
//     access_type: String,
//     scope: Vec<String>,
// }

// pub async fn auth() -> Redirect {
//     //load env variables
//     dotenv().ok();
//     let q = QueryString {
//         client_id: env::var("GOOGLE_CLIENT_ID").expect("Variable not found inside env file"),
//         redirect_uri: "http://localhost:3000/auth/google/callback".to_string(),
//         response_type: "code".to_string(),
//         access_type: "offline".to_string(),
//         scope: vec![],
//     };

//     let redirect_url = format!(
//         "https://accounts.google.com/o/oauth2/v2/auth?scope=https%3A//www.googleapis.com/auth/drive.metadata.readonly&access_type=offline&include_granted_scopes=true&response_type=code&state={}&redirect_uri=https%3A//localhost%3A3000/auth/google/callback&client_id={}", "boogabooga", q.client_id
//     );

//     return Redirect::temporary(&redirect_url);
// }

#[derive(Debug)]
pub struct User {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
