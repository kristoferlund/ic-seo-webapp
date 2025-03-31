use ic_http_certification::{HttpRequest, HttpResponse};
use router_library::router::RouteParams;

pub fn handler(_: HttpRequest, _: RouteParams) -> HttpResponse<'static> {
    HttpResponse::not_found(
        b"Not Found",
        vec![("Content-Type".into(), "text/plain".into())],
    )
    .build()
}
