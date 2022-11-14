use wasmedge_http_req::request;
use anyhow::anyhow;

pub async fn make_auth_request(login_name: String, auth_token: String) -> Result<std::string::String, anyhow::Error>{


    let addr_with_params = format!("0.0.0.0:8000/checkAuthUser?login_name={}&auth_token={}&isAdmin=true", login_name, auth_token);

    //let body_string = format!("{{\"login_name\" : \"{}\", \"auth_token\" : \"{}\"}}", login_name, auth_token);
    let mut writer = Vec::new(); //container for body of a response

    // b "..." -> Byte string literal; constructs an array of bytes instead of a string
     const BODY: &[u8; 2] = b"{}";
    let res = request::post(format!("http://{}", addr_with_params), BODY, &mut writer).unwrap();

    if res.status_code().is_success() {
        println!("Status: {} {}", res.status_code(), res.reason());
        println!("Headers {}", res.headers());
        println!("{}", String::from_utf8_lossy(&writer));
        Ok("Authentification passed".to_string())

    } else {
        return Err(anyhow!("Request failed: {}", res.status_code()));
    }


}