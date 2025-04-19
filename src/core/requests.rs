use std::time::SystemTime;

use hmac::{Hmac, Mac};
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT},
};
use sha2::{Digest, Sha256, Sha512};

use super::credentials::Credentials;

#[derive(Debug)]
pub enum ReqError {
    Reqwest(reqwest::Error),
    SystemTimeFailure,
    HmacFailure,
    InvalidHeader(InvalidHeaderValue),
    SigningError(base64::DecodeError),
    OverTheReqLimit,
}

impl From<reqwest::Error> for ReqError {
    fn from(src: reqwest::Error) -> Self {
        Self::Reqwest(src)
    }
}

impl From<InvalidHeaderValue> for ReqError {
    fn from(src: InvalidHeaderValue) -> Self {
        Self::InvalidHeader(src)
    }
}

pub struct RestClient {
    api_url: String,
    client: Client,
    creds: Credentials,
}

impl RestClient {
    pub fn new(api_url: String, creds: Credentials) -> Self {
        RestClient {
            api_url,
            client: Client::new(),
            creds,
        }
    }

    pub fn make_request(
        &self,
        endpoint: String,
        post_url: String,
        post_body: String,
    ) -> Result<Response, ReqError> {
        let post_data = post_url.clone() + &post_body;
        let nonce = Self::nonce()?.to_string();
        let sig = self.sign_message(&nonce, &endpoint, &post_data)?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert("APIKey", HeaderValue::from_str(&self.creds.key)?);
        headers.insert("Nonce", HeaderValue::from_str(&nonce)?);
        headers.insert("Authent", HeaderValue::from_str(&sig)?);

        let url = {
            if post_url.is_empty() {
                format!("{}{}", self.api_url, endpoint)
            } else {
                format!("{}{}?{}", self.api_url, endpoint, post_url)
            }
        };
        Ok(self.client.get(url).headers(headers).send()?)
    }

    fn sign_message(
        &self,
        nonce: &String,
        endpoint_path: &String,
        post_data: &String,
    ) -> Result<String, ReqError> {
        let mut endpoint_path = endpoint_path.clone();
        if endpoint_path.starts_with("/derivatives") {
            endpoint_path = endpoint_path["/derivatives".len()..].to_string();
        }

        // 1. Concatenate postData + Nonce + endpointPath
        let message = format!("{}{}{}", post_data, nonce, endpoint_path);

        // 2. Hash the result of step 1 with the SHA-256 algorithm
        let sha2_message = {
            let mut h = Sha256::default();
            h.update(&message);
            h.finalize()
        };

        // 3. Base64-decode your api_secret
        #[allow(deprecated)]
        let decoded_secret = base64::decode(&self.creds.secret).map_err(ReqError::SigningError)?;

        // 4. Use the result of step 3 to hash the result of the step 2 with the HMAC-SHA-512 algorithm
        let hmac_message = {
            let mut mac = Hmac::<Sha512>::new_from_slice(&decoded_secret)
                .map_err(|_| ReqError::HmacFailure)?;
            mac.update(&sha2_message);
            mac.finalize().into_bytes()
        };

        // 5. Base64-encode the result of step 4
        #[allow(deprecated)]
        let sig = base64::encode(hmac_message);

        Ok(sig)
    }

    fn nonce() -> Result<u64, ReqError> {
        Ok(SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ReqError::SystemTimeFailure)?
            .as_millis() as u64)
    }
}
