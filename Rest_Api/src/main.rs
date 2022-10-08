
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use postgres::{Client, NoTls};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

// TODO: Implement Error Handling
// TODO: Authentifizierung innerhalb der MS Architektur noch festlegen
// TODO: Rust nightly -> Wirklich empfehlenswert fuer production? -> gibt es auch eine Alternative um rocket zu kompelieren ?

// NOTE: i16 deshalb verwendet stat u16, da SQL Trait nicht mit u16 arbeitet

fn create_fahrzeug_list(query_result: Vec<postgres::row::Row>) -> Vec<Fahrzeug> {

    let mut vec_fahrzeuge : Vec<Fahrzeug> = Vec::new();

    for row in query_result {

        let fahrzeug = Fahrzeug {
            id: row.get(0),
            marke: row.get(1),
            model: row.get(2),
            leistung: row.get(3),
            plz: row.get(4),
        };
        vec_fahrzeuge.push(fahrzeug);
    }
    return vec_fahrzeuge;
}



#[get("/getVehicle/<plz>/<id>")]
fn get_vehicle(id: i32, plz: i32) -> Json<Vec<Fahrzeug>> {

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung, plz FROM fahrzeuge WHERE id=$1 AND plz=$2 AND active=TRUE",  &[&id, &plz]).unwrap();
    return Json(create_fahrzeug_list(query_result));

}

#[get("/getVehicles/<plz>")]
fn get_vehicles(plz: i32) -> Json<Vec<Fahrzeug>> {

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung, plz FROM fahrzeuge WHERE plz=$1 AND active=TRUE",  &[&plz]).unwrap();
    return Json(create_fahrzeug_list(query_result));

}

#[post("/updateVehicle", format = "json", data = "<fahrzeug_json>")]
fn update_vehicle(fahrzeug_json: Json<Fahrzeug>) -> String {

    let fahrzeug: Fahrzeug = fahrzeug_json.into_inner();

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();

    client.execute("UPDATE fahrzeuge SET marke = $1, model = $2, leistung = $3 , plz = $4 WHERE id = $5",
                   &[&fahrzeug.marke, &fahrzeug.model, &fahrzeug.leistung, &fahrzeug.plz ,&fahrzeug.id]).unwrap();

    return format!("Updated successfully vehicle with id: {}", fahrzeug.id);
}

#[post("/addVehicle", format = "json", data = "<fahrzeug_json>")]
fn add_vehicle(fahrzeug_json: Json<Fahrzeug>) -> Json<Vec<Fahrzeug>> {

    let fahrzeug: Fahrzeug = fahrzeug_json.into_inner();

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();

    client.execute("INSERT INTO fahrzeuge(marke, model, leistung, plz, active) VALUES ($1, $2, $3, $4, TRUE)",
                   &[&fahrzeug.marke, &fahrzeug.model, &fahrzeug.leistung, &fahrzeug.plz]).unwrap();

    let query_result = client.query("SELECT id, marke, model, leistung, plz FROM fahrzeuge WHERE plz = $1 ORDER BY id DESC LIMIT 1",
                                    &[&fahrzeug.plz]).unwrap();

    return Json(create_fahrzeug_list(query_result));
}

#[post("/inactiveVehicle/<id>")]
fn inactive_vehicle(id: i32) -> Json<Vec<Fahrzeug>> {

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung, plz FROM fahrzeuge WHERE id=$1",  &[&id]).unwrap();

    client.execute("UPDATE fahrzeuge SET active = FALSE WHERE id = $1", &[&id]).unwrap();

    return Json(create_fahrzeug_list(query_result));
}

fn main()  {
    rocket::ignite().mount("/", routes![get_vehicle, get_vehicles, update_vehicle, add_vehicle, inactive_vehicle]).launch();
}

#[derive(Serialize, Deserialize, Debug)]
struct Fahrzeug {
    id: i32,
    marke: String,
    model: String,
    leistung: i32,
    plz: i32
}