use std::time::Instant;

use reqwest::Client;

use super::creds::Credentials;
use crate::code;
use crate::error_handling::error_type::ErrorType;
use crate::error_handling::errors::Error;

#[derive(Debug, Clone)]
pub(crate) struct Cookie {
    pub(crate) cookie: String,
    time_of_creation: Instant,
}
impl Cookie {
    /// makes a new instance of `Cookie`.
    pub(crate) async fn new(
        authority: &String,
        reqwest_client: &Client,
        credentials: &Credentials,
    ) -> Result<Self, Error> {
        let now = Instant::now();

        let cookie = Self::request_raw_cookie(authority, reqwest_client, credentials).await?;

        Ok(Self {
            cookie,
            time_of_creation: now,
        })
    }

    pub(crate) async fn request_raw_cookie(
        authority: &String,
        reqwest_client: &Client,
        credentials: &Credentials,
    ) -> Result<String, Error> {
        let response = reqwest_client
            .post(format!("{}/api/v2/auth/login", authority))
            .header(reqwest::header::REFERER, authority)
            .form(&[
                ("username", credentials.username.clone()),
                ("password", credentials.password.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

        let status = response.status();

        if status.is_success() {
            match response
                .headers()
                .get("set-cookie")
                .and_then(|s| s.to_str().ok())
            {
                Some(cookie) => {
                    return Ok(cookie.to_string().split("=").collect::<Vec<&str>>()[1]
                        .split(";")
                        .collect::<Vec<&str>>()[0]
                        .to_string());
                }

                None => return Err(Error::build(ErrorType::WrongCreds, None)),
            }
        } else if status.as_u16() == 403 {
            return Err(Error::build(
                ErrorType::TooManyFailedAttempts,
                Some(403_u16),
            ));
        } else {
            return Err(Error::build(
                ErrorType::MiscError(
                    "Something went wrong while getting the auth cookie.".to_string(),
                ),
                code!(response),
            ));
        }
    }

    /// checks if the `Cookie` expired.
    ///
    /// # FUNCTIONING
    /// returns `true` when the cookie is expired.
    ///
    /// # WARNING
    /// - this method relies on the elapsed seconds from the time of creation. make sure to something that messes with that.
    /// - this will check that there is at least 1 minute of margin from the official expiration, to ensure any operation after this methis is called is still possible.
    pub(crate) fn is_expired(&self) -> bool {
        if self.time_of_creation.elapsed().as_secs() >= 3300 {
            true
        } else {
            false
        }
    }

    /// checks if the `Cookie` is expired, and if it is, requests a new one.
    pub(crate) async fn reset(
        &mut self,
        authority: &String,
        reqwest_client: &Client,
        credentials: &Credentials,
    ) -> Result<(), Error> {
        if self.is_expired() {
            self.cookie = Self::request_raw_cookie(authority, reqwest_client, credentials).await?;
        }

        Ok(())
    }
}
