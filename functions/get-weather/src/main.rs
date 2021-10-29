use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = handler_fn(get_weather);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn get_weather(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let api_token = std::env::var("WEATHER_KEY")
        .expect("An OpenWeather API key is required. See https://openweathermap.org/.");

    let lat = event.query_string_parameters.get("lat");
    let lon = event.query_string_parameters.get("lon");

    if let None = lat {
        return Ok(err_response("`lat` is required"));
    }
    if let None = lon {
        return Ok(err_response("`lon` is required"));
    }

    let client = reqwest::Client::new();
    let weather_response = client
        .get("http://api.openweathermap.org/data/2.5/onecall")
        .header(USER_AGENT, "the-weather-bat")
        .query(&[("appid", api_token), ("lat", lat.unwrap().to_owned()), ("lon", lon.unwrap().to_owned())])
        .send().await?;

    Ok(success_response(weather_response.text().await?))
}

fn err_response(err: &str) -> ApiGatewayProxyResponse {
    ApiGatewayProxyResponse {
        status_code: 400,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(err.to_string())),
        is_base64_encoded: Some(false),
    }
}

fn success_response (body: String) -> ApiGatewayProxyResponse{
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(body)),
        is_base64_encoded: Some(false),
    }
}
