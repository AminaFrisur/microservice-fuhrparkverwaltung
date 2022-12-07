'use strict';
const express = require('express');
const bodyParser = require('body-parser');
const Auth = require("./auth.js")();
var jsonBodyParser = bodyParser.json({ type: 'application/json' });
const JWT_SECRET = "goK!pusp6ThEdURUtRenOwUhAsWUCLheasfr43qrf43rttq3";

const PORT = 8000;
const HOST = '0.0.0.0';

const middlerwareCheckAuth = (isAdmin) => {
    return (req, res, next) => {
        Auth.checkAuth(req, res, isAdmin, JWT_SECRET,  next);
    }
}

var mysql = require('mysql');
var pool  = mysql.createPool({
    connectionLimit : 10000,
    host            : process.env.MYSQL_HOST,
    user            : 'root',
    password        : process.env.MYSQL_ROOT_PASSWORD,
    database        : 'fuhrpark'
});

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


app.get('/getVehicle/:id',[middlerwareCheckAuth(true)], async function (req, res) {
    try {
        let params = checkParams(req, res,["id"]);
        pool.query(`SELECT id, marke, model, leistung, kennzeichen, latitude, longitude FROM fahrzeuge WHERE id=${params.id} AND active=TRUE`, function (error, results) {
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

app.get('/getVehicles',[middlerwareCheckAuth(true)], async function (req, res) {
    try {
        pool.query(`SELECT id, marke, model, leistung, kennzeichen, latitude, longitude FROM fahrzeuge WHERE active=TRUE`, function (error, results) {
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

app.post('/addVehicle',[middlerwareCheckAuth(true), jsonBodyParser], async function (req, res) {
    try {
        let params = checkParams(req, res,["model", "leistung", "marke", "kennzeichen"]);
        var data  = {"marke": params.marke, "model": params.model, "leistung": params.leistung, "kennzeichen": params.kennzeichen};
        pool.query('INSERT INTO fahrzeuge SET ?', data, function (error) {
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

app.post('/updateVehicle',[middlerwareCheckAuth(true), jsonBodyParser], async function (req, res) {
    try {
        let params = checkParams(req, res,["id", "model", "leistung", "marke", "latitude", "longitude", "kennzeichen"]);
        var data  = {"marke": params.marke, "model": params.model, "leistung": params.leistung,
                     "latitude": params.latitude, "longitude": params.longitude, "kennzeichen": params.kennzeichen};
        pool.query('UPDATE fahrzeuge SET ?', data, function (error) {
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

app.post('/inactiveVehicle/:id',[middlerwareCheckAuth(true)], async function (req, res) {
    try {
        let params = checkParams(req, res,["id"]);
        var data  = {"active": false};
        pool.query('UPDATE fahrzeuge SET ? WHERE id = ?', [data, params.id], function (error) {
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
