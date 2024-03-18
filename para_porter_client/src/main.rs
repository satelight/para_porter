
#[tokio::main]
async fn main() {
    let res = reqwest::get("http://127.0.0.1:8000/").await.unwrap().text().await.unwrap();
    println!("{}",res);
}
