//! Bifrost MCP tools — multi-agent swarm engine integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_swarm_status".into(),
        description: "Get the status of a specific swarm run by ID.".into(),
        method: "GET".into(),
        path: "/api/v1/swarm/status/{id}".into(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "string" }
            },
            "required": ["id"]
        }),
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
