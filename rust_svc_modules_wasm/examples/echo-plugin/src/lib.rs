wit_bindgen::generate!({
    world: "plugin",
    path: "wit",
});

struct EchoPlugin;

impl exports::dxps::service_plugins::plugin_api::Guest for EchoPlugin {
    fn metadata() -> exports::dxps::service_plugins::plugin_api::PluginMetadata {
        exports::dxps::service_plugins::plugin_api::PluginMetadata {
            name: "echo".to_owned(),
            version: "0.1.0".to_owned(),
            description: "Echoes the incoming dispatch request as JSON".to_owned(),
        }
    }

    fn configure(config_json: String) -> Result<(), String> {
        if config_json.trim().is_empty() {
            return Err("config JSON must not be empty".to_owned());
        }
        Ok(())
    }

    fn handle(
        req: exports::dxps::service_plugins::plugin_api::Request,
    ) -> Result<exports::dxps::service_plugins::plugin_api::Response, String> {
        Ok(exports::dxps::service_plugins::plugin_api::Response {
            status: 200,
            body: format!(
                r#"{{"method":"{}","path":"{}","body":"{}"}}"#,
                escape_json(&req.method),
                escape_json(&req.path),
                escape_json(&req.body)
            ),
        })
    }
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

export!(EchoPlugin);
