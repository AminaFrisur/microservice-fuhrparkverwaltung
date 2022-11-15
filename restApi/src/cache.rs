use chrono::{DateTime, Utc};
use anyhow::anyhow;
use std::sync::{Arc, Mutex};

// Rust
// struct + impl = "class"
#[derive(Clone)]
pub struct User<'a> {
    login_name: &'a str,
    auth_token: &'a str,
    auth_token_timestamp: DateTime<Utc>,
    cache_timestamp: DateTime<Utc>,
}

impl <'a> User<'a>  {
    pub fn new(login_name: &'a str, auth_token: &'a str, auth_token_timestamp: DateTime<Utc>, cache_timestamp: DateTime<Utc>) -> Self {

        return Self {login_name, auth_token, auth_token_timestamp, cache_timestamp};
    }

}

#[derive(Clone)]
pub struct Cache<'a> {
    cached_user: Arc<std::sync::Mutex<Vec<User<'a>>>> ,
    max_size: i64,
    cache_time: i64,
    timestamp:  Arc<std::sync::Mutex<DateTime<Utc>>>,
}
impl <'a> Cache<'a>   {
    pub fn new(max_size: i64, cache_time: i64) -> Self {

        return Self {cached_user: Arc::new(Mutex::new(Vec::new())), max_size, cache_time, timestamp: Arc::new(Mutex::new(Utc::now()))};
    }

    fn clear_cache(& mut self) {
        println!("Cache: Prüfe ob Einträge aus dem Cache gelöscht werden können");
        let mut cached_user = self.cached_user.lock().unwrap();

        if cached_user.len() > self.max_size.try_into().unwrap() {
            // kompletter reset des caches
            // sollte aber eigentlich nicht passieren
            *cached_user =  Vec::new();
            return;
        }

        let mut temp_index = cached_user.len();
        let mut check = true;
        let current_timestamp = Utc::now();

        while check {
            temp_index = temp_index / 2;
            println!("Cache: TempIndex ist {}", temp_index);
            // Falls im Cache nur ein Element ist
            if temp_index >= 1 {

                let time_diff = current_timestamp.signed_duration_since(cached_user[temp_index - 1].cache_timestamp).num_seconds();

                println!("Cache: {}", time_diff - self.cache_time);
                // Wenn für den Eintrag die Cache Time erreicht ist -> lösche die hälfte vom Part des Arrays was betrachtet wird
                // Damit sind dann nicht alle alten Cache einträge gelöscht -> aber das clearen vom Cache sollte schnell gehen
                if time_diff >= self.cache_time {
                    println!("Cache: Clear Cache");
                    *cached_user =  cached_user[temp_index..].to_vec();
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
    }

    fn get_user_index(& mut self, login_name: &str) -> Result<usize, anyhow::Error> {
        self.clear_cache();
        let mut final_index: usize = 0;
        let mut user_found: bool = false;
        let mut cached_user = self.cached_user.lock().unwrap();

        for i in 0..(cached_user.len() - 1) {
            println!("{}", cached_user[i].login_name);
            if cached_user[i].login_name == login_name {
                final_index = i;
                // Auch beim Suchen eines Users -> Timestamp für Cache Eintrag aktualisieren
                println!("Cache: Update Timestamp vom Cache Eintrag");
                cached_user[i].cache_timestamp = Utc::now();
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

    fn update_or_insert_cached_user(&self, user_found: bool, index: usize, login_name: &'a str, auth_token: &'a str, auth_token_timestamp: &'a str, is_admin: bool ) -> Result<(), anyhow::Error> {
        let mut cached_user = self.cached_user.lock().unwrap();

        let user = User {login_name, auth_token,
                         auth_token_timestamp:  DateTime::parse_from_rfc3339(auth_token_timestamp)?.with_timezone(&Utc),
                          cache_timestamp: Utc::now() };

        if user_found  {
            // update Nutzer
            println!("Cache: mache ein Update zum User");
            cached_user.remove(index);
            // user.auth_token = auth_token;
            // user.auth_token_timestamp = DateTime::parse_from_rfc3339(auth_token_timestamp)?.with_timezone(&Utc);
            // user.cache_timestamp = Utc::now();
            // let vec_part_one = cached_user[..index].to_vec();
            // let vec_part_two = cached_user[(index + 1)..].to_vec();
            // vec_part_one.append(&mut vec_part_two);

            // *cached_user = vec_part_one;

        }
        // Füge User neu im Cache hinzu, da nicht im cache vorhanden
        println!("Cache: Füge Eintrag neu in Cache hinzu");
        cached_user.push(user);

        Ok(())
    }

    fn check_token(&self, index: usize, auth_token: &str) -> bool {

        let cached_user = self.cached_user.lock().unwrap();


        if auth_token != cached_user[index].auth_token { println!("Cache: Token aus dem Header stimmt nicht mit dem Token aus dem cache überein"); return false};

        let current_timestamp = Utc::now();
        let time_diff = current_timestamp.signed_duration_since(cached_user[index].auth_token_timestamp).num_hours();

        // Wenn token älter ist als 24 Stunden
        if time_diff > 24  {
            return false;
        }
        return true;

    }

}


