#![allow(non_snake_case)]
use serde_derive::{Serialize};

#[derive(Serialize, Debug)]
pub struct Albums {
    pub album_id: i64,
    pub title: String,
    pub artist_id: i64 
}