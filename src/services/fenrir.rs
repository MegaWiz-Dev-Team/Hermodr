//! Fenrir MCP tools — CI/CD automation integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_deployment_status".into(),
        description: "Get the current deployment status from the pipeline.".into(),
        method: "GET".into(),
        path: "/api/v1/deployments/status".into(),
        input_schema: serde_json::json!({"type": "object", "properties": {}}),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 1);
    }
}
