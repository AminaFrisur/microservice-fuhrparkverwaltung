
#[macro_use] extern crate nickel;
use mysql::*;
use mysql::prelude::*;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main() -> std::result::Result<(), Error>  {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    let url = "mysql://root:test@localhost:3306/fuhrpark";
    let pool = Pool::new(url)?;


    router.get("/getVehicle/:id", middleware! { |request, response|

        format!("Hello from GET /users")

    });

    router.get("/getVehicles", middleware! { |request, response|

        format!("Hello from POST /users/new")

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