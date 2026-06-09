//! Analytics MCP tools — Asgard Analytics data engine (mimir-lab, ADR-024).
//!
//! Exposes the `asgard_analytics` tenant's data engine as MCP tools for the
//! `analyst-*` agents. All compute (DuckDB query, schema inference, PII gate,
//! row-cap, query timeout, Tyr audit) is enforced upstream by the analytics-api
//! (mimir-lab); Hermodr is just the JSON-RPC surface. Tool names match the
//! `analyst-*` agent tool allowlist exactly (Agent Studio does not validate, so
//! a mismatch silently breaks the call).
//!
//! Deploy: a `hermodr-analytics` sidecar with SERVICE_NAME=analytics and
//! UPSTREAM_URL=<analytics-api base>.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "dataset_list".into(),
            description: "List datasets registered for a tenant in Asgard Analytics. Returns id, \
                          name, source_type, row_count, and PII gate status (pending/clean/flagged)."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/datasets/list".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "tenant_id": { "type": "string", "description": "Tenant ID (e.g. asgard_analytics)" }
                },
                "required": ["tenant_id"]
            }),
        },
        ToolDefinition {
            name: "dataset_profile".into(),
            description: "Profile one dataset: inferred column schema (name/type/nullable), row \
                          count, PII status, and storage URI. Use before querying to know columns."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/datasets/profile".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "dataset_id": { "type": "string", "description": "Dataset id from dataset_list" }
                },
                "required": ["tenant_id", "dataset_id"]
            }),
        },
        ToolDefinition {
            name: "run_sql".into(),
            description: "Run a READ-ONLY SQL query (DuckDB dialect) over registered datasets and \
                          return columns + rows. Only SELECT/WITH/DESCRIBE/SUMMARIZE are allowed; \
                          results are row-capped and time-limited; every call is audited to Tyr."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/query".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "sql": { "type": "string", "description": "Read-only SQL (SELECT/WITH/...). Reference datasets by their table name." },
                    "row_limit": { "type": "integer", "description": "Max rows returned (default 1000)" }
                },
                "required": ["tenant_id", "sql"]
            }),
        },
        ToolDefinition {
            name: "plot".into(),
            description: "Render a chart spec from a read-only SQL query. Returns an Apache ECharts \
                          option object (rendered by the portal) — not an image. Pick chart_type and \
                          the x / y columns produced by the query."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/plot".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "sql": { "type": "string", "description": "Read-only SQL producing the rows to plot" },
                    "chart_type": { "type": "string", "enum": ["bar", "line", "scatter", "pie"], "description": "Chart kind" },
                    "x": { "type": "string", "description": "Column for the category / x axis" },
                    "y": { "type": "string", "description": "Column for the value / y axis" }
                },
                "required": ["tenant_id", "sql", "chart_type", "x", "y"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 4);
    }

    #[test]
    fn tool_names_match_analyst_allowlist() {
        let defs = tools();
        let names: Vec<&str> = defs.iter().map(|t| t.name.as_str()).collect();
        for expected in ["dataset_list", "dataset_profile", "run_sql", "plot"] {
            assert!(names.contains(&expected), "missing tool {expected}");
        }
    }

    #[test]
    fn run_sql_requires_tenant_and_sql() {
        let t = tools().into_iter().find(|t| t.name == "run_sql").unwrap();
        let req = t.input_schema["required"].as_array().unwrap();
        assert!(req.iter().any(|v| v == "tenant_id"));
        assert!(req.iter().any(|v| v == "sql"));
    }
}
