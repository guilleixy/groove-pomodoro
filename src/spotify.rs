use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Artist{
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}


#[derive(Deserialize, Serialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Serialize, Debug)]
struct APIResponse{
    track: Items<Track>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Items<T>{
    items: Vec<T>,
}