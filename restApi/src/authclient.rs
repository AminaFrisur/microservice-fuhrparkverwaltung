use wasmedge_http_req::request;

pub async fn make_auth_request(addr_with_params: String) -> Result<(std::string::String, std::string::String), wasmedge_http_req::error::Error>{

    //let body_string = format!("{{\"login_name\" : \"{}\", \"auth_token\" : \"{}\"}}", login_name, auth_token);
    let mut writer = Vec::new(); //container for body of a response

    // b "..." -> Byte string literal; constructs an array of bytes instead of a string
    println!("AuthClient: Führe Request aus");
    const BODY: &[u8; 2] = b"{}";
    println!("{}", addr_with_params);
    let res = request::post(format!("http://{}", addr_with_params), BODY, &mut writer)?;

    if res.status_code().is_success() {
        println!("Status: {} {}", res.status_code(), res.reason());
        println!("Headers {}", res.headers());
        println!("{}", String::from_utf8_lossy(&writer));
    }

    Ok((format!("{}", res.status_code()), String::from_utf8_lossy(&writer).to_string()))


}