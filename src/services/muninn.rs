//! Muninn MCP tools — Issue watcher and auto-fixer integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "list_open_issues".into(),
            description: "List openly watched issues currently tracked by Muninn.".into(),
            method: "GET".into(),
            path: "/api/v1/issues".into(),
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
        },
        ToolDefinition {
            name: "forseti_auto_fix".into(),
            description: concat!(
                "Dispatch a Muninn auto-fix from a Forseti test failure. ",
                "Creates a synthetic issue inside Muninn and runs the code agent ",
                "(gemini-cli by default) using the LLM model named in the payload ",
                "(e.g. gemini-3-flash). Used by Forseti dashboard's \"Send to Muninn\" ",
                "button via Bifrost agent dispatch."
            )
            .into(),
            method: "POST".into(),
            path: "/api/forseti-fix".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "required": ["forseti_run_id", "scenario_name", "target_repo"],
                "properties": {
                    "forseti_run_id": {
                        "type": "integer",
                        "description": "Forseti SQLite run id that produced this failure",
                    },
                    "scenario_name": {
                        "type": "string",
                        "description": "Forseti scenario name as it appears in the test yaml",
                    },
                    "phase": {
                        "type": "string",
                        "enum": ["SIT", "UAT"],
                        "description": "Forseti test phase",
                    },
                    "base_url": {
                        "type": "string",
                        "description": "Target service base URL the scenario hit",
                    },
                    "error_message": {
                        "type": "string",
                        "description": "Raw failure message from the test runner",
                    },
                    "test_script_path": {
                        "type": "string",
                        "description": "Repo-relative path to the yaml test script",
                    },
                    "target_repo": {
                        "type": "string",
                        "description": "owner/repo to fix, e.g. MegaWiz-Dev-Team/Syn",
                    },
                    "target_branch": {
                        "type": "string",
                        "default": "main",
                        "description": "Branch the agent should base the fix PR on",
                    },
                    "commit_sha": {
                        "type": ["string", "null"],
                        "description": "Commit SHA the failing run was captured against",
                    },
                    "recommendation_reason": {
                        "type": "string",
                        "description": "Why Forseti classifier routed this to Muninn",
                    },
                    "matched_rule": {
                        "type": "string",
                        "description": "Forseti classifier rule id, e.g. R2_EXPECTED_STATUS_MISMATCH",
                    },
                    "muninn_confidence": {
                        "type": "number",
                        "minimum": 0.0,
                        "maximum": 1.0,
                        "description": "Forseti's confidence in Muninn-routability",
                    },
                    "llm_model": {
                        "type": "string",
                        "default": "gemini-3-flash",
                        "description": "LLM model Muninn passes to gemini-cli via -m flag",
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
    fn test_tool_count() {
        assert_eq!(tools().len(), 2);
    }

    #[test]
    fn forseti_auto_fix_is_post_with_correct_path() {
        let t = tools()
            .into_iter()
            .find(|t| t.name == "forseti_auto_fix")
            .expect("tool present");
        assert_eq!(t.method, "POST");
        assert_eq!(t.path, "/api/forseti-fix");
    }

    #[test]
    fn forseti_auto_fix_schema_requires_core_fields() {
        let t = tools()
            .into_iter()
            .find(|t| t.name == "forseti_auto_fix")
            .unwrap();
        let required = t.input_schema["required"].as_array().unwrap();
        let names: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
        assert!(names.contains(&"forseti_run_id"));
        assert!(names.contains(&"scenario_name"));
        assert!(names.contains(&"target_repo"));
    }

    #[test]
    fn forseti_auto_fix_defaults_to_gemini_3_flash() {
        let t = tools()
            .into_iter()
            .find(|t| t.name == "forseti_auto_fix")
            .unwrap();
        assert_eq!(
            t.input_schema["properties"]["llm_model"]["default"]
                .as_str()
                .unwrap(),
            "gemini-3-flash"
        );
    }
}
