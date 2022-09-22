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