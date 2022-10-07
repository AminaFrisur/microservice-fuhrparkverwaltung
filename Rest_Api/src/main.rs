
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use postgres::{Client, NoTls};
use std::str::FromStr;
use itertools::Itertools;
use std::io::Read;
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

// TODO: set HTTP Statuscode for Errors

// TODO: Fuege Standort noch hinzu
// Buchungsservices orientieren sich nach bestimmten Standort -> deshalb dies hinzufügen

// TODO: Authentifizierung innerhalb der MS Architektur noch festlegen

// TODO: Rust nightly -> Wirklich empfehlenswert fuer production? -> gibt es auch eine Alternative um rocket zu kompelieren ?

fn create_fahrzeug_list(query_result: Vec<postgres::row::Row>) -> Vec<Fahrzeug> {

    let mut vec_fahrzeuge : Vec<Fahrzeug> = Vec::new();

    for row in query_result {

        let fahrzeug = Fahrzeug {
            id: row.get(0),
            marke: row.get(1),
            model: row.get(2),
            leistung: row.get(3),
        };
        vec_fahrzeuge.push(fahrzeug);
    }
    return vec_fahrzeuge;
}



#[get("/getVehicle/<id>")]
fn get_vehicle(id: i32) -> Json<Vec<Fahrzeug>> {

    let mut vec_error : Vec<Fahrzeug> = Vec::new();
    let fahrzeug = Fahrzeug {
        id: 1,
        marke: String::from(""),
        model: String::from(""),
        leistung: 1,
    };
    vec_error.push(fahrzeug);

    let mut client = match Client::connect("host=database port=5432 user=postgres password=test", NoTls) {
        Ok(conn) => conn,
        Err(e) => {
            let error_json = format!("{{ 'ERROR':{} }}", e);
            return Json(vec_error);
        }
    };

    let query_result = match client.query("SELECT id, marke, model, leistung FROM fahrzeuge WHERE id=$1",  &[&id]) {
        Ok(res) => res,
        Err(e) => {
            let error_json = format!("{{ 'ERROR':{} }}", e);
            return Json(vec_error);
        }
    };

    return Json(create_fahrzeug_list(query_result));
}

fn main()  {
    rocket::ignite().mount("/", routes![get_vehicle]).launch();

}

#[derive(Serialize, Deserialize, Debug)]
struct Fahrzeug {
    id: i32,
    marke: String,
    model: String,
    leistung: i32,
}
