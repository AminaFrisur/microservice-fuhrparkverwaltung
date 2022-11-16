use serde::{Deserialize};

#[path = "./circuitbreaker.rs"] mod circuitbreaker;
use crate::circuitbreaker::CircuitBreaker;
#[path = "./cache.rs"] mod cache;
use crate::cache::Cache;

#[derive(Deserialize)]
pub struct JsonResultUser<'a>  {
    login_name: &'a str,
    auth_token: &'a str,
    auth_token_timestamp: &'a str,
    is_admin: bool
}

pub async fn check_auth_user(mut cache: Cache, mut circuitbreaker: CircuitBreaker<'_>, addr_with_params: String, login_name: &str, auth_token: &str) -> Result<(), anyhow::Error> {

    let user_index_cache;
    let mut user_found = false;
    let mut check = false;

    match cache.get_user_index(login_name) {
        Ok(index) => {
            user_index_cache = index;
            check = match cache.check_token(user_index_cache, auth_token) {
                Ok(bool_value) => bool_value,
                Err(err) => return Err(err)
            };
            user_found = true
        },
        Err(_) => {
            user_index_cache = 0
        }
    };

    if user_found && check {
        println!("Auth: Nutzer {} ist im cache noch zwischengespeichert", login_name);
        return Ok(());
    } else {

        let response_json;

        match circuitbreaker.circuit_breaker_post_request(addr_with_params).await {
            Ok((_, response_json_string)) =>  {
              response_json = response_json_string;
                println!("{:?}", response_json);
            },
            Err(err) => return Err(err)
        }

        // insert user in cache
        println!("Insert in Cache: {} {}", login_name, auth_token);
        let response_json = response_json.replace(&['[', ']'][..], "");
        let json_result_data: JsonResultUser = serde_json::from_str(&response_json)?;
        cache.update_or_insert_cached_user(user_found, user_index_cache, json_result_data.login_name,
                                           json_result_data.auth_token,
                                           json_result_data.auth_token_timestamp)?;

    }


    Ok(())
}
