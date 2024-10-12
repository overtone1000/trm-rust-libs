use hyper::{header::HeaderValue, Request, Response};

pub const HEADER_ORIGIN: &str = "Origin";
const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";

type Allowed_Origin_Function<T, U> = dyn Fn(&Request<T>, &hyper::Response<U>) -> String;
pub fn set_cors_allowed_origins<T, U>(
    request: &Request<T>,
    response: &mut Response<U>,
    allowed_origins: &Allowed_Origin_Function<T, U>,
) {
    let valstr = allowed_origins(request, response);
    let value: HeaderValue = HeaderValue::from_str(&valstr).expect("Should be a valid header.");
    response
        .headers_mut()
        .insert(HEADER_ACCESS_CONTROL_ALLOW_ORIGIN, value);
}

pub fn permit_all_cors<T>(response: &mut Response<T>) {
    let value: HeaderValue = HeaderValue::from_str("*").expect("Should be a valid header.");
    response
        .headers_mut()
        .insert(HEADER_ACCESS_CONTROL_ALLOW_ORIGIN, value);
}
