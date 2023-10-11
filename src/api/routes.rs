// SPDX-License-Identifier: Apache-2.0
use actix_web::web;
use crate::api::registry::blobs::cache;
use crate::api::registry::forward::forward;

pub fn registry_api_config(cfg: &mut web::ServiceConfig) {
    // ---------------------------------------------------------------------------------------------
    // BLOBS
    // Get
    cfg.service(
        web::resource("/{name:((?:[^/]*/)*)(.*)}/blobs/{reference}")
            // retrieve a blob -
            .route(web::get().to(cache))

            // check the existence of a blob -
            .route(web::head().to(cache))

        // Forward everything else
    ).default_service(web::to(forward));
}