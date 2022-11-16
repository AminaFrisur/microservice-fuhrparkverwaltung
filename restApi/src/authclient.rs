use wasmedge_http_req::request;
#[path = "./cache.rs"] mod cache;
use crate::cache::Cache;
use crate::cache::User;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct JsonResultUser<'a>  {
    login_name: &'a str,
    auth_token: &'a str,
    auth_token_timestamp: &'a str,
    is_admin: bool
}

pub async fn make_auth_request(addr_with_params: String, mut cache: Cache, login_name: &str, auth_token: &str) -> Result<(std::string::String, std::string::String), anyhow::Error> {

    //let body_string = format!("{{\"login_name\" : \"{}\", \"auth_token\" : \"{}\"}}", login_name, auth_token);

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
        println!("NUTZER IST NOCH ZWISCHENGESPEICHERT");
        Ok(("Nutzer ist noch zwischengespeichert".to_string(), login_name.to_string()))
    } else {
        let mut writer = Vec::new(); //container for body of a response

        // b "..." -> Byte string literal; constructs an array of bytes instead of a string
        println!("AuthClient: Führe Request aus");
        const BODY: &[u8; 2] = b"{}";
        println!("{}", addr_with_params);
        let res = request::post(format!("http://{}", addr_with_params), BODY, &mut writer)?;

        if res.status_code().is_success() {
            // insert user in cache
            println!("Insert in Cache: {} {}", login_name, auth_token);


            let response_json = String::from_utf8_lossy(&writer);
            let response_json = response_json.replace(&['[', ']'][..], "");;
            println!("{:?}", response_json);
            let json_result_data: JsonResultUser = serde_json::from_str(&response_json)?;
            cache.update_or_insert_cached_user(user_found, user_index_cache, json_result_data.login_name, json_result_data.auth_token, json_result_data.auth_token_timestamp);

            // user.print_login_name();
            // user.print_auth_token();
            // user.print_auth_token_timestamp();
            println!("Status: {} {}", res.status_code(), res.reason());
            println!("Headers {}", res.headers());
        }


        Ok((format!("{}", res.status_code()), String::from_utf8_lossy(&writer).to_string()))
    }
}