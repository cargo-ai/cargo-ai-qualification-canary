//! Author-owned implementation area for this Cargo AI tool.
//!
//! Start here when changing what the tool does. The protocol adapter in
//! `lib.rs` calls these functions to produce `describe` output and to handle
//! `invoke` requests.

use crate::{
    AccessLevel, InvocationContext, ParamSpec, ResourceProfile, ResultSpec, SelfTestSpec, ToolError,
};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub(crate) const TOOL_NAME: &str = "qualification_probe";

pub(crate) fn description() -> &'static str {
    "Return a deterministic canonical summary of structured qualification inputs."
}

pub(crate) fn params() -> BTreeMap<String, ParamSpec> {
    BTreeMap::from([
        (
            "enabled".to_string(),
            ParamSpec {
                kind: "boolean".to_string(),
                required: false,
                description: "Whether the probe is enabled.".to_string(),
                default: Some(Value::Bool(true)),
            },
        ),
        (
            "labels".to_string(),
            ParamSpec {
                kind: "array".to_string(),
                required: true,
                description: "Ordered string labels included in the result.".to_string(),
                default: None,
            },
        ),
        (
            "metadata".to_string(),
            ParamSpec {
                kind: "object".to_string(),
                required: true,
                description: "Structured metadata canonicalized by key.".to_string(),
                default: None,
            },
        ),
    ])
}

pub(crate) fn result() -> ResultSpec {
    ResultSpec {
        kind: "string".to_string(),
        nullable: true,
        description: "Canonical JSON summary of the validated structured inputs.".to_string(),
    }
}

pub(crate) fn resource_profile() -> ResourceProfile {
    ResourceProfile {
        network: AccessLevel::None,
        filesystem_read: AccessLevel::None,
        filesystem_write: AccessLevel::None,
        subprocess: AccessLevel::None,
        env_read: AccessLevel::None,
        credential_access: AccessLevel::None,
    }
}

pub(crate) fn self_test() -> SelfTestSpec {
    SelfTestSpec {
        supported: true,
        safe: true,
        description: "Pure deterministic transformation with no external access.".to_string(),
    }
}

pub(crate) fn minimal_example_params() -> BTreeMap<String, Value> {
    BTreeMap::from([
        ("labels".to_string(), serde_json::json!(["canary"])),
        (
            "metadata".to_string(),
            serde_json::json!({"source": "minimal"}),
        ),
    ])
}

pub(crate) fn full_example_params() -> BTreeMap<String, Value> {
    BTreeMap::from([
        ("enabled".to_string(), Value::Bool(true)),
        (
            "labels".to_string(),
            serde_json::json!(["windows", "macos", "ubuntu"]),
        ),
        (
            "metadata".to_string(),
            serde_json::json!({"revision": 1, "source": "full"}),
        ),
    ])
}

pub(crate) fn invoke(
    params: BTreeMap<String, Value>,
    _context: InvocationContext,
) -> Result<Option<String>, ToolError> {
    let mut params = params;
    let enabled = match params.remove("enabled") {
        Some(Value::Bool(value)) => value,
        Some(_) => return Err(ToolError::new("Parameter 'enabled' must be a boolean.")),
        None => true,
    };
    let labels = match params.remove("labels") {
        Some(Value::Array(values)) if values.iter().all(|value| value.as_str().is_some()) => {
            Value::Array(values)
        }
        Some(_) => {
            return Err(ToolError::new(
                "Parameter 'labels' must be an array containing only strings.",
            ));
        }
        None => return Err(ToolError::new("Missing required parameter 'labels'.")),
    };
    let metadata = match params.remove("metadata") {
        Some(Value::Object(values)) => Value::Object(values),
        Some(_) => return Err(ToolError::new("Parameter 'metadata' must be an object.")),
        None => return Err(ToolError::new("Missing required parameter 'metadata'.")),
    };
    if let Some(unexpected) = params.keys().next() {
        return Err(ToolError::new(format!(
            "Unexpected parameter '{unexpected}'."
        )));
    }

    let result = Value::Object(Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        ("labels".to_string(), canonicalize(labels)),
        ("metadata".to_string(), canonicalize(metadata)),
    ]));
    Ok(Some(serde_json::to_string(&result)?))
}

fn canonicalize(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize).collect()),
        Value::Object(values) => {
            let ordered = values
                .into_iter()
                .map(|(key, value)| (key, canonicalize(value)))
                .collect::<BTreeMap<_, _>>();
            Value::Object(Map::from_iter(ordered))
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_structured_inputs() {
        let result = invoke(
            BTreeMap::from([
                ("labels".to_string(), serde_json::json!(["beta", "alpha"])),
                (
                    "metadata".to_string(),
                    serde_json::json!({"z": 2, "nested": {"b": true, "a": false}}),
                ),
            ]),
            InvocationContext::default(),
        )
        .expect("probe should succeed")
        .expect("probe should return a summary");

        assert_eq!(
            result,
            r#"{"enabled":true,"labels":["beta","alpha"],"metadata":{"nested":{"a":false,"b":true},"z":2}}"#
        );
    }

    #[test]
    fn rejects_non_string_labels() {
        let error = invoke(
            BTreeMap::from([
                ("labels".to_string(), serde_json::json!(["ok", 7])),
                ("metadata".to_string(), serde_json::json!({})),
            ]),
            InvocationContext::default(),
        )
        .expect_err("mixed labels must fail");

        assert!(error.to_string().contains("only strings"));
    }
}
