use cerbos_serde::{Response, ResultEntry};

use crate::exports::cerbos::policy::authorization::{self as authz};

fn map_effect(effect: &i32) -> authz::Effect {
    match *effect {
        1 => authz::Effect::Allow,
        2 => authz::Effect::Deny,
        3 => authz::Effect::NoMatch,
        _ => authz::Effect::Unspecified,
    }
}
fn map_result_entry(result_entry: &ResultEntry) -> authz::ResultEntry {
    authz::ResultEntry {
        resource: map_resource(&result_entry.resource),
        actions: result_entry
            .actions
            .iter()
            .map(|(action, effect)| (action.clone(), map_effect(effect)))
            .collect(),
        meta: map_meta(&result_entry.meta),
        outputs: result_entry
            .outputs
            .iter()
            .map(|x| authz::OutputEntry {
                src: x.src.clone(),
                val_json: serde_json::to_string(&x.val).unwrap(),
            })
            .collect(),
    }
}

fn map_meta(meta: &cerbos_serde::ResultEntryMeta) -> authz::ResultEntryMeta {
    authz::ResultEntryMeta {
        actions: meta
            .actions
            .iter()
            .map(|(k, v)| (k.clone(), map_effect_meta(v)))
            .collect(),
        effective_derived_roles: meta.effective_derived_roles.clone(),
    }
}

fn map_effect_meta(v: &cerbos_serde::EffectMeta) -> authz::EffectMeta {
    authz::EffectMeta {
        matched_scope: v.matched_scope.clone(),
        matched_policy: v.matched_policy.clone(),
    }
}

fn map_resource(r: &cerbos_serde::ResultEntryResource) -> authz::ResultEntryResource {
    authz::ResultEntryResource {
        id: r.id.clone(),
        policy_version: r.policy_version.clone(),
        kind: r.kind.clone(),
        scope: r.scope.clone(),
    }
}
pub fn map_response(response: &Response) -> Result<authz::Response, String> {
    Ok(authz::Response {
        request_id: response.request_id.clone(),
        results: response.results.iter().map(map_result_entry).collect(),
    })
}

#[cfg(test)]
mod tests {
    use crate::exports::cerbos::policy::authorization::{self as authz};
    use cerbos_serde::Response;

    #[test]
    fn map_response() {
        let r: Response = serde_json::from_str(
            r#"
            {
              "requestId": "test01",
              "results": [
                {
                  "resource": {
                    "id": "abc123",
                    "policyVersion": "",
                    "kind": "album:object",
                    "scope": ""
                  },
                  "actions": {
                    "view": 1
                  },
                  "meta": {
                    "actions": {
                      "view": {
                        "matchedScope": "",
                        "matchedPolicy": "resource.album_object.vdefault"
                      }
                    },
                    "effectiveDerivedRoles": []
                  },
                  "outputs": []
                }
              ]
            }
            "#,
        )
        .unwrap();
        let actual = super::map_response(&r).unwrap();
        let expected = authz::Response {
            request_id: r.request_id.clone(),
            results: vec![authz::ResultEntry {
                resource: authz::ResultEntryResource {
                    id: "abc123".to_string(),
                    policy_version: "".to_string(),
                    kind: "album:object".to_string(),
                    scope: "".to_string(),
                },
                actions: vec![("view".to_string(), authz::Effect::Allow)],
                meta: authz::ResultEntryMeta {
                    actions: vec![(
                        "view".to_string(),
                        authz::EffectMeta {
                            matched_scope: "".to_string(),
                            matched_policy: "resource.album_object.vdefault".to_string(),
                        },
                    )],
                    effective_derived_roles: vec![],
                },
                outputs: vec![],
            }],
        };
        assert_eq!(actual, expected);
    }
}
