use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
pub use mysql_async::prelude::*;
pub use mysql_async::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use serde::{Deserialize, Serialize};
extern crate regex;
use regex::Regex;
mod circuitbreaker;
mod authclient;
use crate::circuitbreaker::CircuitBreaker;

fn get_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
        if opts
            .db_name()
            .expect("a database name is required")
            .is_empty()
        {
            panic!("database name is empty");
        }
        url
    } else {
        "mysql://root:test@127.0.0.1:3000/fuhrpark".into()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Fahrzeug {
    id: i32,
    marke: String,
    model: String,
    leistung: i32,
    latitude: i32,
    longitude: i32
}

impl Fahrzeug {
    fn new(
        id: i32,
        marke: String,
        model: String,
        leistung: i32,
        latitude: i32,
        longitude: i32
    ) -> Self {
        Self {
            id,
            marke,
            model,
            leistung,
            latitude,
            longitude
        }
    }
}

pub fn regex_route(re: Regex, route: &str) -> String {
    if re.is_match(route) {
        let cap = re.captures(route).unwrap();
        return (&cap[0]).to_string();
    } else {
        return "/error".to_string();
    }
}

async fn handle_request_wrapper(circuit_breaker: CircuitBreaker<'_>, req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {
    match handle_request(circuit_breaker, req, pool).await {
        Ok(result) => Ok(result),
        Err(err) => {
            let error_message = format!("{:?}", err);
            Ok(response_build_error(&error_message, 500))

        }
    }
}

async fn handle_request(mut circuit_breaker: CircuitBreaker<'_>, req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {

    let mut login_name ="";
    let mut auth_token ="";

    // get Header Information for login_name and auth_token
    for (key, value) in req.headers().iter() {
        if key == "login_name" {
            login_name = value.to_str()?;
            // login_name = &value;
            println!("REST API login_name found {:?}", login_name);

        }
        if key == "auth_token" {
            auth_token = value.to_str()?;
            // auth_token = &value;
            println!("REST API auth_token found {:?}", auth_token);

        }
    }

    // Definiere hier zusätlich welche Routen erlaubt sind
    // Wichtig um auch zu checken ob Parameter in der URL dabei sind
    let re = Regex::new(r"/getVehicle/\d+|/echo|/getVehicles|/updateVehicle|/addVehicle|/inactiveVehicle/\d+")?;
    let regex_route = regex_route(re, req.uri().path());
    let filtered_route: String = regex_route.chars().filter(|c| !c.is_digit(10)).collect();

    match (req.method(),  filtered_route.as_str()) {
        // Prüfe ob Service Verfügbar ist
        // Könnte ich theoretisch für das Monitioring nutzen
        (&Method::GET, "/echo") => Ok(Response::new(req.into_body())),

        (&Method::GET, "/getVehicle/") => {
            let addr_with_params = format!("/checkAuthUser?login_name={}&auth_token={}&isAdmin=true", login_name, auth_token);

            match circuit_breaker.circuit_breaker_post_request(addr_with_params).await {
                Ok(message) => println!("Rest API: {}", message),
                Err(err) => return Ok(response_build_error(&format!("{}", err), 401)),
            }

            // get Params from url
            // nutze dafür das Ergebnis aus dem Regulären Ausdruck
            let id: String = regex_route.chars().filter(|c| c.is_digit(10)).collect();
            println!("REST API getVehicle: START CALL");

            let mut conn = match  pool.get_conn().await {
              Ok(result) => result,
              Err(_) => return Ok(response_build_error("Verbindung zur Datenbank ist fehlgeschlagen", 500))
            };

            let id: i32 = id.parse()?;

            let statement = format!("SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE id={} AND active=TRUE", id);

            let fahrzeug = match statement.with(()).map(&mut conn, |(id, marke, model, leistung, latitude, longitude)| { Fahrzeug::new(id, marke, model, leistung, latitude, longitude) }, ).await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("SQL Statement ist fehlgeschlagen!", 500))
            };

            println!("REST API: Ergebnis ist: {} ", serde_json::to_string(&fahrzeug)?.as_str());

            drop(conn);
            Ok(response_build(serde_json::to_string(&fahrzeug)?.as_str()))
        }

        (&Method::GET, "/getVehicles") => {
            println!("REST API getVehicle: START CALL");

            let mut conn = match  pool.get_conn().await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("Verbindung zur Datenbank ist fehlgeschlagen", 500))
            };

            let statement = "SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE active=TRUE";
            let fahrzeug = match statement.with(()).map(&mut conn, |(id, marke, model, leistung, latitude, longitude)| { Fahrzeug::new(id, marke, model, leistung, latitude, longitude) }, ).await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("SQL Statement ist fehlgeschlagen!", 500))
            };
            println!("REST API: Ergebnis ist: {} ", serde_json::to_string(&fahrzeug)?.as_str());

            drop(conn);
            Ok(response_build(serde_json::to_string(&fahrzeug)?.as_str()))
        }

        (&Method::POST, "/addVehicle") => {

            println!("REST API addVehicle: START CALL");
            let mut conn = match  pool.get_conn().await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("Verbindung zur Datenbank ist fehlgeschlagen", 500))
            };

            let byte_stream = hyper::body::to_bytes(req).await?;
            let fahrzeug: Fahrzeug = serde_json::from_slice(&byte_stream)?;

            match "INSERT INTO fahrzeuge (marke, model, leistung, latitude, longitude) VALUES (:marke, :model, :leistung, latitude, longitude)"
                .with(params! {
                    "marke" => fahrzeug.marke,
                    "model" => fahrzeug.model,
                    "leistung" => fahrzeug.leistung,
                    "latitude" => fahrzeug.latitude,
                    "longitude" => fahrzeug.longitude

                })
                .ignore(&mut conn)
                .await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("SQL Statement ist fehlgeschlagen!",500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde erfolgreich hinzugefügt"))
        }

        (&Method::POST, "/updateVehicle") => {
            let mut conn = pool.get_conn().await?;
            println!("REST API updateVehicle: START CALL");
            let byte_stream = hyper::body::to_bytes(req).await?;
            let fahrzeug: Fahrzeug = serde_json::from_slice(&byte_stream)?;

            match "UPDATE fahrzeuge SET marke=:marke, model=:model, leistung=:leistung, latitude=:latitude, longitude=:longitude WHERE id=:id"
                .with(params! {
                    "id" => fahrzeug.id,
                    "marke" => fahrzeug.marke,
                    "model" => fahrzeug.model,
                    "leistung" => fahrzeug.leistung,
                    "latitude" => fahrzeug.latitude,
                    "longitude" => fahrzeug.longitude,
                })
                .ignore(&mut conn)
                .await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("SQL Statement ist fehlgeschlagen!", 500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde erfolgreich aktualisiert"))
        }

        (&Method::POST, "/inactiveVehicle/") => {
            let id: String = regex_route.chars().filter(|c| c.is_digit(10)).collect();
            let mut conn = pool.get_conn().await?;
            println!("REST API inactiveVehicle: START CALL");

            match "UPDATE fahrzeuge SET active=FALSE WHERE id=:id"
                .with(params! {
                    "id" => id
                })
                .ignore(&mut conn)
                .await {
                Ok(result) => result,
                Err(_) => return Ok(response_build_error("SQL Statement ist fehlgeschlagen!", 500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde auf inaktiv gesetzt"))
        }


        _ => {
            println!("REST API: ROUTE NOT FOUND");
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// TODO: Prüfe ob wirklich gebraucht wird
// CORS headers
fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

fn response_build_error(body: &str, status: u16) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = Opts::from_url(&*get_url()).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    // The connection pool will have a min of 5 and max of 10 connections.
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));

    // circuit Breaker:
    let circuit_breaker_benutzerverwaltung = CircuitBreaker::new(150, 30, 0, -3, 10, 3, "0.0.0.0", 8000);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8002));
    let make_svc = make_service_fn(|_| {
        let pool = pool.clone();
        let circuit_breaker_benutzerverwaltung = circuit_breaker_benutzerverwaltung.clone();
        // move converts any variables captured by reference or mutable reference to variables captured by value.
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                let circuit_breaker_benutzerverwaltung = circuit_breaker_benutzerverwaltung.clone();
                handle_request_wrapper(circuit_breaker_benutzerverwaltung, req, pool)
            }))
        }
    });
    println!("REST API: Start Server");
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}


