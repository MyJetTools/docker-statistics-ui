use flurl::FlUrl;

pub async fn get_logs(flurl: FlUrl, container_id: String, lines: u32) -> Result<String, String> {
    let url_response = flurl
        .append_path_segment("api")
        .append_path_segment("containers")
        .append_path_segment("logs")
        .append_query_param("id", Some(container_id))
        .append_query_param("lines_number", Some(lines.to_string()))
        .get()
        .await;

    if let Err(err) = &url_response {
        return Err(format!("Error: {:?}", err));
    };

    let result = url_response.unwrap().receive_body().await;

    if let Err(err) = &result {
        return Err(format!("Error: {:?}", err));
    };

    let result = String::from_utf8(result.unwrap()).unwrap();

    Ok(result)
}
