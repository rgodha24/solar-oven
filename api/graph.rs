use serde::{Deserialize, Serialize};
use solar_oven::{GraphDataResponse, Oven};
use specta::{export, Type};
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

#[derive(Debug, Serialize, Deserialize, Type)]
struct Payload {
    oven: Oven,
    reflector_ml: f64,
    data_type: GraphDataResponse,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<Payload>();

    if let Ok(Some(Payload {
        oven,
        reflector_ml,
        data_type,
    })) = payload
    {
        let graph_data = oven.graph_data(reflector_ml, data_type);

        println!("{:?}", graph_data);

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&graph_data)?.into())
            .unwrap())
    } else {
        bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        })
    }
}
