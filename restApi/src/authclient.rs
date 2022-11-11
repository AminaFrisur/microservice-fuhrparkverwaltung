use wasmedge_http_req::request;

pub async fn make_auth_request(login_name: String, auth_token: String) -> Result<std::string::String, anyhow::Error>{

    let addr = "0.0.0.0:8000/checkAuthUser";
    let mut writer = Vec::new(); //container for body of a response

    const BODY: &[u8; 42] = b"{\"field1\" : \"value1\", \"field2\" : \"value2\"}";
    let res = request::post(format!("http://{}/post", addr), BODY, &mut writer).unwrap();

    println!("POST");
    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
    Ok("TEST123".to_string())
}