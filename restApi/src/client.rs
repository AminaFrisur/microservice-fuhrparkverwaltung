use wasmedge_http_req::request;

pub async fn make_post_request(addr_with_params: String) -> Result<(wasmedge_http_req::response::Response, String), anyhow::Error> {

        let mut writer = Vec::new(); //container for body of a response
        const BODY: &[u8; 2] = b"{}";
        println!("{}", addr_with_params);
        let res = request::post(format!("http://{}", addr_with_params), BODY, &mut writer)?;
        let response_json = String::from_utf8_lossy(&writer);
        Ok((res, response_json.to_string()))

}