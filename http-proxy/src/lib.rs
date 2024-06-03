wit_bindgen::generate!();

use cerbos::policy::authorization as authz;
use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let role = match request
            .path_with_query()
            .unwrap()
            .split('=')
            .collect::<Vec<_>>()[..]
        {
            ["/?role", role] => role.to_string(),
            _ => "user".to_string(),
        };
        let request = authz::Request {
            request_id: "test01".into(),
            principal: authz::Principal {
                id: "user".into(),
                roles: vec![role],
                scope: None,
                policy_version: None,
                attr_json: None,
            },
            resources: vec![authz::ResourceEntry {
                actions: vec!["view".into()],
                resource: authz::Resource {
                    id: "abc123".into(),
                    policy_version: None,
                    kind: "album:object".into(),
                    scope: None,
                    attr_json: Some(r#"{"public": true}"#.into()),
                },
            }],
            aux_data: None,
        };
        let res = authz::check(&request).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(format!("{:?}", res.results[0].actions[0].1).as_bytes())
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

export!(HttpServer);
