use std::collections::HashMap;
extern crate reqwest;
use std::fs::File;
use std::io::prelude::*;

async fn get_token() -> Result<String, Box<dyn std::error::Error>>
{
    let mut map = HashMap::new();
    map.insert("client_id", "enqh38t6jnsf5wrewjjlcn120e99d4");
    map.insert("client_secret", "qxh373tvktg9ls0tws8xd80t2wjk44");
    map.insert("grant_type", "client_credentials");
    map.insert("scope", "channel:manage:redemptions");
    let client = reqwest::Client::new();
    let response = client
        .post("https://id.twitch.tv/oauth2/token")
        .json(&map)
        .send()
        .await?;
       
    let jsoncontent = response.json::<serde_json::Value>().await?;
    println!("{:#?}",jsoncontent);
    let token = jsoncontent["access_token"].as_str().unwrap();
    
    Ok(token.to_string())
}

async fn get_user_access_token() -> Result<(), Box<dyn std::error::Error>>
{
let mut map = HashMap::new();
    map.insert("client_id", "enqh38t6jnsf5wrewjjlcn120e99d4");
    map.insert("redirect_uri", "http://localhost:3000");
    map.insert("response_type", "code");
    map.insert("scope", "channel:manage:redemptions");
    map.insert("force_verify", "true");
    // let mut map = HashMap::new();
    // map.insert("token", "Bearer 1odbcerdjbnydph4wdukqde31v0vor");
    // map.insert("client_id", "enqh38t6jnsf5wrewjjlcn120e99d4");
    
   // map.insert("Content-Type", "application/json");

   let client = reqwest::Client::new();

    let response = client
        .get("https://id.twitch.tv/oauth2/authorize")
        .json(&map)
        .send()
        .await?;
        println!("{:#?}", response.status());
  // let jsoncontent = response.json::<serde_json::Value>().await?;
  // let output = jsoncontent["access_token"].as_str().unwrap();
  let textcontent = response.text().await?;
  let mut file = File::create("index.html")?;
    file.write(textcontent.as_bytes()).ok();
    Ok(())
}

async fn get_broadcast_id() -> Result<(), Box<dyn std::error::Error>>
{
    use reqwest::header;
let mut headers = header::HeaderMap::new();
headers.insert("Authorization", header::HeaderValue::from_static("Bearer 1odbcerdjbnydph4wdukqde31v0vor"));
headers.insert("Client-ID", header::HeaderValue::from_static("enqh38t6jnsf5wrewjjlcn120e99d4"));

    // let mut map = HashMap::new();
    // map.insert("token", "Bearer 1odbcerdjbnydph4wdukqde31v0vor");
    // map.insert("client_id", "enqh38t6jnsf5wrewjjlcn120e99d4");
    
   // map.insert("Content-Type", "application/json");

   let client = reqwest::Client::builder()
   .default_headers(headers)
   .build()?;

    let response = client
        .get("https://api.twitch.tv/helix/streams?user_login=badukulelecover")
        .send()
        .await?;
        println!("{:#?}", response.status());
   let jsoncontent = response.json::<serde_json::Value>().await?;
  // let output = jsoncontent["access_token"].as_str().unwrap();
  
    println!("{:#?}", jsoncontent);

    Ok(())
}

async fn validate_token() -> Result<(), Box<dyn std::error::Error>>
{
    use reqwest::header;
let mut headers = header::HeaderMap::new();
headers.insert("Authorization", header::HeaderValue::from_static("Bearer 1odbcerdjbnydph4wdukqde31v0vor"));

    // let mut map = HashMap::new();
    // map.insert("token", "Bearer 1odbcerdjbnydph4wdukqde31v0vor");
    // map.insert("client_id", "enqh38t6jnsf5wrewjjlcn120e99d4");
    
   // map.insert("Content-Type", "application/json");

   let client = reqwest::Client::builder()
   .default_headers(headers)
   .build()?;

    let response = client
        .get("https://id.twitch.tv/oauth2/validate")
        .send()
        .await?;
        println!("{:#?}", response);
   let jsoncontent = response.json::<serde_json::Value>().await?;
  // let output = jsoncontent["access_token"].as_str().unwrap();
  
    println!("{:#?}", jsoncontent);

    Ok(())
}

async fn create_custom_reward() -> Result<(), Box<dyn std::error::Error>>
{
    use reqwest::header;
let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_static("Bearer 1odbcerdjbnydph4wdukqde31v0vor"));
    headers.insert("Client-ID", header::HeaderValue::from_static("enqh38t6jnsf5wrewjjlcn120e99d4"));
    let params = [("Content-Type", "application/json"), ("title", "league"), ("cost", "5000")];
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
    let response = client
        .post("https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=6ea6bca4-4712-4ab9-a906-e3336a9d8039")
        .form(&params)
        .send()
        .await?;
        //6ea6bca4-4712-4ab9-a906-e3336a9d8039
    let jsoncontent = response.json::<serde_json::Value>().await?;
   // let token = jsoncontent["access_token"].as_str().unwrap();
    println!("{:#?}", jsoncontent);
    Ok(())
}

fn main(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _token = rt.block_on(get_token()).ok();
}

