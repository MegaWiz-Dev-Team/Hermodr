//! Insurance MCP tools — Underwriting and RAG capabilities for the Insurance tenant.
//! Exposes tools to interact with the insurance policy knowledge base and core system.

use crate::registry::ToolDefinition;

/// Returns the available MCP tools for the Insurance sidecar
pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "insurance_policy_rag".into(),
            description: "Search the insurance policy knowledge base for underwriting rules and guidelines.".into(),
            method: "POST".into(),
            path: "/api/v1/insurance/rag/search".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The underwriting query, e.g. 'What is the HbA1c threshold for loading premium?'"
                    }
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "generate_underwriting_report".into(),
            description: "Generate a formal underwriting decision report for the core insurance system (eBao).".into(),
            method: "POST".into(),
            path: "/api/v1/insurance/report/generate".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "decision": {
                        "type": "string",
                        "description": "The decision: APPROVE, DECLINE, or LOAD"
                    },
                    "reasoning": {
                        "type": "string",
                        "description": "Actuarial reasoning for the decision"
                    },
                    "conditions": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Special conditions or premium loadings"
                    }
                },
                "required": ["decision", "reasoning"]
            }),
        },
    ]
}
