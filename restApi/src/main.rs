use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
pub use mysql_async::prelude::*;
pub use mysql_async::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use serde::{Deserialize, Serialize};
extern crate regex;
use regex::Regex;
mod auth;

fn get_url_db() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
        if opts
            .db_name()
            .expect("a database name is required")
            .is_empty()
        {
            panic!("database name is empty");
        }
        println!("{ }", url);

        url
    } else {
        panic!("Datebase URL is wrong!");
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

async fn handle_request_wrapper( req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {
    match handle_request(req, pool).await {
        Ok(result) => Ok(result),
        Err(err) => {
            let error_message = format!("{:?}", err);
            Ok(response_build(&error_message, 500))

        }
    }
}

async fn handle_request(req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {

    let mut login_name ="";
    let mut auth_token ="";
    let JWT_SECRET : String = "goK!pusp6ThEdURUtRenOwUhAsWUCLheasfr43qrf43rttq3".to_string();

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

    let addr_with_params = format!("/checkAuthUser?login_name={}&auth_token={}&isAdmin=true", login_name, auth_token);
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

            match auth::check_auth_user(login_name, auth_token, false, JWT_SECRET).await {
                Ok(()) => println!("Rest API: Nutzer ist authentifiziert"),
                Err(err) => return Ok(response_build(&format!("Authentifizierung fehlgeschlagen: {}", err), 401)),
            }

            // get Params from url
            // nutze dafür das Ergebnis aus dem Regulären Ausdruck
            let id: String = regex_route.chars().filter(|c| c.is_digit(10)).collect();
            println!("REST API getVehicle: START CALL");

            let mut conn = match  pool.get_conn().await {
              Ok(result) => result,
              Err(_) => return Ok(response_build("Verbindung zur Datenbank ist fehlgeschlagen", 500))
            };

            let id: i32 = id.parse()?;

            let statement = format!("SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE id={} AND active=TRUE", id);

            let fahrzeug = match statement.with(()).map(&mut conn, |(id, marke, model, leistung, latitude, longitude)| { Fahrzeug::new(id, marke, model, leistung, latitude, longitude) }, ).await {
                Ok(result) => result,
                Err(_) => return Ok(response_build("SQL Statement ist fehlgeschlagen!", 500))
            };

            println!("REST API: Ergebnis ist: {} ", serde_json::to_string(&fahrzeug)?.as_str());

            drop(conn);
            Ok(response_build(serde_json::to_string(&fahrzeug)?.as_str(), 200))
        }

        (&Method::GET, "/getVehicles") => {
            println!("REST API getVehicles: START CALL");

            match auth::check_auth_user(login_name, auth_token, false, JWT_SECRET).await {
                Ok(()) => println!("Rest API: Nutzer ist authentifiziert"),
                Err(err) => return Ok(response_build(&format!("Authentifizierung fehlgeschlagen: {}", err), 401)),
            }

            let mut conn = match  pool.get_conn().await {
                Ok(result) => result,
                Err(_) => return Ok(response_build("Verbindung zur Datenbank ist fehlgeschlagen", 500))
            };

            let statement = "SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE active=TRUE";
            let fahrzeug = match statement.with(()).map(&mut conn, |(id, marke, model, leistung, latitude, longitude)| { Fahrzeug::new(id, marke, model, leistung, latitude, longitude) }, ).await {
                Ok(result) => result,
                Err(_) => return Ok(response_build("SQL Statement ist fehlgeschlagen!", 500))
            };
            println!("REST API: Ergebnis ist: {} ", serde_json::to_string(&fahrzeug)?.as_str());

            drop(conn);
            Ok(response_build(serde_json::to_string(&fahrzeug)?.as_str(), 200))
        }

        (&Method::POST, "/addVehicle") => {

            match auth::check_auth_user(login_name, auth_token, false, JWT_SECRET).await {
                Ok(()) => println!("Rest API: Nutzer ist authentifiziert"),
                Err(err) => return Ok(response_build(&format!("Authentifizierung fehlgeschlagen: {}", err), 401)),
            }

            println!("REST API addVehicle: START CALL");
            let mut conn = match  pool.get_conn().await {
                Ok(result) => result,
                Err(_) => return Ok(response_build("Verbindung zur Datenbank ist fehlgeschlagen", 500))
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
                Err(_) => return Ok(response_build("SQL Statement ist fehlgeschlagen!",500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde erfolgreich hinzugefügt", 200))
        }

        (&Method::POST, "/updateVehicle") => {

            match auth::check_auth_user(login_name, auth_token, false, JWT_SECRET).await {
                Ok(()) => println!("Rest API: Nutzer ist authentifiziert"),
                Err(err) => return Ok(response_build(&format!("Authentifizierung fehlgeschlagen: {}", err), 401)),
            }

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
                Err(_) => return Ok(response_build("SQL Statement ist fehlgeschlagen!", 500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde erfolgreich aktualisiert", 200))
        }

        (&Method::POST, "/inactiveVehicle/") => {

            match auth::check_auth_user(login_name, auth_token, false, JWT_SECRET).await {
                Ok(()) => println!("Rest API: Nutzer ist authentifiziert"),
                Err(err) => return Ok(response_build(&format!("Authentifizierung fehlgeschlagen: {}", err), 401)),
            }

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
                Err(_) => return Ok(response_build("SQL Statement ist fehlgeschlagen!", 500))
            }

            drop(conn);
            Ok(response_build("Fahrzeug wurde auf inaktiv gesetzt", 200))
        }


        _ => {
            println!("REST API: ROUTE NOT FOUND");
            Ok(response_build("Route not found", 404))
        }
    }
}

fn response_build(body: &str, status: u16) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opts = Opts::from_url(&*get_url_db()).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    // The connection pool will have a min of 5 and max of 10 connections.
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let make_svc = make_service_fn(|_| {
        let pool = pool.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                handle_request_wrapper( req, pool)
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


