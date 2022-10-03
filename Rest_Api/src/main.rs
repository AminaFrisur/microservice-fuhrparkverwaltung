
#[macro_use] extern crate nickel;
use postgres::{Client, NoTls};

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main()  {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    println!("host=database port=5432 user=postgres password=test");
    let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();


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
}

struct Fahrzeug {
    model: String,
    marke: String,
    id: i32,
    leistung: String
}