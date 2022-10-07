
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use postgres::{Client, NoTls};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

// TODO: Implement Error Handling

// TODO: Fuege Standort noch hinzu -> Buchungsservices orientieren sich nach bestimmten Standort -> deshalb dies hinzufügen

// TODO: Authentifizierung innerhalb der MS Architektur noch festlegen

// TODO: Rust nightly -> Wirklich empfehlenswert fuer production? -> gibt es auch eine Alternative um rocket zu kompelieren ?

// TODO: Enscheiden ob Fahrzeug wirklich gelöscht werden soll oder doch lieber einfach auf deleted = 1 gesetzt -> da problem später bei Rechnungen

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

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung FROM fahrzeuge WHERE id=$1",  &[&id]).unwrap();
    return Json(create_fahrzeug_list(query_result));

}

#[get("/getVehicles")]
fn get_vehicles() -> Json<Vec<Fahrzeug>> {

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung FROM fahrzeuge",  &[]).unwrap();
    return Json(create_fahrzeug_list(query_result));

}

#[post("/updateVehicle", format = "json", data = "<fahrzeug_json>")]
fn update_vehicle(fahrzeug_json: Json<Fahrzeug>) -> String {

    let fahrzeug: Fahrzeug = fahrzeug_json.into_inner();

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();

    client.execute("UPDATE fahrzeuge SET marke = $1, model = $2, leistung = $3 WHERE id = $4", &[&fahrzeug.marke, &fahrzeug.model, &fahrzeug.leistung, &fahrzeug.id]).unwrap();

    return format!("Updated successfully vehicle with id: {}", fahrzeug.id);
}

#[post("/addVehicle", format = "json", data = "<fahrzeug_json>")]
fn add_vehicle(fahrzeug_json: Json<Fahrzeug>) -> Json<Vec<Fahrzeug>> {

    let fahrzeug: Fahrzeug = fahrzeug_json.into_inner();

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();

    client.execute("INSERT INTO fahrzeuge(marke, model, leistung) VALUES ($1, $2, $3)", &[&fahrzeug.marke, &fahrzeug.model, &fahrzeug.leistung]).unwrap();

    let query_result = client.query("SELECT id, marke, model, leistung FROM fahrzeuge ORDER BY id DESC LIMIT 1",  &[]).unwrap();

    return Json(create_fahrzeug_list(query_result));
}

#[delete("/deleteVehicle/<id>")]
fn delete_vehicle(id: i32) -> Json<Vec<Fahrzeug>> {

    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
    let query_result = client.query("SELECT id, marke, model, leistung FROM fahrzeuge WHERE id=$1",  &[&id]).unwrap();

    client.execute("DELETE FROM fahrzeuge WHERE id = $1", &[&id]).unwrap();

    return Json(create_fahrzeug_list(query_result));
}

fn main()  {
    rocket::ignite().mount("/", routes![get_vehicle, get_vehicles, update_vehicle, add_vehicle, delete_vehicle]).launch();
}

#[derive(Serialize, Deserialize, Debug)]
struct Fahrzeug {
    id: i32,
    marke: String,
    model: String,
    leistung: i32,
}