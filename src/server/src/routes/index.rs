use std::{borrow::Cow, collections::HashMap};

use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use minijinja::Environment;
use router_library::router::RouteParams;

pub fn handler(_: HttpRequest, _: RouteParams) -> HttpResponse<'static> {
    let html = include_str!("../../../dist/index.html");
    let env = Environment::new();
    let template = env.template_from_str(html).unwrap();
    let mut ctx = HashMap::new();
    ctx.insert("title", "Main index page");
    let rendered = template.render(ctx).unwrap();
    HttpResponse::builder()
        .with_headers(vec![("Content-Type".into(), "text/html".into())])
        .with_status_code(StatusCode::OK)
        .with_body(Cow::Owned(rendered.into_bytes()))
        .build()
}
