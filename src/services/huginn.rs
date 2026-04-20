//! Huginn MCP tools — Security SAST scanning integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "get_security_findings".into(),
        description: "Retrieves vulnerabilities identified during Huginn scans.".into(),
        method: "GET".into(),
        path: "/api/v1/findings".into(),
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
