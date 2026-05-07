//! Mimir MCP tools — application layer integration.
//!
//! ocr_extract used to live here briefly in Sprint 50 Day-2 but moved to
//! services/syn.rs once syn-api was extracted into its own service.

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
