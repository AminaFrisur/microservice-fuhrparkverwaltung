
#[macro_use] extern crate nickel;
use mysql::*;
use mysql::prelude::*;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main() -> std::result::Result<(), Error>  {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    let url = "mysql://root:test@localhost:3307/fuhrpark";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;


    router.get("/getVehicle/:id", middleware! { |request, response|

        format!("Hello from GET /users")

    });

    router.get("/getVehicles", middleware! { |request, response|

        format!("Hello from POST /users/new")

    });

    router.post("/addVehicle", middleware! { |request, response|

        let mut fahrzeuge = vec![
            Fahrzeug { model: String::from("UP"), marke: String::from("VW"), leistung: String::from("75"), id: 0 },
            ];

       conn.exec_batch(
        r"INSERT INTO fahrzeug (model, marke, leistung)
          VALUES (:model, :marke, :leistung)",
            fahrzeuge.iter().map(|p| params! {
                "model" => p.model,
                "marke" => p.marke,
                "leistung" => p.leistung,
            })
        )?;


        format!("Inserted new dataset")

    });

    router.post("/updateVehicle/:id", middleware! { |request, response|

        format!("Hello from POST /users/new")

    });

    router.delete("/deleteVehicle/:id", middleware! { |request, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("0.0.0.0:3030").unwrap();

    Ok(())
}

struct Fahrzeug {
    model: String,
    marke: String,
    id: i32,
    leistung: String
}