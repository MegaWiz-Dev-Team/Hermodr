//! Forseti MCP tools — Testing and QA pipeline integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_test_report".into(),
        description: "Get the detailed test report by report ID.".into(),
        method: "GET".into(),
        path: "/api/v1/tests/report/{id}".into(),
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
