use super::bench;

#[derive(Clone)]
pub enum Method {
    Get,
    Head,
}

#[derive(Clone)]
pub struct Engine {
    urls: Vec<String>,
    headers: Vec<String>,
    method: Method,
}

impl Engine {
    pub fn new(urls: Vec<String>) -> Engine {
        Engine{
            urls,
            headers: vec![],
            method: Method::Get,
        } 
    }

    fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn run(&self, requests: usize) {
        self.run_reqwest(requests); 
    }


	fn run_reqwest(&self, requests: usize) {
        use reqwest::header;
        use reqwest::{Client, Request};

        let mut headers = header::HeaderMap::new();

        self.headers.iter().for_each(|header| {
            let values: Vec<String> = header.split("=")
                                            .map(|x| x.to_string())
                                            .collect();
            headers.insert(
                header::HeaderName::from_lowercase(values[0].as_bytes())
                                    .expect("invalid header name."),
                header::HeaderValue::from_str(&values[1])
                                     .expect("invalid header value.")
            );
        });

        let method = match self.method {
            Method::Get => reqwest::Method::GET,
            Method::Head => reqwest::Method::HEAD,
        }; 

        let client = Client::builder()
                    .default_headers(headers)
                    .build().expect("Failed to build client");

        for n in 0..requests {
            let url = &self.urls[n % self.urls.len()];
            let request = Request::new(method.clone(), url.parse().expect("Invalid url."));
            let mut len = 0;

            let (resp, duration) = bench::time_it(|| {
                let mut resp = client
                               .execute(request)
                               .expect("Failure to even connect is no good");
                if let Ok(body) = resp.text() {
                    len = body.len();
                }
                resp
            });

            println!("status: {} bench: {:?}",resp.status().as_u16(), duration)
        }
        
    }
}
