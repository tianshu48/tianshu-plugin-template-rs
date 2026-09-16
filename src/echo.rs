//! Pure step helper. `read` is WIT `host.read` in the guest; tests inject a closure.

#[cfg_attr(not(feature = "guest"), allow(dead_code))]
pub fn ir_text(raw: &str) -> String {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) else {
        return raw.to_string();
    };
    match v {
        serde_json::Value::Object(m) => match m.get("type").and_then(|t| t.as_str()) {
            Some("String") => m
                .get("value")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            Some("Number") => m.get("value").map(|x| x.to_string()).unwrap_or_default(),
            _ => raw.to_string(),
        },
        serde_json::Value::String(s) => s,
        _ => raw.to_string(),
    }
}

#[cfg_attr(not(feature = "guest"), allow(dead_code))]
pub fn step_echo(
    kind: &str,
    params_json: &str,
    read: impl Fn(&str, &str, &str) -> String,
) -> String {
    if kind != "echo" {
        return String::new();
    }
    let params: serde_json::Value =
        serde_json::from_str(params_json).unwrap_or(serde_json::Value::Object(Default::default()));
    if let Some(spec) = params.get("host") {
        let kind = spec.get("kind").and_then(|v| v.as_str()).unwrap_or("");
        let a = spec.get("a").and_then(|v| v.as_str()).unwrap_or("");
        let b = spec.get("b").and_then(|v| v.as_str()).unwrap_or("");
        return ir_text(&read(kind, a, b));
    }
    if let Some(key) = params.get("info").and_then(|v| v.as_str()) {
        return ir_text(&read("info", key, ""));
    }
    let mut text = params
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if text.is_empty() {
        text = ir_text(&read("in", "in", ""));
    }
    format!("Echo: {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ir_str(s: &str) -> String {
        serde_json::json!({"type":"String","value":s}).to_string()
    }

    #[test]
    fn unknown_kind_is_empty() {
        assert_eq!(step_echo("other", "{}", |_, _, _| String::new()), "");
    }

    #[test]
    fn text_param() {
        assert_eq!(
            step_echo("echo", r#"{"text":"ping"}"#, |_, _, _| String::new()),
            "Echo: ping"
        );
    }

    #[test]
    fn inbound_when_text_empty() {
        assert_eq!(
            step_echo("echo", "{}", |k, a, _| {
                assert_eq!(k, "in");
                assert_eq!(a, "in");
                ir_str("in-edge")
            }),
            "Echo: in-edge"
        );
    }

    #[test]
    fn info_and_host_passthrough() {
        assert_eq!(
            step_echo("echo", r#"{"info":"plugin.id"}"#, |k, a, _| {
                assert_eq!(k, "info");
                assert_eq!(a, "plugin.id");
                ir_str("example.community.template")
            }),
            "example.community.template"
        );
        assert_eq!(
            step_echo(
                "echo",
                r#"{"host":{"kind":"param","a":"text","b":""}}"#,
                |k, a, _| {
                    assert_eq!(k, "param");
                    assert_eq!(a, "text");
                    ir_str("from-param")
                }
            ),
            "from-param"
        );
    }

    #[test]
    fn ir_unwrap() {
        assert_eq!(ir_text(&ir_str("x")), "x");
        assert_eq!(ir_text(r#"{"type":"Number","value":2}"#), "2");
        assert_eq!(ir_text("plain"), "plain");
    }
}
