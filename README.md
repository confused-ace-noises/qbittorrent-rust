# <img src="https://github.com/confused-ace-noises/qbittorrent-rust/blob/master/.assets/qbittorrent.svg?raw=true" alt="qbittorrent emoji" width="35" height="35" style="vertical-align: middle; margin-top: -5px;"> Qbittorrent-rust <img src="https://github.com/confused-ace-noises/qbittorrent-rust/blob/master/.assets/rust_lang-white.png?raw=true" height="35" width="35" style="vertical-align: middle; margin-top: -5px;">
An asynchronous rust library to interface with the qbittorrent WebUI API.

## Goals 🏁
This library's goal is the one to reflect the qbittorrent WebUI API in its entirety, and simultaneously being simple, fast, and concise.

## <img src="https://github.com/confused-ace-noises/qbittorrent-rust/blob/master/.assets/qbittorrent-rust-white.png?raw=true" width="35" height="35" style="vertical-align: middle; margin-top: -5px;"> Qbittorrent-rust in action <img src="https://github.com/confused-ace-noises/qbittorrent-rust/blob/master/.assets/qbittorrent-rust-white.png?raw=true" width="35" height="35" style="vertical-align: middle; margin-top: -5px;">

```rust
use qbittorrent_rust::*;
use tokio;

// The example uses tokio, but you can use your favorite asynchronous runtime.
#[tokio::main]
fn main() {
    // Set the credentials.
    let credentials = Credentials::new("username", "password");

    // Define QbitApi with the authority of the qbitorrent api and your credentials. 
    let mut api = QbitApi::new("http://localhost:6001/", credentials).await.unwrap();

    // You're all set up!
    // Now, you can use the api variable to make whichever api request you'd like.

    // Let's see how to add a torrent.
    
    // First, define your torrents.
    let torrent_1 = Torrent::new(TorrentType::Url("https://torrents/"));
    let torrent_2 = Torrent::new(TorrentType::TorrentFile("path/to/the/torrent/file"));

    // Now, define your TorrentAddDescriptor.
    // this defines all the settings about downloading torrents:
    // the savepath, the categories, etc;
    // most importantly, always remember to set your torrents to a non-empty vector, or the method will return an error.
    let torrent_add_desc = TorrentAddDescriptor::builder(vec![&torrent1, &torrent2])
        .savepath("/path/to/save/location")
        .build();

    // Finally, send your request!
    api.torrents_add_torrent(&TorrentAddDescriptor).await.unwrap();
}
```

## Features 🛠️
- Complete API parity: everything you could do with the Qbittorrent WebUI API, you can also do in this library!
- Automatic cookie handling: forget about handling your access cookies, the library handles and renews your cookies for you!
- Extremely user-friendly methods: when something doesn't need to be it's own type, it's just plain primary types, making the process of managing the methods simpler.
- Asynchronicity: this library is built to be asynchronous and as fast as possible.
- Complete documentation: the whole library has been documented, in a short and concise way.
- Freedom-giving: this library also aims to giving fine control to the requests done to the API.
- Similar structure to the native API: using this library is very simple, and almost every functionality is similarly structured to the actual native API.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE.txt">Apache License, Version
2.0</a> or <a href="LICENSE-MIT.txt">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
