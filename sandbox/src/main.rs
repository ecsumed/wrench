extern crate reqwest;

// use std::collections::HashMap;

use reqwest::header;



fn main() -> Result<(), Box<std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    //headers.insert(header::USER_AGENT, header::HeaderValue::from_static("randomtest"));
   
    let custom_header = header::HeaderName::from_static("user-agent");

    headers.insert(custom_header, header::HeaderValue::from_static("Chrome 1.0/"));

    // get a client builder
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let resp = client.get("http://phpstack-10392-383663.cloudwaysstagingapps.com")
        .send()?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
