'use strict';
const express = require('express');
const bodyParser = require('body-parser');
const CircuitBreaker = require('./circuitBreaker.js');
const Auth = require("./auth.js")();
const Cache = require("./cache.js")
var jsonBodyParser = bodyParser.json({ type: 'application/json' });

// Constants
// Kann so festgelegt werden, da innerhalb des Containers festgelegt wird
// Nachaußen wird anders gemappt
const PORT = 8000;
const HOST = '0.0.0.0';

// Definition CircuitBreaker
var circuitBreakerBenutzerverwaltung = new CircuitBreaker(150, 30, 0,
    -3, 10, 3,
    process.env.BENUTZERVERWALTUNG, process.env.BENUTZERVERWALTUNGPORT);

const middlerwareWrapperAuth = (cache, isAdmin, circuitBreaker) => {
    return (req, res, next) => {
        Auth.checkAuth(req, res, isAdmin, cache, circuitBreaker, next);
    }
}

// Definition Cache um Nutzer Auth Token zwischen zu speichern
var cache = new Cache(10000, 5000);

var mysql = require('mysql');
var connection = mysql.createConnection({
    host     : process.env.MYSQL_HOST,
    port     : 3306,
    user     : 'root',
    password : process.env.MYSQL_ROOT_PASSWORD,
    database : 'fuhrpark'
});

connection.connect();

function checkParams(req, res, requiredParams) {
    console.log("checkParams", requiredParams);
    let paramsToReturn = {};

    for (let i = 0; i < requiredParams.length; i++) {
            let param = requiredParams[i];
            
        if (!(req.query && param in req.query)
            && !(req.body && param in req.body)
            && !(req.params && param in req.params)) {
            let error = "error parameter " + param + " is missing";
            console.log(error);
            throw error;
            return;
        }

        if (req.query && param in req.query) {
            paramsToReturn[param] = req.query[param];
        }
        if (req.body && param in req.body) {
            paramsToReturn[param] = req.body[param];
        }
        if (req.params && param in req.params) {
            paramsToReturn[param] = req.params[param];
        }
    }
    return  paramsToReturn;
}


const app = express();


app.get('/getVehicle/:id',[middlerwareWrapperAuth(cache, true, circuitBreakerBenutzerverwaltung)], async function (req, res) {
    try {
        let params = checkParams(req, res,["id"]);
        connection.query(`SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE id=${params.id} AND active=TRUE`, function (error, results) {
            if (error) {
                res.status(500).send(error);
            } else {
                res.status(200).send(results[0]);
            }
        });

    } catch(err){
        console.log(err);
        res.status(401).send(err);
    }
});

app.get('/getVehicles',[middlerwareWrapperAuth(cache, true, circuitBreakerBenutzerverwaltung)], async function (req, res) {
    try {
        connection.query(`SELECT id, marke, model, leistung, latitude, longitude FROM fahrzeuge WHERE active=TRUE`, function (error, results) {
            if (error) {
                res.status(500).send(error);
            } else {
                res.status(200).send(results);
            }
        });

    } catch(err){
        console.log(err);
        res.status(401).send(err);
    }
});

app.post('/addVehicle',[middlerwareWrapperAuth(cache, true, circuitBreakerBenutzerverwaltung), jsonBodyParser], async function (req, res) {
    try {
        let params = checkParams(req, res,["model", "leistung", "marke"]);
        var data  = {"marke": params.marke, "model": params.model, "leistung": params.leistung};
        connection.query('INSERT INTO fahrzeuge SET ?', data, function (error) {
            if (error) {
                res.status(500).send(error);
            } else {
                res.status(200).send("Fahrzeug wurde erfolgreich hinzugefügt");
            }

        });


    } catch(err){
        console.log(err);
        res.status(401).send(err);
    }
});

app.post('/updateVehicle',[middlerwareWrapperAuth(cache, true, circuitBreakerBenutzerverwaltung), jsonBodyParser], async function (req, res) {
    try {
        let params = checkParams(req, res,["id", "model", "leistung", "marke", "latitude", "longitude"]);
        var data  = {"marke": params.marke, "model": params.model, "leistung": params.leistung,
                     "latitude": params.latitude, "longitude": params.longitude};
        connection.query('UPDATE fahrzeuge SET ?', data, function (error) {
            if (error) {
                res.status(500).send(error);
            } else {
                res.status(200).send("Fahrzeug wurde erfolgreich aktualisiert");
            }

        });


    } catch(err){
        console.log(err);
        res.status(401).send(err);
    }
});

app.post('/inactiveVehicle/:id',[middlerwareWrapperAuth(cache, true, circuitBreakerBenutzerverwaltung)], async function (req, res) {
    try {
        let params = checkParams(req, res,["id"]);
        var data  = {"active": false};
        connection.query('UPDATE fahrzeuge SET ? WHERE id = ?', [data, params.id], function (error) {
            if (error) {
                res.status(500).send(error);
            } else {
                res.status(200).send("Fahrzeug wurde erfolgreich auf inaktiv gesetzt");
            }

        });


    } catch(err){
        console.log(err);
        res.status(401).send(err);
    }
});

app.listen(PORT, HOST, () => {
    console.log(`Running on http://${HOST}:${PORT}`);
});
