//! Vardr MCP tools — platform observability integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "get_cluster_metrics".into(),
            description: "Get current system and cluster metrics from Vardr.".into(),
            method: "GET".into(),
            path: "/api/v1/metrics/cluster".into(),
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
        },
        ToolDefinition {
            name: "get_service_health".into(),
            description: "Get health status of a specific Asgard service.".into(),
            method: "GET".into(),
            path: "/api/v1/health/{service}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "service": { "type": "string" }
                },
                "required": ["service"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 2);
    }
}
