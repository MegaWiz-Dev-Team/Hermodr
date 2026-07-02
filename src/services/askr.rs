//! Askr — Faculty exam-authoring studio MCP tools.
//!
//!   hermodr-askr   SERVICE_NAME=askr UPSTREAM_URL=http://askr:8095
//!
//! Wraps the Askr REST API so Eir (via Bifrost) can drive the whole WFME authoring
//! flow as tools: browse the item bank → create an exam + blueprint → author MEQ/OSCE
//! → add items → standard-set → publish/release → run education-research analytics.
//!
//! Identity: write endpoints are `x-askr-faculty`-scoped. Hermodr injects that header
//! from `ASKR_FACULTY_ID` (default "1" = admin) so agent calls carry a faculty identity.
use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "askr_browse_catalog".into(),
            description: "Browse the OSCE item bank (246 virtual-patient stations) filtered by specialty. Returns cases with case_id, EPAs, presentation, difficulty, media.".into(),
            method: "GET".into(),
            path: "/api/catalog?specialty={specialty}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "specialty": { "type": "string", "description": "e.g. cardiology, pulmonology" } },
                "required": ["specialty"]
            }),
        },
        ToolDefinition {
            name: "askr_list_exams".into(),
            description: "List all exams in Askr (id, name, kind, status).".into(),
            method: "GET".into(),
            path: "/api/exams".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        ToolDefinition {
            name: "askr_get_exam".into(),
            description: "Get an exam's detail and its items.".into(),
            method: "GET".into(),
            path: "/api/exams/{exam_id}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" } },
                "required": ["exam_id"]
            }),
        },
        ToolDefinition {
            name: "askr_validate_exam".into(),
            description: "Check an exam's WFME blueprint coverage: filled cells, gaps, balance, and whether it is publishable.".into(),
            method: "GET".into(),
            path: "/api/exams/{exam_id}/validate".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" } },
                "required": ["exam_id"]
            }),
        },
        ToolDefinition {
            name: "askr_create_exam".into(),
            description: "Create an exam with a WFME blueprint. kind = 'osce' | 'meq'. blueprint is a list of targets {epa, presentation, target_n}. Coordinator/admin only.".into(),
            method: "POST".into(),
            path: "/api/exams".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "kind": { "type": "string", "enum": ["osce", "meq"] },
                    "blueprint": {
                        "type": "array",
                        "items": { "type": "object", "properties": {
                            "epa": { "type": "string" }, "presentation": { "type": "string" }, "target_n": { "type": "integer" }
                        }, "required": ["epa"] }
                    }
                },
                "required": ["name", "kind", "blueprint"]
            }),
        },
        ToolDefinition {
            name: "askr_add_exam_item".into(),
            description: "Add an item to an exam. item_id may be a bank case_id, an authored MEQ id, or an authored OSCE id. Enforced to the caller's specialty.".into(),
            method: "POST".into(),
            path: "/api/exams/{exam_id}/items".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" }, "item_id": { "type": "string" } },
                "required": ["exam_id", "item_id"]
            }),
        },
        ToolDefinition {
            name: "askr_author_meq".into(),
            description: "Author a MEQ item (net-new). steps is a list of {question, model_answer, key_features[]}. Specialty-scoped.".into(),
            method: "POST".into(),
            path: "/api/meq".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string" }, "specialty": { "type": "string" },
                    "presentation": { "type": "string" },
                    "epas": { "type": "array", "items": { "type": "string" } },
                    "stem": { "type": "string" },
                    "steps": { "type": "array", "items": { "type": "object" } }
                },
                "required": ["id", "specialty"]
            }),
        },
        ToolDefinition {
            name: "askr_author_osce".into(),
            description: "Author an OSCE station (spec §7.4): scenario + candidate task + examiner rubric {checklist:[{item,weight}], global_rating:{anchors[]}}. May clone a bank case via source_case. Specialty-scoped.".into(),
            method: "POST".into(),
            path: "/api/osce".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string" }, "specialty": { "type": "string" },
                    "presentation": { "type": "string" },
                    "epas": { "type": "array", "items": { "type": "string" } },
                    "scenario": { "type": "string" }, "task": { "type": "string" },
                    "rubric": { "type": "object" }, "source_case": { "type": "string" }
                },
                "required": ["id", "specialty"]
            }),
        },
        ToolDefinition {
            name: "askr_list_meq".into(),
            description: "List authored MEQ items in the caller's specialties.".into(),
            method: "GET".into(),
            path: "/api/meq".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        ToolDefinition {
            name: "askr_list_osce".into(),
            description: "List authored OSCE stations in the caller's specialties.".into(),
            method: "GET".into(),
            path: "/api/osce".into(),
            input_schema: serde_json::json!({ "type": "object", "properties": {} }),
        },
        ToolDefinition {
            name: "askr_set_angoff".into(),
            description: "Record an Angoff rating (borderline-candidate pass probability 0..1) for an item, by a judge.".into(),
            method: "POST".into(),
            path: "/api/exams/{exam_id}/angoff".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "exam_id": { "type": "integer" }, "item_id": { "type": "string" },
                    "judge_id": { "type": "integer" }, "prob": { "type": "number" }
                },
                "required": ["exam_id", "item_id", "judge_id", "prob"]
            }),
        },
        ToolDefinition {
            name: "askr_standard_setting".into(),
            description: "Compute the Angoff cut score for an exam (cut, SEM, judge count, sufficiency ≥3 judges).".into(),
            method: "POST".into(),
            path: "/api/exams/{exam_id}/standard-setting".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" } },
                "required": ["exam_id"]
            }),
        },
        ToolDefinition {
            name: "askr_publish_exam".into(),
            description: "Publish an exam to review status (requires blueprint coverage complete). Coordinator/admin only.".into(),
            method: "POST".into(),
            path: "/api/exams/{exam_id}/publish".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" } },
                "required": ["exam_id"]
            }),
        },
        ToolDefinition {
            name: "askr_release_exam".into(),
            description: "Release the immutable, checksummed exam bundle for a cohort (requires review + standard-setting + full coverage). Freezes the Box artifact. Coordinator/admin only.".into(),
            method: "POST".into(),
            path: "/api/exams/{exam_id}/release".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "exam_id": { "type": "integer" }, "cohort": { "type": "string" } },
                "required": ["exam_id"]
            }),
        },
        ToolDefinition {
            name: "askr_run_analytics".into(),
            description: "Run a read-only SQL query for education research/analytics (mimir-analytic, tenant asgard_medical). Returns columns + rows.".into(),
            method: "POST".into(),
            path: "/api/analytics/query".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "query": { "type": "string", "description": "read-only SQL" } },
                "required": ["query"]
            }),
        },
    ]
}
