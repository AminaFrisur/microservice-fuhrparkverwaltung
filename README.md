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

