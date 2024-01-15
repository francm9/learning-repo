use reqwest;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let test = vec![1, 2, 3];
    let _ = client.post("http://192.168.1.133:3000/")
        .json(&test)
        .send()
        .await?;
    Ok(())
}
