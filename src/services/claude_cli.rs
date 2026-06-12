//! Claude CLI tool — invoke Claude Code CLI through TMUX sessions.
//!
//! Enables agents (Odin, Frigg) to execute Claude Code commands in isolated
//! TMUX windows, useful for code refactoring, analysis, and generation tasks.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "claude_cli_invoke".into(),
            description: "Execute Claude Code CLI in a TMUX session. Run Claude on a specific code task with context.".into(),
            method: "POST".into(),
            path: "/tools/claude_cli/invoke".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "task": {
                        "type": "string",
                        "description": "The task description (e.g., 'Fix error handling in overseer.rs')"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory (e.g., '/Users/mimir/Developer/Bifrost')"
                    },
                    "context": {
                        "type": "string",
                        "description": "Additional context or constraints for Claude"
                    },
                    "model": {
                        "type": "string",
                        "enum": ["haiku", "sonnet", "opus"],
                        "description": "Model to use (default: opus for complex tasks)"
                    },
                    "timeout_seconds": {
                        "type": "integer",
                        "description": "Max execution time in seconds (default: 300)",
                        "minimum": 10,
                        "maximum": 3600
                    }
                },
                "required": ["task", "cwd"]
            }),
        },
        ToolDefinition {
            name: "claude_cli_status".into(),
            description: "Check the status of an active Claude CLI session.".into(),
            method: "GET".into(),
            path: "/tools/claude_cli/status/{session_id}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "TMUX session ID (e.g., 'asgard_asgard_platform_22')"
                    }
                },
                "required": ["session_id"]
            }),
        },
        ToolDefinition {
            name: "claude_cli_list_sessions".into(),
            description: "List all active Claude CLI TMUX sessions for the current tenant.".into(),
            method: "GET".into(),
            path: "/tools/claude_cli/sessions".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 3);
    }

    #[test]
    fn test_claude_cli_invoke_schema() {
        let tools = tools();
        let invoke_tool = &tools[0];
        assert_eq!(invoke_tool.name, "claude_cli_invoke");
        assert_eq!(invoke_tool.method, "POST");
    }

    #[test]
    fn test_claude_cli_status_schema() {
        let tools = tools();
        let status_tool = &tools[1];
        assert_eq!(status_tool.name, "claude_cli_status");
        assert_eq!(status_tool.method, "GET");
    }
}
