#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};

use std::{io::ErrorKind, sync::Arc};
use surrealdb::{sql::Object, Datastore, Session};

mod error;
mod prelude;
mod utils;
mod db;

fn main() {
    println!("Hello, world!");
}
