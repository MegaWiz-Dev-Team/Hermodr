//! Odin MCP tools — Agent Orchestrator integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_workflow_status".into(),
        description: "Track the live status of the distributed workflow running via Odin.".into(),
        method: "GET".into(),
        path: "/api/v1/orchestrator/status/{id}".into(),
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
