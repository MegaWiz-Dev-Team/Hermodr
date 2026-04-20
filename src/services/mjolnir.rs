//! Mjölnir MCP tools — Asgard load testing service integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_load_test_results".into(),
        description:
            "Extracts metrics like P99 latency and total capacity stats from a targeted run.".into(),
        method: "GET".into(),
        path: "/api/v1/load-test/results/{id}".into(),
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
