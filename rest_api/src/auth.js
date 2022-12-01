module.exports = function() {
    var module = {};
     module.checkAuth = async function(req, res, isAdmin, cache, circuitBreaker, next) {
        let authToken = req.headers.auth_token;
        let loginName = req.headers.login_name;

        // Schritt 1: Schaue ob der User im Cache ist
         //Hier aufgerufen um nur einmal getUserIndex aufzurufen
        let userIndexCache = cache.getUserIndex(loginName);

        // Nutzer lässt sich nicht im Cache finden
        // Frage nun bei der Benutzerverwaltung nach
        if(userIndexCache == - 1 ) {
            let bodyData = {"login_name":loginName, "auth_token": authToken, "isAdmin": isAdmin};
            let headerData = { 'Content-Type': 'application/json'};
            console.log(bodyData);
            try {
                let loginData = await circuitBreaker.circuitBreakerPostRequest( "/checkAuthUser", bodyData, headerData);
                console.log("Authentification: Request checkAuthUser ergab folgendes Ergebnis: " + loginData);
                if(loginData) {
                    cache.updateOrInsertCachedUser(userIndexCache, loginName, loginData[0].auth_token, loginData[0].auth_token_timestamp, loginData[0].is_admin);
                    next();
                } else {
                    console.log("Authentification: Token ist laut Benutzerverwaltung nicht valide");
                    res.status(401).send("token and/or login name are missing or are not valid");
                }
            } catch(e) {
                console.log("Authentification: Reqeust schlug fehl ->" + e);
                res.status(500).send("Request zur Benutzerverwaltung schlug fehl!!");
            }


        } else {
            // Nutzer ist im Cache
            // Prüfe ob Token valide ist (Gleicheit, Zugriffsrecht, Alter des Token
            let check = cache.checkToken(userIndexCache, authToken, isAdmin);
            if(check) {
                console.log("Authentifizierungstoken ist valide");
                next();
            } else {
                res.status(401).send("token and/or login name are missing or are not valid");
            }

        }
    }
    return module;
}
