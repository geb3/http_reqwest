use std::{thread, time};
use reqwest::ClientBuilder;
use reqwest::Error;

fn to_str(http_url: &str) -> &str {
    return http_url;
}

fn check_url(http_url: String) -> String {
    if http_url[0..6].to_string() == String::from("https:") {
        return format!("{}", http_url);
    }
    if http_url[0..5].to_string() ==  String::from("http:") {
        return format!("{}", http_url);
    }
    else {
        return format!("http://{}/", http_url);
    }
}

#[tokio::main]
async fn request() -> Result<(), Error> {

    let delay_while_string: String = std::env::args().nth(1).expect("Ошибка аргумента 1)");
    let http_url: String = std::env::args().nth(2).expect("Ошибка аргумента 2)");

    let format_http_url: String = check_url(http_url);
    let request_url: &str = to_str(&format_http_url);
    let delay_while: u64 = delay_while_string.trim().parse::<u64>().unwrap();
    loop {
        let timeout: time::Duration = time::Duration::new(delay_while, 0);
        let client: reqwest::Client = ClientBuilder::new().timeout(timeout).build()?;
        let response: reqwest::Response = client.head(request_url).send().await?;
        
        let response_format: String = format!("{:?}", response.status()); 
        let http_code: i32 = response_format.trim().parse().unwrap();
        // println!("{:#?}", response);
        
        if response.status().is_success() {
            println!("Checking '{request_url}'. Result: OK({})", http_code);
        } 
        else {
            println!("Checking '{request_url}'. Result: ERR({})", http_code);
        }
        thread::sleep(timeout);  
    }
}

fn main() {
    let _check_request: () = match request() {
        Ok(_check_request) => _check_request,
        Err(_) => println!("URL parsing error")
    };
}