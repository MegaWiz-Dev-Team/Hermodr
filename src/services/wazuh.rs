//! Wazuh MCP tools — SIEM integration.

use crate::jsonrpc::{RpcError, CODE_INTERNAL_ERROR};
use crate::proxy::Proxy;
use crate::registry::ToolDefinition;
use crate::AppState;
use std::sync::Arc;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "get_wazuh_alerts".into(),
            description: "Get recent security alerts from Wazuh.".into(),
            method: "GET".into(),
            path: "/security/alerts".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": { "type": "integer", "default": 20 }
                }
            }),
        },
        ToolDefinition {
            name: "get_wazuh_agents".into(),
            description: "List Wazuh agents and their status.".into(),
            method: "GET".into(),
            path: "/agents".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        ToolDefinition {
            name: "get_critical_vulnerabilities".into(),
            description: "Get critical vulnerabilities detected on agents.".into(),
            method: "GET".into(),
            path: "/vulnerability/agents".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "severity": { "type": "string", "default": "Critical" }
                }
            }),
        },
    ]
}

pub async fn get_auth_header(_state: &crate::AppState) -> Result<(String, String), crate::jsonrpc::RpcError> {
    let user = std::env::var("WAZUH_API_USER").map_err(|_| crate::jsonrpc::RpcError {
        code: crate::jsonrpc::CODE_INTERNAL_ERROR,
        message: "WAZUH_API_USER environment variable not set".to_string(),
    })?;
    let pass = std::env::var("WAZUH_API_PASSWORD").map_err(|_| crate::jsonrpc::RpcError {
        code: crate::jsonrpc::CODE_INTERNAL_ERROR,
        message: "WAZUH_API_PASSWORD environment variable not set".to_string(),
    })?;
    use base64::{engine::general_purpose, Engine as _};
    let token = general_purpose::STANDARD.encode(format!("{}:{}", user, pass));
    Ok(("Authorization".to_string(), format!("Basic {}", token)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 3);
    }
}
