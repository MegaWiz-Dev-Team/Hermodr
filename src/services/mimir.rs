//! Mimir MCP tools — application layer integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "list_agents".into(),
        description: "List available agents configured in the Mimir database.".into(),
        method: "GET".into(),
        path: "/api/v1/agents".into(),
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
