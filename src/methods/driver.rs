use reqwest::Response;

pub struct Delta {
    pub url: String,
    pub token: String,
    pub timeout: std::time::Duration,
    pub auth: Auth,
}

#[derive(Clone)]
pub struct Auth {
    usertype: String,
    token: String,
}

impl Delta {
    pub fn new(url: &str, token: &str, timeout: u64, isbot: bool) -> Self {
        let usertype = match isbot {
            true => "x-bot-token",
            false => "x-session-token",
        }
        .to_string();

        let auth = Auth {
            usertype,
            token: String::from(token),
        };

        Self {
            url: String::from(url),
            token: String::from(token),
            timeout: std::time::Duration::from_secs(timeout),
            auth,
        }
    }

    pub async fn get(&self, route: &str) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::GET,
            None,
            self.auth.clone(),
        )
        .await
    }
    pub async fn post(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::POST,
            data,
            self.auth.clone(),
        )
        .await
    }

    pub async fn put(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PUT,
            data,
            self.auth.clone(),
        )
        .await
    }

    pub async fn patch(&self, route: &str, data: Option<&str>) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::PATCH,
            data,
            self.auth.clone(),
        )
        .await
    }
    pub async fn delete(
        &self,
        route: &str,
        data: Option<&str>,
    ) -> Result<Response, reqwest::Error> {
        common(
            &format!("{}{}", self.url, route),
            reqwest::Method::DELETE,
            data,
            self.auth.clone(),
        )
        .await
    }
}

pub async fn common(
    url: &str,
    method: reqwest::Method,
    data: Option<&str>,
    auth: Auth,
) -> Result<Response, reqwest::Error> {
    // custom client build conf
    let client_b = reqwest::ClientBuilder::new()
        .user_agent("RevoltBot/10.0 (Linux; async-tokio-runtime)")
        .timeout(std::time::Duration::from_secs(10));
    // client constructor
    let mut client = client_b
        .build()
        .unwrap()
        .request(method, url)
        .timeout(std::time::Duration::from_secs(10))
        .header("Idempotency-Key", rand::random::<u64>());

    // token
    client = client.header(auth.usertype, auth.token);

    // optional fields
    if let Some(json) = data {
        let json = json.to_string();
        client = client.header("Content-Type", "application/json").body(json);
    };

    client.send().await
}

#[derive(Debug)]
pub enum DeltaError {
    HTTP(reqwest::StatusCode),
    REQWEST(reqwest::Error),
}

pub async fn result<T: serde::de::DeserializeOwned>(
    http: Result<Response, reqwest::Error>,
) -> Result<T, DeltaError> {
    let res = http;
    let result: T = match res {
        Err(http) => {
            return Err(DeltaError::REQWEST(http));
        }
        Ok(a) => {
            if !a.status().is_success() {
                return Err(DeltaError::HTTP(a.status()));
            }
            if a.status() == 204 {
                return Ok(serde_json::from_value(serde_json::Value::Null).unwrap());
            }

            match a.json().await {
                Ok(a) => a,
                Err(a) => return Err(DeltaError::REQWEST(a)),
            }
        }
    };
    Ok(result)
}
