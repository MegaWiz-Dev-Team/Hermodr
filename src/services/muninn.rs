//! Muninn MCP tools — Issue watcher and auto-fixer integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "list_open_issues".into(),
        description: "List openly watched issues currently tracked by Muninn.".into(),
        method: "GET".into(),
        path: "/api/v1/issues".into(),
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
