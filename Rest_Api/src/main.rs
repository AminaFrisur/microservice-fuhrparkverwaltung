
#[macro_use] extern crate nickel;
use postgres::{Client, NoTls};

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main()  {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    println!("host=database port=5432 user=postgres password=test");
    // TODO: Kein Unwrap verwenden -> Hier muss eine Fehlerbehandlung durchgeführt werden
    // Möglichkeiten der Fehlerbehandlung:
    // unwrap: ruft bei Fehler direkt panic auf -> Programm wird direkt gestoppt
    // .unwrap_or -> Wenn Fehler dann setzte einen Default Wert
    // an die caller Funktion zurückgeben: -> Result<String, Error> als Beispiel
    // verwenden von match operator: match test() { Ok(Wert) => mache das , Err(e) => mache das
    // ? Operator: dieser ist ähnlich zu unwrap, es leitet aber den Fehler an die Caller Funktion weiter statt direkt panic! aufzurufen


    router.get("/getVehicle/:id", middleware! { |request, response|

        let mut client = Client::connect("host=database port=5432 user=postgres password=test", NoTls).unwrap();
        let queryResult = client.query("SELECT id, marke, model, leistung FROM fahrzeuge", &[]);

        let result = match queryResult {
            Ok(res) => res,
            Err(err) => Vec::new(),
        };

        if result.len() == 0 {
            println!("Select Statement not working");
        } else {
            println!("Great Success!")
        }

        format!("BEEP");

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