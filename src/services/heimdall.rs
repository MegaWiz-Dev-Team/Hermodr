//! Heimdall MCP tools — LLM Gateway integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "switch_model".into(),
            description: "Switch the underlying active LLM model used by Heimdall Gateway (e.g., step up to Gemini for complex cases).".into(),
            method: "POST".into(),
            path: "/v1/models/switch".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "model": {
                        "type": "string",
                        "description": "Model identifier to switch to"
                    }
                },
                "required": ["model"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 1);
    }

    #[test]
    fn test_switch_model_definition() {
        let t = tools().into_iter().find(|t| t.name == "switch_model").unwrap();
        assert_eq!(t.method, "POST");
        assert_eq!(t.path, "/v1/models/switch");
        assert!(!t.description.is_empty());
    }
}
