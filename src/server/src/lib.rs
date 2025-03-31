mod __route_tree;
mod routes;

use __route_tree::ROUTES;
use ic_asset_certification::AssetRouter;
use ic_cdk::{init, post_upgrade, query, update};
use ic_http_certification::{HttpRequest, HttpResponse};
use include_dir::{include_dir, Dir};
use router_library::assets::{certify_all_assets, delete_assets};
use std::cell::RefCell;

thread_local! {
    static ASSET_ROUTER: RefCell<AssetRouter<'static>> = Default::default();
}

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../dist");

#[init]
fn init() {
    //Certify all the pre-built assets produced by 'npm run build'
    certify_all_assets(&ASSETS_DIR);

    // The previous call will certify the pre-built index.html as the the asset
    // for the root path. We want this asset to be generated dynamically instead,
    // so we need to delete it. It will be generated on the first request.
    delete_assets(vec!["/"]);
}

#[post_upgrade]
fn post_upgrade() {
    init();
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    router_library::http_request(req)
}

#[update]
fn http_request_update(req: HttpRequest) -> HttpResponse {
    ROUTES.with(|routes| router_library::http_request_update(req, routes))
}
