//! VoR supply-chain + place-resolver MCP tools.
//!
//! Exposes the asgard-vor dashboard's resolver + in-memory supply-chain graph as MCP tools
//! for the `vor-analyst` agent (tenant asgard_vor). Compute happens upstream in asgard-vor;
//! Hermodr is just the JSON-RPC surface.
//!
//! Deploy: a `hermodr-vor-tools` sidecar with SERVICE_NAME=vor_tools and
//! UPSTREAM_URL=http://vor.asgard.svc:8095.
use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "resolve_place".into(),
            description: "Resolve a Thai place name (จังหวัด/อำเภอ/ตำบล, abbreviation like กทม, or a \
                          typo/partial) to the canonical province/amphur used in the store database. \
                          Input: place_name. Call this before filtering stores by place."
                .into(),
            method: "POST".into(),
            path: "/api/v1/vor/resolve_place".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "place_name": { "type": "string", "description": "Thai place name, e.g. กทม, ดอนเมือง, ปทุมธาน" } },
                "required": ["place_name"]
            }),
        },
        ToolDefinition {
            name: "graph_footprint".into(),
            description: "Supply-chain: how many stores carry each brand (ours vs competitors). No input.".into(),
            method: "POST".into(),
            path: "/api/v1/vor/graph_footprint".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        ToolDefinition {
            name: "graph_cooccurrence".into(),
            description: "Supply-chain: brand pairs that share the same store shelves (co-occurrence). No input.".into(),
            method: "POST".into(),
            path: "/api/v1/vor/graph_cooccurrence".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        ToolDefinition {
            name: "graph_lookalike".into(),
            description: "Supply-chain: stores carrying a competitor but NOT our brand → premium \
                          conversion targets. Input: competitor (optional brand name)."
                .into(),
            method: "POST".into(),
            path: "/api/v1/vor/graph_lookalike".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "competitor": { "type": "string", "description": "competitor brand, e.g. Royal Canin" } }
            }),
        },
        ToolDefinition {
            name: "graph_hub".into(),
            description: "Supply-chain what-if: white-space target count within 5km of each \
                          distribution hub. No input."
                .into(),
            method: "POST".into(),
            path: "/api/v1/vor/graph_hub".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
    ]
}
