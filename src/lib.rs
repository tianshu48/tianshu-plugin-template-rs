//! WIT guest.

mod echo;
mod play;

#[allow(dead_code)]
const PLUGIN_JSON: &str = include_str!("../plugin.json");
#[allow(dead_code)]
const UI_JSON: &str = include_str!("../ui.json");

#[allow(dead_code)]
fn plugin_id() -> String {
    serde_json::from_str::<serde_json::Value>(PLUGIN_JSON)
        .ok()
        .and_then(|v| v.get("id")?.as_str().map(str::to_string))
        .unwrap_or_default()
}

#[cfg(feature = "guest")]
mod guest_api {
    wit_bindgen::generate!({
        world: "plugin",
        path: "wit/plugin.wit",
    });

    use super::echo::step_echo;
    use super::play::{step_set_attr, step_sum};
    use exports::tianshu::plugin::guest::Guest;
    use tianshu::plugin::host;

    struct Component;
    export!(Component);

    impl Guest for Component {
        fn abi_version() -> u32 {
            3
        }

        fn id() -> String {
            super::plugin_id()
        }

        fn ui_json() -> String {
            super::UI_JSON.into()
        }

        fn step(kind: String, params_json: String) -> String {
            match kind.as_str() {
                "echo" => step_echo(&kind, &params_json, |k, a, b| host::read(k, a, b)),
                "sum" => step_sum(&kind, &params_json, |k, a, b| host::read(k, a, b)),
                "set_attr" => step_set_attr(&kind, &params_json),
                _ => String::new(),
            }
        }

        fn on_tool(id: String) -> String {
            let Ok(ui) = serde_json::from_str::<serde_json::Value>(super::UI_JSON) else {
                return String::new();
            };
            let Some(tools) = ui.get("tools").and_then(|v| v.as_array()) else {
                return String::new();
            };
            for t in tools {
                if t.get("id").and_then(|v| v.as_str()) != Some(id.as_str()) {
                    continue;
                }
                let op = t.get("op").and_then(|v| v.as_str()).unwrap_or("");
                let args = t.get("args").cloned().unwrap_or(serde_json::json!({}));
                return host::apply(op, &args.to_string());
            }
            String::new()
        }
    }
}

#[cfg(test)]
mod desc_tests {
    #[test]
    fn plugin_json_matches_guest_id() {
        let v: serde_json::Value = serde_json::from_str(super::PLUGIN_JSON).unwrap();
        assert_eq!(v["id"], super::plugin_id());
        assert_eq!(v["id"], "example.community.template");
        assert_eq!(v["abi"], 3);
        assert_eq!(v["readme"], "USER.md");
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../schema/ui.schema.json")).unwrap();
        let ops = schema["properties"]["tools"]["items"]["properties"]["op"]["enum"]
            .as_array()
            .expect("op enum");
        let ui: serde_json::Value = serde_json::from_str(super::UI_JSON).unwrap();
        for t in ui["tools"].as_array().unwrap() {
            let op = t["op"].as_str().unwrap();
            assert!(
                ops.iter().any(|x| x.as_str() == Some(op)),
                "{op} missing from schema"
            );
        }
        assert!(std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/USER.md")).is_file());
        assert_eq!(ui["nodes"][0]["kind"], "echo");
        assert_eq!(ui["nodes"][1]["kind"], "sum");
        assert_eq!(ui["nodes"][1]["output"], "data");
        assert_eq!(ui["nodes"][2]["kind"], "set_attr");
        assert_eq!(ui["nodes"][2]["write"], "attr");
    }
}
