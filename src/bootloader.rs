use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use oas3;

pub async fn bootloader(
	req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, std::convert::Infallible> {
	let path = req.uri().path().to_string();

	let open_api_spec = oas3::from_path("examples/openapi.json").unwrap();
	let version = open_api_spec.validate_version().unwrap().to_string();

	let response_body = format!("{}\n{}", path, version);

	Ok(Response::new(Full::new(Bytes::from(response_body))))
}
