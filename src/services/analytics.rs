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
        // ── spatial (mimir-geo) — geometry is (lat, lng); pure compute, no tenant_id ──
        ToolDefinition {
            name: "geo_distance".into(),
            description: "Great-circle (haversine) distance between two points, in metres.".into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/distance".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "a": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2, "description": "[lat, lng]" },
                    "b": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2, "description": "[lat, lng]" }
                },
                "required": ["a", "b"]
            }),
        },
        ToolDefinition {
            name: "geo_buffer".into(),
            description: "Circular buffer around a point: returns the polygon ring ([lat,lng] vertices) \
                          at radius_m metres (e.g. a catchment area)."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/buffer".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "lat": { "type": "number" },
                    "lng": { "type": "number" },
                    "radius_m": { "type": "number", "description": "Buffer radius in metres" },
                    "segments": { "type": "integer", "description": "Ring vertices (default 32)" }
                },
                "required": ["lat", "lng", "radius_m"]
            }),
        },
        ToolDefinition {
            name: "geo_join".into(),
            description: "Point-in-polygon join: for each point, the index of the first polygon that \
                          contains it (or null). Use to assign points to regions."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/join".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "points": { "type": "array", "items": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 }, "description": "[[lat,lng], ...]" },
                    "polygons": { "type": "array", "items": { "type": "array", "items": { "type": "array", "items": { "type": "number" } } }, "description": "[[[lat,lng], ...outer ring], ...]" }
                },
                "required": ["points", "polygons"]
            }),
        },
        ToolDefinition {
            name: "geo_choropleth".into(),
            description: "Classify numeric values into choropleth classes (0..classes). method = \
                          equal_interval | quantile (default quantile)."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/choropleth".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "values": { "type": "array", "items": { "type": "number" } },
                    "classes": { "type": "integer", "description": "Number of classes (≥1)" },
                    "method": { "type": "string", "enum": ["equal_interval", "quantile"], "description": "default quantile" }
                },
                "required": ["values", "classes"]
            }),
        },
        ToolDefinition {
            name: "geo_h3".into(),
            description: "Aggregate points into Uber H3 hexagons at the given resolution (0–15). \
                          Returns [{cell, count}] — the heatmap binning."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/h3".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "points": { "type": "array", "items": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 }, "description": "[[lat,lng], ...]" },
                    "resolution": { "type": "integer", "description": "H3 resolution 0 (coarse) – 15 (fine)" }
                },
                "required": ["points", "resolution"]
            }),
        },
        ToolDefinition {
            name: "geo_ingest".into(),
            description: "Summarise a GeoJSON document: feature count, geometry-type histogram, bbox, \
                          and property keys (candidate attribute columns)."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/geo/ingest".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "geojson": { "type": "string", "description": "GeoJSON FeatureCollection/Feature/Geometry as a string" }
                },
                "required": ["geojson"]
            }),
        },
        ToolDefinition {
            name: "stats_moran".into(),
            description: "Global Moran's I spatial autocorrelation over points with values, using \
                          binary distance-band weights (≤ threshold_m). ≈+1 clustered, ≈0 random, ≈−1 dispersed."
                .into(),
            method: "POST".into(),
            path: "/api/v1/analytics/stats/moran".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "points": { "type": "array", "items": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 }, "description": "[[lat,lng], ...]" },
                    "values": { "type": "array", "items": { "type": "number" }, "description": "value per point (same length as points)" },
                    "threshold_m": { "type": "number", "description": "neighbour distance band in metres" }
                },
                "required": ["points", "values", "threshold_m"]
            }),
        },
        ToolDefinition {
            name: "stats_nn".into(),
            description: "Mean nearest-neighbour distance (metres) — a point-pattern summary (Clark–Evans style).".into(),
            method: "POST".into(),
            path: "/api/v1/analytics/stats/nn".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "points": { "type": "array", "items": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 }, "description": "[[lat,lng], ...]" }
                },
                "required": ["points"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 12); // 4 tabular + 6 geo_* + 2 stats_*
    }

    #[test]
    fn tool_names_match_analyst_allowlist() {
        let defs = tools();
        let names: Vec<&str> = defs.iter().map(|t| t.name.as_str()).collect();
        for expected in [
            "dataset_list", "dataset_profile", "run_sql", "plot",
            "geo_distance", "geo_buffer", "geo_join", "geo_choropleth", "geo_h3", "geo_ingest",
            "stats_moran", "stats_nn",
        ] {
            assert!(names.contains(&expected), "missing tool {expected}");
        }
    }

    #[test]
    fn geo_paths_are_under_analytics() {
        for t in tools().iter().filter(|t| t.name.starts_with("geo_") || t.name.starts_with("stats_")) {
            assert!(t.path.starts_with("/api/v1/analytics/"), "bad path for {}: {}", t.name, t.path);
            assert_eq!(t.method, "POST");
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
