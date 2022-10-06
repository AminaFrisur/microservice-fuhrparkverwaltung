
#[macro_use] extern crate nickel;
use postgres::{Client, NoTls};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode;
use std::str::FromStr;
use itertools::Itertools;

// TODO: Fuege Standort noch hinzu
// Buchungsservices orientieren sich nach bestimmten Standort -> deshalb dies hinzufügen

// TODO: Authentifizierung innerhalb der MS Architektur noch festlegen

fn create_json(query_result: Vec<postgres::row::Row>) -> String {

    let mut vec_string : Vec<String> = Vec::new();

    for row in query_result {
        let id: i32 = row.get(0);
        let marke: &str = row.get(1);
        let model: &str = row.get(2);
        let leistung: i32 = row.get(3);

        let json_string = format!("{{'id':'{}', 'model':'{}', 'marke': '{}', 'leistung': '{}'}}", id, model, marke, leistung);
        vec_string.push(json_string);
    }


    let s: String = vec_string.iter().cloned().intersperse(format!(", ")).collect();
    let result: String = format!("{{'result':{{ {} }}", s);
    return result;
}

fn main()  {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/getVehicle/:id", middleware! { |request, mut response|

        let id : i32;

        // TODO: it is not necessary
        // if id is missing -> route not found !
        if request.param("id").is_none() {
            return response.send(format!("parameter id is missing"));
        } else {
            id = match FromStr::from_str(request.param("id").unwrap()) {
                Ok(res) => res,
                Err(e) => {
                     response.set(StatusCode::InternalServerError);
                     return response.send("ERROR: Parameter id must be an integer!");
                }

            }
        }

        let mut client = match Client::connect("host=database port=5432 user=postgres password=test", NoTls) {
            Ok(conn) => conn,
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                return response.send(format!("{}", e));
            }
        };

        let query_result = match client.query("SELECT id, marke, model, leistung FROM fahrzeuge WHERE id=$1",  &[&id]) {
            Ok(res) => res,
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                return response.send(format!("{}", e));
            }
        };

        response.set(MediaType::Json);

        return response.send(create_json(query_result));

    });

    router.get("/getVehicles", middleware! { |request, mut response|

        let mut client = match Client::connect("host=database port=5432 user=postgres password=test", NoTls) {
            Ok(conn) => conn,
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                return response.send(format!("{}", e));
            }
        };

        let query_result = match client.query("SELECT id, marke, model, leistung FROM fahrzeuge",  &[]) {
            Ok(res) => res,
            Err(e) => {
                response.set(StatusCode::InternalServerError);
                return response.send(format!("{}", e));
            }
        };

        response.set(MediaType::Json);

        return response.send(create_json(query_result));

    });

    router.post("/updateVehicle/:id", middleware! { |request, response|

        format!("Hello from POST /users/new")

    });

    router.delete("/deleteVehicle/:id", middleware! { |request, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("0.0.0.0:3030").unwrap();
}

