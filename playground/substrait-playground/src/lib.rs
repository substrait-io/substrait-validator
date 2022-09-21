// SPDX-License-Identifier: Apache-2.0

use futures::future::join_all;
use prost::Message;
use std::{cell::RefCell, collections::HashMap, fmt::Display, str};
use substrait::protobuf::Plan;
use substrait_validator::{export::Format, Config};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Error(String);
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
impl std::error::Error for Error {}

// TODO(mbrobbel): remove and just rely on browser cache?
thread_local! {
    static CACHE: RefCell<HashMap<String, Result<Vec<u8>, Error>>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
pub fn version() -> String {
    substrait_validator::version().into()
}

#[wasm_bindgen]
pub fn substrait_version() -> String {
    substrait_validator::substrait_version().into()
}

#[wasm_bindgen]
pub async fn validate(plan: String) -> Result<String, String> {
    console_error_panic_hook::set_once();

    let plan = serde_json::from_str::<Plan>(&plan).map_err(|e| e.to_string())?;

    join_all(
        plan.extension_uris
            .iter()
            .map(|extension_uri| extension_uri.uri.clone())
            .filter(|uri| CACHE.with(|cache| !cache.borrow().contains_key(uri)))
            .map(|uri| async {
                let res = reqwest::get(&uri)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .map(|string| string.as_bytes().to_vec())
                    .map_err(|e| Error(e.to_string()));
                (uri, res)
            }),
    )
    .await
    .into_iter()
    .for_each(|(uri, result)| {
        CACHE.with(|cache| {
            cache.borrow_mut().insert(uri, result);
        });
    });

    let mut config = Config::new();
    config.add_uri_resolver(move |url| -> Result<Vec<u8>, Error> {
        CACHE.with(|cache| match cache.borrow().get(url) {
            Some(res) => res
                .as_ref()
                .map(|data| data.clone())
                .map_err(|e| Error(e.to_string())),
            None => Err(Error("URI not resolved".to_string())),
        })
    });
    config.set_max_uri_resolution_depth(Some(1));

    // Validate
    let mut output = Vec::new();
    substrait_validator::parse(plan.encode_to_vec().as_slice(), &config)
        .export(&mut output, Format::Html)
        .map_err(|e| e.to_string())?;

    Ok(str::from_utf8(output.as_slice())
        .map_err(|e| e.to_string())?
        .to_string())
}
