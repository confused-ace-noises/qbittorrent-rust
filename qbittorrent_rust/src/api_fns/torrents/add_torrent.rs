use core::panic;
use std::borrow::Borrow;

use proc_macros_qbittorrent_rust::Builder;
use reqwest::header;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    code, core::api::QbitApi, error_handling::{error_type::ErrorType, errors::Error}, misc::sep_vec::SepVec
};

use super::torrents::Torrent;

/// ## Info
/// struct that describes the adding of a torrent.
/// create a new [`TorrentAddDescriptor`] by either:
/// - using the `new` function;
/// - using the `builder` function; if you use this option, remember to always set the `torrents`. 
#[derive(Debug, Clone)]
pub struct TorrentAddDescriptor {
    urls: SepVec<String, String>,

    paths: Vec<String>,

    /// Download folder path
    savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    cookie: Option<String>,

    /// Category for the torrent
    category: Option<String>,

    /// Tags for the torrent, separated by commas
    tags: Option<SepVec<String, char>>,

    /// Skip hash checking (true, false)
    skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    root_folder: Option<String>,

    /// Rename the torrent
    rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    seeding_time_limit: Option<u32>,

    /// Use Automatic Torrent Management
    auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptor {
    /// ## Usage
    /// creates a new instance of [`TorrentAddDescriptor`].
    /// 
    /// ## Warning
    /// the `torrents` argument mustn't be an empty vector. If it is, the function will return an [`Error`] with error type [`ErrorType::TorrentsNotSet`]
    pub fn new(torrents: Vec<Torrent>) -> Result<Self, Error> {
        Self::builder(torrents).build()
    }

    /// ## Usage
    /// returns a new instance of [`TorrentAddDescriptorBuilder`]: the builder for [`TorrentAddDescriptor`].
    ///
    /// ## Warning
    /// the argument `torrents` mustn't be empty, or the `build` function for the [`TorrentAddDescriptor`] struct will return an [`Error`] with error type [`ErrorType::TorrentsNotSet`]
    pub fn builder(torrents: Vec<Torrent>) -> TorrentAddDescriptorBuilder {
        TorrentAddDescriptorBuilder::new(torrents)
    }
}

/// ## Info
/// the builder struct for [`TorrentAddDescriptor`].
/// to have a description of what each setter does, look at the [qbittorrent documentation]()
/// ## Usage
/// call its methods and set the various fields.
/// Once you're done, call `build()`
/// 
/// ## Warning
/// - The `torrents` field is a vector, so it could be put as empty. Although, this isn't supported, and when `build()` is called, it will return an [`Error`] with error type: [`ErrorType::TorrentsNotSet`]
/// 
/// ## Fields:
/// | Property                   | Type    | Description                                                                 |
/// |----------------------------|---------|-----------------------------------------------------------------------------|
/// | `torrents`                 | `Vec<Torrent>`     | A vector of [`Torrent`]. Represents the torrents to download.        |
/// | `savepath`       |  `String`  | Download folder.                                                            |
/// | `cookie`         |  `String`  | Cookie sent to download the .torrent file.                                  |
/// | `category`       |  `String`  | Category for the torrent.                                                   |
/// | `tags`           |  `Vec<String>`  | Tags for the torrent.                                         |
/// | `skip_checking`  |  `Bool`  | Whether to skip hash checking. (default: false)|
/// | `paused`         |  `Bool`  | Add torrents in the paused state. (default: false)|
/// | `root_folder`    |  `Bool`  | Whether to create the root folder.|
/// | `rename`         |  `String`  | Rename the torrents.                                                             |
/// | `upLimit`        | `Integer` | Set torrent upload speed limit. Unit in bytes/second                       |
/// | `dlLimit`        | `Integer` | Set torrent download speed limit. Unit in bytes/second                     |
/// | `ratioLimit`  | `Float`   | Set torrent share ratio limit.                                              |
/// | `seedingTimeLimit`  | `Integer` | Set torrent seeding time limit. Unit in minutes.                            |
/// | `autoTMM`        | `Bool`    | Whether Automatic Torrent Management should be used. (default: false)                        |
/// | `sequentialDownload`  |  `Bool`  | Enable sequential download. (default: false) |
/// | `firstLastPiecePrio`  |  `Bool`  | Prioritize download first last piece. (default: false) |
#[derive(Debug, Clone, Builder)]
pub struct TorrentAddDescriptorBuilder {
    #[builder(custom)]
    torrents: Option<Vec<Torrent>>,

    /// Download folder path
    savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    cookie: Option<String>,

    /// Category for the torrent
    category: Option<String>,

    /// Tags for the torrent, separated by commas
    tags: Option<Vec<String>>,

    /// Skip hash checking (true, false)
    skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    root_folder: Option<bool>,

    /// Rename the torrent
    rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    seeding_time_limit: Option<u32>,

    /// doc
    auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptorBuilder {
    ///## Info 
    /// 
    /// creates a new instance of [`TorrentAddDescriptorBuilder`].
    /// 
    /// ## Warning
    /// if `torrents` is set as an empty vector, the `build` function will return an [`Error`] with error type [`ErrorType::TorrentsNotSet`]
    pub fn new(torrents: Vec<Torrent>) -> Self {
        Self {
            torrents: Some(torrents),
            savepath: None,
            cookie: None,
            category: None,
            tags: None,
            skip_checking: None,
            paused: None,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            ratio_limit: None,
            seeding_time_limit: None,
            auto_tmm: None,
            sequential_download: None,
            first_last_piece_prio: None,
        }
    }

    /// ## Info
    /// returns the finalized [`TorrentAddDescriptor`].
    ///
    /// ## Errors
    /// - if the `torrent`s vector was set as empty, it will return an [`Error`] with error type [`ErrorType::TorrentsNotSet`].
    pub fn build(self) -> Result<TorrentAddDescriptor, Error> {
        let (urls, paths) = match self.torrents {
            Some(t) => {
                if t.is_empty() {
                    return Err(Error::build(ErrorType::TorrentsNotSet, None));
                } else {
                    let mut vec_urls = vec![];
                    let mut vec_paths = vec![];

                    for item in t.iter().map(|l| l.get_inner()) {
                        match item {
                            crate::api_fns::torrents::torrents::TorrentInner::Url(url) => {
                                vec_urls.push(url)
                            }
                            crate::api_fns::torrents::torrents::TorrentInner::RawTorrent(path) => {
                                vec_paths.push(path)
                            }
                        }
                    }

                    (SepVec::new(vec_urls, "".to_string()), vec_paths)
                }
            }
            None => {
                return Err(Error::build(ErrorType::TorrentsNotSet, None))
            }
        };

        let tags = self.tags.and_then(|v| Some(SepVec::new(v, ',')));

        let root_folder = match self.root_folder {
            None => String::from("unset"),
            Some(true) => String::from("true"),
            Some(false) => String::from("false"),
        };

        Ok(TorrentAddDescriptor {
            urls,
            paths,
            savepath: self.savepath,
            cookie: self.cookie,
            category: self.category,
            tags: tags,
            skip_checking: self.skip_checking,
            paused: self.paused,
            root_folder: Some(root_folder),
            rename: self.rename,
            up_limit: self.up_limit,
            dl_limit: self.dl_limit,
            ratio_limit: self.ratio_limit,
            seeding_time_limit: self.seeding_time_limit,
            auto_tmm: self.auto_tmm,
            sequential_download: self.sequential_download,
            first_last_piece_prio: self.first_last_piece_prio,
        })
    }
}

impl QbitApi {
    ///## Usage
    /// adds one (or more) torrents.
    pub async fn torrents_add_torrent(&mut self, descriptor: impl Borrow<TorrentAddDescriptor>) -> Result<(), Error> {
        let descriptor = descriptor.borrow();

        match (
            descriptor.paths.is_empty(),
            descriptor.urls.inner_vec().is_empty(),
        ) {
            (true, true) => panic!(),
            (true, false) => {
                let mut form_urls = reqwest::multipart::Form::new();

                form_urls = form_urls.text("urls", descriptor.urls.to_string());

                form_urls = thing(form_urls, descriptor.clone());

                let response_urls = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_urls)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                    .send()
                    .await
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?;

                if response_urls.status().is_success() {
                    return Ok(());
                } else {
                    return Err(Error::build(ErrorType::MiscNetError(code!(response_urls).unwrap()), code!(response_urls)));
                }
            }
            (false, true) => {
                let form = torrents_part(&descriptor).await?;

                let response_torrents = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                    .send()
                    .await
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?;

                if response_torrents.status().is_success() {
                    return Ok(());
                } else {
                    return Err(Error::build(ErrorType::MiscNetError(code!(response_torrents).unwrap()), code!(response_torrents)));
                }
            }

            (false, false) => {
                // ---------- TORRENT FILES ----------
                let form_torrents = torrents_part(&descriptor).await?;

                let built_torrents = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_torrents)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?));

                // ---------- TORRENT FILES ----------

                // ---------- URLS ----------
                let mut form_urls = reqwest::multipart::Form::new();

                form_urls = form_urls.text("urls", descriptor.urls.to_string());
                form_urls = thing(form_urls, descriptor.clone());

                let built_urls = self
                    .reqwest_client
                    .post(format!("{}/api/v2/torrents/add", self.authority))
                    .multipart(form_urls)
                    .header(header::COOKIE, format!("SID={}", self.get_cookie().await?));

                // ---------- URLS ----------

                let (response_torrents, response_urls) =
                    tokio::join!(built_torrents.send(), built_urls.send());

                let mut thing = (false, false);

                if response_torrents
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?
                    .status()
                    .is_success()
                {
                    thing.0 = true;
                }

                if response_urls
                    .map_err(|e| {
                        Error::build(ErrorType::ReqwestError(Box::new(e)), None)
                    })?
                    .status()
                    .is_success()
                {
                    thing.1 = true
                }

                match thing {
                        (true, true) => return Ok(()),
                        (true, false) => return Err(Error::build(ErrorType::MiscError("something went wrong while adding urls.".to_string()), None)),
                        (false, true) => return Err(Error::build(ErrorType::MiscError("something went wrong while adding torrent files.".to_string()), None)),
                        (false, false) => return Err(Error::build(ErrorType::MiscError("wow, you really messed up. both torrents and urls failed.".to_string()), None)),
                    }
            }
        };
    }
}

fn thing(
    mut form: reqwest::multipart::Form,
    descriptor: TorrentAddDescriptor,
) -> reqwest::multipart::Form {
    if let Some(savepath) = descriptor.savepath {
        form = form.text("savepath", savepath);
    }

    if let Some(cookie) = descriptor.cookie {
        form = form.text("cookie", cookie);
    }

    if let Some(category) = descriptor.category {
        form = form.text("category", category);
    }

    if let Some(tags) = descriptor.tags {
        form = form.text("tags", tags.to_string());
    }

    if let Some(skip_checking) = descriptor.skip_checking {
        form = form.text("skip_checking", skip_checking.to_string());
    }

    if let Some(paused) = descriptor.paused {
        form = form.text("paused", paused.to_string());
    }

    if let Some(root_folder) = descriptor.root_folder {
        form = form.text("root_folder", root_folder);
    }

    if let Some(rename) = descriptor.rename {
        form = form.text("rename", rename);
    }

    if let Some(up_limit) = descriptor.up_limit {
        form = form.text("upLimit", up_limit.to_string());
    }

    if let Some(dl_limit) = descriptor.dl_limit {
        form = form.text("dlLimit", dl_limit.to_string());
    }

    if let Some(ratio_limit) = descriptor.ratio_limit {
        form = form.text("ratioLimit", ratio_limit.to_string());
    }

    if let Some(seeding_time_limit) = descriptor.seeding_time_limit {
        form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
    }

    if let Some(auto_tmm) = descriptor.auto_tmm {
        form = form.text("autoTMM", auto_tmm.to_string());
    }

    if let Some(sequential_download) = descriptor.sequential_download {
        form = form.text("sequentialDownload", sequential_download.to_string());
    }

    if let Some(first_last_piece_prio) = descriptor.first_last_piece_prio {
        form = form.text("firstLastPiecePrio", first_last_piece_prio.to_string());
    }

    form
}

async fn torrents_part(
    descriptor: &TorrentAddDescriptor,
) -> Result<reqwest::multipart::Form, Error> {
    let mut form_torrents = reqwest::multipart::Form::new();
    for path in descriptor.paths.clone() {
        let mut file = File::open(path)
            .await
            .map_err(|_| Error::build(ErrorType::TorrentFilePathError, None))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .await
            .map_err(|_| Error::build(ErrorType::TorrentFilePathError, None))?;

        // part 4 the multipart form
        let file_part = reqwest::multipart::Part::bytes(buffer)
            .file_name("torrent_file.torrent")
            .mime_str("application/x-bittorrent")
            .unwrap();

        form_torrents = form_torrents.part("torrents", file_part);
    }
    Ok(form_torrents)
}