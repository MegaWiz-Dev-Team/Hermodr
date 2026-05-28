//! Loki MCP tools — Security vulnerability scanning and testing.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "tool_enumerate_targets".into(),
            description: "Enumerate security testing targets (endpoints) in a service.".into(),
            method: "POST".into(),
            path: "/api/v1/loki/enumerate".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "target_service": {
                        "type": "string",
                        "enum": ["bifrost", "heimdall", "mimir", "syn", "qdrant", "mariadb", "all"],
                        "description": "Target service to enumerate"
                    },
                    "depth": {
                        "type": "string",
                        "default": "1",
                        "description": "Enumeration depth (1 = endpoints only)"
                    },
                    "include_auth_methods": {
                        "type": "boolean",
                        "default": true,
                        "description": "Include authentication method details"
                    }
                },
                "required": ["target_service"]
            }),
        },
        ToolDefinition {
            name: "tool_api_injection".into(),
            description: "Test endpoints for API injection vulnerabilities (SQL injection, parameter tampering, JWT attacks, etc).".into(),
            method: "POST".into(),
            path: "/api/v1/loki/test/api-injection".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "target_endpoint": {
                        "type": "string",
                        "description": "API endpoint to test (e.g., /api/v1/knowledge/search)"
                    },
                    "test_type": {
                        "type": "string",
                        "enum": ["sql_injection", "parameter_tampering", "jwt_manipulation", "authorization_bypass", "header_injection", "path_traversal"],
                        "description": "Type of injection attack to test"
                    },
                    "verbose": {
                        "type": "boolean",
                        "default": false,
                        "description": "Include detailed payload responses"
                    },
                    "timeout_seconds": {
                        "type": "integer",
                        "default": 10,
                        "description": "Test timeout in seconds"
                    }
                },
                "required": ["target_endpoint", "test_type"]
            }),
        },
        ToolDefinition {
            name: "tool_prompt_injection".into(),
            description: "Test LLM endpoints for prompt injection vulnerabilities and jailbreak attempts.".into(),
            method: "POST".into(),
            path: "/api/v1/loki/test/prompt-injection".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "injection_type": {
                        "type": "string",
                        "enum": ["system_override", "jailbreak", "token_smuggling", "indirect_injection", "context_bleeding", "role_play", "constraint_escape", "hidden_instruction"],
                        "description": "Type of prompt injection to test"
                    },
                    "model": {
                        "type": "string",
                        "default": "gemma-4-26b",
                        "description": "LLM model to test against"
                    },
                    "payload": {
                        "type": "string",
                        "description": "Custom injection payload (optional, uses defaults if not provided)"
                    },
                    "check_skuggi": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check if Skuggi PII guard detects the injection"
                    },
                    "verbose": {
                        "type": "boolean",
                        "default": false,
                        "description": "Include LLM response details"
                    }
                },
                "required": ["injection_type"]
            }),
        },
        ToolDefinition {
            name: "tool_data_exfiltration".into(),
            description: "Test for unauthorized data access and exfiltration attempts across services.".into(),
            method: "POST".into(),
            path: "/api/v1/loki/test/data-exfiltration".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "target": {
                        "type": "string",
                        "description": "Target service or endpoint"
                    },
                    "exfiltration_method": {
                        "type": "string",
                        "enum": ["unauthorized_read", "cross_tenant_access", "privilege_escalation", "direct_database"],
                        "description": "Exfiltration technique to test"
                    },
                    "sample_doc": {
                        "type": "string",
                        "description": "Sample document ID to attempt exfiltration of (optional)"
                    },
                    "verbose": {
                        "type": "boolean",
                        "default": false,
                        "description": "Include detailed attempt logs"
                    },
                    "expect_failure": {
                        "type": "boolean",
                        "default": true,
                        "description": "Test expects failure (security guard blocking access)"
                    }
                },
                "required": ["target", "exfiltration_method"]
            }),
        },
        ToolDefinition {
            name: "tool_validate_tyr".into(),
            description: "Validate that Tyr SIEM correctly detects and alerts on security test payloads.".into(),
            method: "POST".into(),
            path: "/api/v1/loki/test/validate-tyr".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "test_scenario": {
                        "type": "string",
                        "enum": ["sql_injection_detected", "prompt_injection_blocked", "unauthorized_access_logged", "data_exfiltration_prevented"],
                        "description": "Tyr detection scenario to validate"
                    },
                    "payload": {
                        "type": "string",
                        "description": "Test payload to send (optional)"
                    }
                },
                "required": ["test_scenario"]
            }),
        },
        ToolDefinition {
            name: "tool_scan_all".into(),
            description: "Execute comprehensive automated vulnerability scan across all Asgard and MegaCare services.".into(),
            method: "POST".into(),
            path: "/api/v1/loki/scan".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "environment": {
                        "type": "string",
                        "enum": ["production", "development"],
                        "default": "production",
                        "description": "Target environment"
                    },
                    "targets": {
                        "type": "array",
                        "items": { "type": "string" },
                        "default": ["bifrost", "heimdall", "mimir", "syn"],
                        "description": "Specific services to scan"
                    },
                    "include_megacare": {
                        "type": "boolean",
                        "default": false,
                        "description": "Also scan MegaCare admin portals"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "tool_quick_scan".into(),
            description: "Run quick vulnerability scan on selected endpoints (faster than full scan).".into(),
            method: "POST".into(),
            path: "/api/v1/loki/scan/quick".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "target_endpoints": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Specific endpoints to test"
                    },
                    "test_types": {
                        "type": "array",
                        "items": { "type": "string" },
                        "default": ["sql_injection", "prompt_injection"],
                        "description": "Types of tests to run"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "tool_get_results".into(),
            description: "Retrieve historical vulnerability test results and findings.".into(),
            method: "GET".into(),
            path: "/api/v1/loki/results".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "default": 50,
                        "description": "Maximum results to return"
                    },
                    "offset": {
                        "type": "integer",
                        "default": 0,
                        "description": "Result offset for pagination"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "tool_generate_report".into(),
            description: "Generate vulnerability scan report summarizing findings and recommendations.".into(),
            method: "GET".into(),
            path: "/api/v1/loki/scan/report".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "format": {
                        "type": "string",
                        "enum": ["text", "json"],
                        "default": "text",
                        "description": "Report output format"
                    }
                }
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loki_tool_count() {
        assert_eq!(tools().len(), 9);
    }

    #[test]
    fn test_tool_names() {
        let tool_names: Vec<String> = tools().iter().map(|t| t.name.clone()).collect();
        assert!(tool_names.contains(&"tool_enumerate_targets".into()));
        assert!(tool_names.contains(&"tool_api_injection".into()));
        assert!(tool_names.contains(&"tool_prompt_injection".into()));
        assert!(tool_names.contains(&"tool_scan_all".into()));
    }
}
