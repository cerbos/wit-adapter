wit_bindgen::generate!({
   additional_derives: [PartialEq]
});
use crate::exports::cerbos::policy::authorization as authz;
use cerbos_hub::epdp::authorization;
use wasi::clocks::wall_clock;

mod from_wit;
mod to_wit;

struct Cerbos;

impl crate::exports::cerbos::policy::authorization::Guest for Cerbos {
    fn check(input: authz::Request) -> Result<authz::Response, _rt::String> {
        let request = from_wit::map_request(&input);
        let s = serde_json::to_string(&request).unwrap();
        let result = authorization::check_wasi(&s, wall_clock::now().seconds);
        let response: cerbos_serde::Response =
            serde_json::from_str(&result).map_err(|e| e.to_string())?;
        let response: Result<authz::Response, String> = to_wit::map_response(&response);
        response
    }
}

export!(Cerbos);
