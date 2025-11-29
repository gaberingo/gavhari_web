use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde_json::Value;

type ApiResponse<T> = Result<T, String>;

async fn fetchApi<T>(url: &str, method: &str, body: Option<Value>) -> ApiResponse<T>
where
    T: DeserializeOwned,
{
    let builder = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PUT" => Request::put(url),
        "DELETE" => Request::delete(url),
        _ => return Err("Method Tidak ada".to_string()),
    }
    .header("Content-Type", "application/json");

    let req = if let Some(body) = body {
        builder.body(body.to_string()).map_err(|e| e.to_string())?
    } else {
        builder.build().map_err(|e| e.to_string())?
    };

    let resp = req.send().await.map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(format!("Err {}: {}", resp.status(), resp.status_text()));
    }

    resp.json::<T>().await.map_err(|e| e.to_string())
}

