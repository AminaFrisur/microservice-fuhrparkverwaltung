# microservice-fuhrparkverwaltung
Fuhrparkverwaltung mit Wasm

Ordnerstruktur:

Logik:
Hier befindet sich die Geschäftslogik für den Microservice Booking.
Dieser ist komplett in Wasm definiert. Zugrunde liegt die Laufzeitumgebung Wasmedge.
Die Geschäftslogik läuft komplett eigenständig.
Für die Umsetzung der REST API wird wasmEdge Lib benutzt. Diese nimmt jegliche HTTP Anfragen.
Was aber fehlt:
- Weiterleitung nach dem Path im HTTP Header -> Unterscheidung zwischen GET, POST, DELETE, PUT, DELETE, PATCH

Das muss selber entwickelt werden
Datenbank:
MySQL Datenbank auf die die Geschäftslogik zugreift.


Anmerkungen:
WICHTIG: Aktuell unterstützt WasmEdge kein Threading !
Somit probleme bei gleichzeitigen Zugriff.

https://github.com/WasmEdge/WasmEdge/issues/1467
https://webassembly.org/roadmap/ -> Node.js beispielsweise aber schon

TODOS:
- Erstelle Config Datei für Serveradressen

- IDEE: Statt Fuhrparkverwaltung mit Business Logik etc. zu lösen -> 
- CQRS Pattern mit REST API entwickeln und feierabend
- und diese REST API dann mit Wasm eventuell ? 

Zu CQRS:
- cqrs-es = "0.4.2" -> nicht mit Wasm kompelierbar !

Weiteres in Bezug auf Datenbanken über HTTP:
- MongoDB: nicht möglich 
- Mysql: ebenfalls nicht möglich 
- PostgresSQL: Keine Ahnung

Neue Idee: Erstelle ein Interface in einer anderen Programmiersprache das eine Rest Schnittstelle zur DB aufbaut

Stand 22.09:
- Es ist aktuell nicht unbedingt möglich über HTTP auf die Datenbank generell zuzugreifen
- nicht alle Datenbanksysteme bieten dies an 
- Somit neue Umsetzung
- Dieser Microservice wird nicht mehr als WASM Microservice umgesetzt
- Dieser besteht nun aus einem Docker Container der eine MySQL Datenbank und eine eigene REST API enthält aus Rust
- Aber ohne dies in WASM zu übersetzen

WASM Rust erstellen:
- wasmedge ./target/wasm32-wasi/release/microservice_fuhrparkverwaltung.wasm
- cargo build --target wasm32-wasi --release

https://serverfault.com/questions/1084915/still-confused-why-docker-works-when-you-make-a-process-listen-to-0-0-0-0-but-no


Stand 2.10.2022:
- Dadurch das Docker eine Prozessorientierte Virtualisierungstechnologie ist, muss bzw. sollte für jeden Prozess ein eigener Container erstellt werden
- Deshalb jeweils ein Container für die REST API die auf die Datenbank zugreift
- ein Container für die mysql Datenbank an sich

Einzelnes erstellen des Datenbank Containers:
- docker build -t database_fuhrpark_ms .
- docker run -d --name database_fuhrpark_ms_v4 --env MYSQL_ROOT_PASSWORD= database_fuhrpark_ms

Einzelnes erstellen der Rest API:


Stand 4.10.2022:
docker run -d -p 3030:3030 --network test  rest_api_fuhrpark_ms
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=test --network test --network-alias database  datenbank_postgres_fuhrpark_ms

Mit dem Network Alias funktioniert es 

Stand 6.10.2022:
- Möglichkeiten der Fehlerbehandlung:
- unwrap: ruft bei Fehler direkt panic auf -> Programm wird direkt gestoppt
- .unwrap_or -> Wenn Fehler dann setzte einen Default Wert
- an die caller Funktion zurückgeben: -> Result<String, Error> als Beispiel
- verwenden von match operator: match test() { Ok(Wert) => mache das , Err(e) => mache das
- ? Operator: dieser ist ähnlich zu unwrap, es leitet aber den Fehler an die Caller Funktion weiter statt direkt panic! aufzurufen




