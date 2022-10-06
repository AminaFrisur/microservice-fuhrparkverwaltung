
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use postgres::{Client, NoTls};
use std::str::FromStr;
use itertools::Itertools;
use std::io::Read;

// TODO: Fuege Standort noch hinzu
// Buchungsservices orientieren sich nach bestimmten Standort -> deshalb dies hinzufügen

// TODO: Authentifizierung innerhalb der MS Architektur noch festlegen

// TODO: Rust nightly -> Wirklich empfehlenswert fuer production? -> gibt es auch eine Alternative um rocket zu kompelieren ?

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main()  {
    rocket::ignite().mount("/", routes![hello]).launch();

}


