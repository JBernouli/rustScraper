use reqwest::Error;

#[tokio::main]

async fn main() -> Result<(), Error> {

    let baseUrl:String = "https://dnd5e.wikidot.com/".to_string();

    // Make a GET request to the specified URL.
    let response = reqwest::get(baseUrl+"/spells").await?;

    // Check if the request was successful (status code 200).
    if response.status().is_success() {
        // Read the response body as a string.
        let body = response.text().await?;
        println!("Body:\n{}", body);  
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

