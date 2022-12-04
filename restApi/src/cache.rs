use chrono::{DateTime, Utc};
use anyhow::anyhow;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct User {
    login_name: String,
    auth_token: String,
    auth_token_timestamp: String,
    cache_timestamp: String
}

impl User {
    pub fn new(login_name: &str, auth_token: &str, auth_token_timestamp: DateTime<Utc>, cache_timestamp: DateTime<Utc>) -> Self {

        return Self {login_name: login_name.to_string(), auth_token: auth_token.to_string(), auth_token_timestamp: auth_token_timestamp.to_rfc3339(), cache_timestamp: cache_timestamp.to_rfc3339()};
    }

    pub fn print_login_name(&self) {
        println!("CACHE LOGIN NAME: {}", self.login_name);
    }

    pub fn print_auth_token(&self) {
        println!("CACHE AUTH TOKEN: {}", self.auth_token);
    }

    pub fn print_auth_token_timestamp(&self) {
        println!("CACHE AUTH TOKEN TIMESTAMP: {}", self.auth_token_timestamp);
    }

}


#[derive(Clone)]
pub struct Cache {
    cached_user: Arc<std::sync::Mutex<Vec<User>>> ,
    max_size: i64,
    cache_time: i64
}
impl Cache   {
    pub fn new(max_size: i64, cache_time: i64) -> Self {

        return Self {cached_user: Arc::new(Mutex::new(Vec::new())), max_size, cache_time};
    }

    fn clear_cache(& mut self) -> Result<(), anyhow::Error> {
        println!("Cache: Prüfe ob Einträge aus dem Cache gelöscht werden können");
        let mut cached_user = self.cached_user.lock().unwrap();

        if cached_user.len() > self.max_size.try_into().unwrap() {
            // kompletter reset des caches
            // sollte aber eigentlich nicht passieren
            *cached_user =  Vec::new();
            return Ok(());
        }

        let mut temp_index = cached_user.len();
        let mut check = true;
        let current_timestamp = Utc::now();

        while check {
            temp_index = temp_index / 2;
            println!("Cache: TempIndex ist {}", temp_index);
            // Falls im Cache nur ein Element ist
            if temp_index >= 1 {

                let cached_user_timestamp = DateTime::parse_from_rfc3339(&cached_user[temp_index - 1].cache_timestamp)?.with_timezone(&Utc);

                let time_diff = current_timestamp.signed_duration_since(cached_user_timestamp).num_seconds();

                println!("Cache: Zeit Differenz zwsichen Aktueller Zeit und Cachetime beträgt {} Sekunden", time_diff - self.cache_time);
                // Wenn für den Eintrag die Cache Time erreicht ist -> lösche die hälfte vom Part des Arrays was betrachtet wird
                // Damit sind dann nicht alle alten Cache einträge gelöscht -> aber das clearen vom Cache sollte schnell gehen
                if time_diff >= self.cache_time {
                    println!("Cache: Clear Cache");
                    *cached_user = cached_user[temp_index..].to_vec();
                    check = false;
                }

                // Wenn timeDiff noch stimmt dann mache weiter

            } else {

                // auch wenn das eine Element im Array ein alter Eintrag ist
                // kann dies vernachlässigt werden bzw. ist nicht so wichtig
                println!("Cache: nichts zu clearen");
                check = false;
            }
        }

        Ok(())
    }

    pub fn get_user_index(& mut self, login_name: &str) -> Result<usize, anyhow::Error> {
        self.clear_cache()?;
        let mut final_index: usize = 0;
        let mut user_found: bool = false;
        let mut cached_user = self.cached_user.lock().unwrap();

        for i in 0..(cached_user.len()) {
            println!("{}", cached_user[i].login_name);
            if cached_user[i].login_name == login_name {
                final_index = i;
                // Auch beim Suchen eines Users -> Timestamp für Cache Eintrag aktualisieren
                println!("Cache: Update Timestamp vom Cache Eintrag");
                cached_user[i].cache_timestamp = Utc::now().to_rfc3339();
                user_found = true;
                break;
            }
        }

        if user_found {
            println!("Cache: User Index ist: {} ", final_index);
            Ok(final_index)
        } else {
            Err(anyhow!("Cache: User wurde im Cache nicht gefunden"))
        }

    }

    pub fn update_or_insert_cached_user(&self, user_found: bool, index: usize, login_name: &str, auth_token: &str, auth_token_timestamp: &str) -> Result<(), anyhow::Error> {
        let mut cached_user = self.cached_user.lock().unwrap();

        let user = User::new(login_name, auth_token, DateTime::parse_from_rfc3339(auth_token_timestamp)?.with_timezone(&Utc), Utc::now());

        if user_found  {
            println!("Cache: mache ein Update zum User");
            cached_user.remove(index);
        }
        // Füge User neu im Cache hinzu, da nicht im cache vorhanden
        println!("Cache: Füge Eintrag neu in Cache hinzu");
        cached_user.push(user);

        Ok(())
    }

    pub fn check_token(&self, index: usize, auth_token: &str) -> Result<bool, anyhow::Error> {

        let cached_user = self.cached_user.lock().unwrap();
        if auth_token != cached_user[index].auth_token { println!("Cache: Token aus dem Header stimmt nicht mit dem Token aus dem cache überein"); return Ok(false)};
        let current_timestamp = Utc::now();
        let auth_token_timestamp =  DateTime::parse_from_rfc3339(&cached_user[index].auth_token_timestamp)?.with_timezone(&Utc);
        println!("Cache: Auth Token Timestamp ist  {}", auth_token_timestamp);
        let time_diff = current_timestamp.signed_duration_since(auth_token_timestamp).num_hours();
        println!("Cache: Differenz zwischen aktueller Zeit und Auth Token Timestamp beträgt: {} Stunden", time_diff);
        // Wenn token älter ist als 24 Stunden
        if time_diff > 24  {
            return Ok(false);
        }
        Ok(true)

    }

}


