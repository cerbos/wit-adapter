use crate::exports::cerbos::policy::authorization as authz;
use cerbos_serde::{AuxData, Principal, Request, Resource, ResourceEntry};
use serde_json::{Map, Value};

pub fn map_request(r: &authz::Request) -> Request {
    Request {
        request_id: r.request_id.clone(),
        principal: map_principal(&r.principal),
        resources: r.resources.iter().map(map_resource_entry).collect(),
        aux_data: r.aux_data.as_ref().map(map_aux_data).unwrap_or_default(),
    }
}

fn map_resource_entry(r: &authz::ResourceEntry) -> ResourceEntry {
    ResourceEntry {
        actions: r.actions.iter().map(Into::into).collect(),
        resource: map_resource(&r.resource),
    }
}

fn map_resource(r: &authz::Resource) -> Resource {
    Resource {
        id: r.id.as_str().into(),
        policy_version: r.policy_version.as_ref().cloned().unwrap_or_default(),
        kind: r.kind.as_str().into(),
        scope: r.scope.as_ref().cloned().unwrap_or_default(),
        attr: map_attr(&r.attr_json),
    }
}

fn map_attr(attr: &Option<String>) -> Map<String, Value> {
    match attr {
        None => Default::default(),
        Some(s) => serde_json::from_str(s).unwrap(),
    }
}

fn map_aux_data(data: &authz::AuxData) -> AuxData {
    AuxData {
        jwt: map_attr(&data.jwt_json),
    }
}

fn map_principal(p: &authz::Principal) -> Principal {
    Principal {
        id: p.id.as_str().into(),
        policy_version: p.policy_version.as_ref().cloned().unwrap_or_default(),
        roles: p
            .roles
            .iter()
            .map(Into::into)
            .collect::<Vec<String>>()
            .into(),
        scope: p.scope.as_ref().cloned().unwrap_or_default(),
        attr: map_attr(&p.attr_json),
    }
}
