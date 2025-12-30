use reqwest::{header::COOKIE, Client};
use serde::Serialize;
#[allow(unused_imports)]
use serde_json::Value;
use std::{collections::HashMap, hash::Hash, sync::Arc};
use tokio::sync::{Mutex, RwLock};

use crate::{
    code, core::cookie::Cookie, error_handling::error_type::ErrorType, post_request_no_return,
};

use super::creds::Credentials;
use crate::error_handling::errors::Error;

///## Description
/// the main struct of the library.
/// each API-related method is listed as its category, followed by its name: `category` + `name_of_the_method`, in snake case. (eg: `torrents_add_torrent`).
/// For more info about categories, look at the main  description of the library.
///
/// ## Methods
/// Practically all methods that return some kind of response, return a json one;
/// so as a rule of thumb:
/// - if the method ends with `raw`, it means it'll return the raw json [`String`] from the response.
/// - if the method doesn't have anything at its end, (or has `json` at the end, for cases where further clarity is needed), it'll return a serde_json [`Value`].
/// - if it ends in any other way, it returns a custom type that represents that json (or parts of it) in a particular way.
#[derive(Debug, Clone)]
pub struct QbitApi {
    pub(crate) authority: String,
    pub(crate) cookie: Arc<RwLock<Cookie>>,
    pub(crate) reqwest_client: Client,
    credentials: Credentials,
    cookie_hold: Arc<Mutex<bool>>,
}

impl QbitApi {
    /// ## Usage
    ///
    /// creates a new instance of [`QbitApi`].
    ///
    /// ## Arguments
    ///
    /// authority: the authority for the Qbittorrent WebUI API. eg: `"http://localhost:6011/"`
    /// credentials: the credentials to the account.
    ///
    /// ## Example
    /// ```
    /// let qbit_api = QbitApi::new("http://localhost:6011/", Credentials::new("user_name", "password")).await.unwrap();
    /// ```
    pub async fn new(authority: impl AsRef<str>, credentials: Credentials) -> Result<Self, Error> {
        let s = authority.as_ref();
        let authority = Into::<String>::into(s) as String;
        let authority = authority
            .chars()
            .rev()
            .skip_while(|s| *s as u8 == 47)
            .clone()
            .map(|k| k.to_string())
            .collect::<Vec<String>>()
            .into_iter()
            .rev()
            .collect::<String>();
        let reqwest_client = Client::new();
        let cookie = Arc::new(RwLock::new(
            Cookie::new(&authority, &reqwest_client, &credentials).await?,
        ));
        return Ok(QbitApi {
            authority,
            cookie,
            reqwest_client,
            credentials,
            cookie_hold: Arc::new(Mutex::new(false)),
        });
    }

    pub(crate) async fn get_cookie(&mut self) -> Result<String, Error> {
        let read_lock = self.cookie.read().await;

        let res: String;

        if read_lock.is_expired() {
            let hold_cookie = self.cookie_hold.clone();
            let mut hold_cookie = hold_cookie.lock().await;
            if read_lock.is_expired() && *hold_cookie == false {
                *hold_cookie = true;
                drop(read_lock);
                let cookie = self.cookie.clone();
                let mut cookie = cookie.write().await;
                cookie
                    .reset(&self.authority, &self.reqwest_client, &self.credentials)
                    .await?;
                res = (&*cookie.cookie).to_string();
                drop(cookie);
                *hold_cookie = false;
            } else {
                res = self.cookie.clone().read().await.cookie.clone();
            }
            drop(hold_cookie);
        } else {
            drop(read_lock);
            res = self.cookie.clone().read().await.cookie.clone();
        }
        Ok(res)
    }

    pub(crate) async fn make_request<T: Into<String>, S: Into<String>>(
        &mut self,
        url: T,
        custom_error: S,
    ) -> Result<String, crate::Error> {
        let resp = self
            .reqwest_client
            .post(format!("{}/api/v2{}", self.authority, url.into()))
            .header(COOKIE, format!("SID={}", self.get_cookie().await?))
            .send()
            .await
            .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

        if resp.status().is_success() {
            let text = resp
                .text()
                .await
                .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

            return Ok(text);
        } else {
            return Err(Error::build(
                ErrorType::MiscError(format!(
                    "something went wrong. function name: {}",
                    custom_error.into()
                )),
                Some(resp.status().as_u16()),
            ));
        }
    }

    pub(crate) async fn make_request_with_form<
        T: std::cmp::Eq + Hash + Serialize,
        S: Serialize,
        U,
        N: Into<String>,
        X: Into<String>,
    >(
        &mut self,
        url: N,
        custom_error: X,
        hashmap: HashMap<T, S, U>,
    ) -> Result<String, crate::Error> {
        let response = self
            .reqwest_client
            .post(format!("{}/api/v2{}", self.authority, url.into()))
            .header(
                reqwest::header::COOKIE,
                format!("SID={}", self.get_cookie().await?),
            )
            .form(&hashmap)
            .send()
            .await
            .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

        if response.status().is_success() {
            let text = response
                .text()
                .await
                .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;
            Ok(text)
        } else {
            Err(Error::build(
                ErrorType::MiscError(format!("function name: {}", custom_error.into())),
                Some(response.status().as_u16()),
            ))
        }
    }

    pub(crate) async fn make_request_with_form_hash<
        T: std::cmp::Eq + Hash + Serialize,
        S: Serialize,
        U,
        N: Into<String>,
        X: Into<String>,
    >(
        &mut self,
        url: N,
        custom_error: X,
        hashmap: HashMap<T, S, U>,
    ) -> Result<String, crate::Error> {
        let response = self
            .reqwest_client
            .post(format!("{}/api/v2{}", self.authority, url.into()))
            .header(
                reqwest::header::COOKIE,
                format!("SID={}", self.get_cookie().await?),
            )
            .form(&hashmap)
            .send()
            .await
            .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;

        // Handle the response
        if response.status().is_success() {
            let text = response
                .text()
                .await
                .map_err(|e| Error::build(ErrorType::ReqwestError(Box::new(e)), None))?;
            Ok(text)
        } else if response.status().as_u16() == 404 {
            Err(Error::build(
                ErrorType::TorrentHashNotFound,
                code!(response),
            ))
        } else {
            Err(Error::build(
                ErrorType::MiscError(format!(
                    "something went wrong. function name: {}",
                    custom_error.into()
                )),
                code!(response),
            ))
        }
    }

    post_request_no_return!(logout, "/auth/logout");
}
