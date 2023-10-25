use flurl::IntoFlUrl;

use crate::models::StatisticsContract;

pub async fn get_statistics(url: String) -> Result<StatisticsContract, String> {
    let url_response = url
        .append_path_segment("api")
        .append_path_segment("containers")
        .get()
        .await;

    if let Err(err) = &url_response {
        return Err(format!("Error: {:?}", err));
    };

    let result = url_response.unwrap().get_json().await;

    if let Err(err) = &result {
        return Err(format!("Error: {:?}", err));
    };

    Ok(result.unwrap())
}
