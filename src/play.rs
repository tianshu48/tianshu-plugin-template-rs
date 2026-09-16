//! Play examples: `sum` (data out) and `set_attr` (host writes attrs).

use super::echo::ir_text;

fn ir_number(raw: &str) -> f64 {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) else {
        return ir_text(raw).trim().parse().unwrap_or(0.0);
    };
    match v {
        serde_json::Value::Object(m)
            if m.get("type").and_then(|t| t.as_str()) == Some("Number") =>
        {
            m.get("value").and_then(|x| x.as_f64()).unwrap_or(0.0)
        }
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        _ => ir_text(raw).trim().parse().unwrap_or(0.0),
    }
}

fn addend(params: &serde_json::Value) -> f64 {
    match params.get("add") {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

#[cfg_attr(not(feature = "guest"), allow(dead_code))]
pub fn step_sum(
    kind: &str,
    params_json: &str,
    read: impl Fn(&str, &str, &str) -> String,
) -> String {
    if kind != "sum" {
        return String::new();
    }
    let params: serde_json::Value =
        serde_json::from_str(params_json).unwrap_or(serde_json::Value::Object(Default::default()));
    let n = ir_number(&read("in", "in", "")) + addend(&params);
    serde_json::json!({
        "log": format!("sum {n}"),
        "out": { "type": "Number", "value": n },
    })
    .to_string()
}

#[cfg_attr(not(feature = "guest"), allow(dead_code))]
pub fn step_set_attr(kind: &str, params_json: &str) -> String {
    if kind != "set_attr" {
        return String::new();
    }
    let params: serde_json::Value =
        serde_json::from_str(params_json).unwrap_or(serde_json::Value::Object(Default::default()));
    let subject = params
        .get("subject")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let attr = params
        .get("attr")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let value = match params.get("value") {
        Some(serde_json::Value::Number(n)) => serde_json::json!({
            "type": "Number",
            "value": n.as_f64().unwrap_or(0.0)
        }),
        Some(serde_json::Value::String(s)) => {
            if let Ok(n) = s.trim().parse::<f64>() {
                serde_json::json!({ "type": "Number", "value": n })
            } else {
                serde_json::json!({ "type": "String", "value": ir_text(s) })
            }
        }
        _ => serde_json::json!({ "type": "Number", "value": 0.0 }),
    };
    serde_json::json!({
        "log": format!("set {subject}.{attr}"),
        "set": [{ "subject": subject, "attr": attr, "value": value }],
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ir_num(n: f64) -> String {
        serde_json::json!({"type":"Number","value":n}).to_string()
    }

    #[test]
    fn sum_adds_inbound_and_field() {
        let raw = step_sum("sum", r#"{"add":2}"#, |k, a, _| {
            assert_eq!(k, "in");
            assert_eq!(a, "in");
            ir_num(10.0)
        });
        let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(v["out"]["value"], 12.0);
        assert_eq!(v["log"], "sum 12");
    }

    #[test]
    fn set_attr_builds_host_set() {
        let raw = step_set_attr("set_attr", r#"{"subject":"Hero","attr":"hp","value":"7"}"#);
        let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(v["set"][0]["subject"], "Hero");
        assert_eq!(v["set"][0]["value"]["value"], 7.0);
    }
}
