use std::cell::RefCell;

use assets::{get_asset_headers, NO_CACHE_ASSET_CACHE_CONTROL};
use ic_asset_certification::{Asset, AssetConfig, AssetRouter};
use ic_cdk::api::{certified_data_set, data_certificate};
use ic_http_certification::{HttpRequest, HttpResponse};
use router::RouteNode;

pub mod assets;
pub mod build;
pub mod router;

thread_local! {
    static ASSET_ROUTER: RefCell<AssetRouter<'static>> = Default::default();
}

/// Serve assets that have already been certified, or upgrade the request to an update call
pub fn http_request(req: HttpRequest) -> HttpResponse {
    ASSET_ROUTER.with_borrow(|asset_router| {
        if let Ok(response) = asset_router.serve_asset(
            &data_certificate().expect("No data certificate available"),
            &req,
        ) {
            ic_cdk::println!("http_request: {:?}", req.url());
            response
        } else {
            HttpResponse::builder().with_upgrade(true).build()
        }
    })
}

/// Match incoming requests to the appropriate handler, generating assets as needed
/// and certifying them for future requests.
pub fn http_request_update(req: HttpRequest, root_route_node: &RouteNode) -> HttpResponse<'static> {
    ic_cdk::println!("http_request_update: {:?}", req.url());

    let path = req.get_path().unwrap();
    match root_route_node.match_path(&path) {
        Some((handler, params)) => {
            let response = handler(req, params);

            let asset = Asset::new(path.clone(), response.body().to_vec());

            // TODO: handlers should be able to specify asset settings when they generate assets
            let asset_config = AssetConfig::File {
                path: path.to_string(),
                content_type: Some("text/html".to_string()),
                headers: get_asset_headers(vec![(
                    "cache-control".to_string(),
                    NO_CACHE_ASSET_CACHE_CONTROL.to_string(),
                )]),
                fallback_for: vec![],
                aliased_by: vec![],
                encodings: vec![],
            };

            ASSET_ROUTER.with_borrow_mut(|asset_router| {
                if let Err(err) = asset_router.certify_assets(vec![asset], vec![asset_config]) {
                    ic_cdk::trap(format!("Failed to certify dynamic asset: {err}"));
                }
                certified_data_set(asset_router.root_hash());
            });

            response
        }
        None => HttpResponse::not_found(
            b"Not Found",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build(),
    }
}
