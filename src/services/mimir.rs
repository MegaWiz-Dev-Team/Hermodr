//! Mimir MCP tools — application layer integration.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "list_agents".into(),
            description: "List available agents configured in the Mimir database.".into(),
            method: "GET".into(),
            path: "/api/v1/agents".into(),
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
        },
        // Sprint 50 B-50d: Syn 4-tier OCR exposed through Hermodr so any
        // agent (Eir clinical, Mimir Assistant, Bifrost runtime) calls the
        // same tool. Routes through the smart router → chandra | paddleocr |
        // gemini-3-flash | gemini-3.1-pro per tenant policy.
        ToolDefinition {
            name: "ocr_extract".into(),
            description:
                "Extract text from a medical document image using Syn's 4-tier hybrid OCR \
                 (chandra + PaddleOCR locally, optional Gemini Flash/Pro on opt-in tenants). \
                 The smart router picks the engine; cloud tiers are blocked on phi_strict tenants. \
                 Image must be base64-encoded PNG/JPEG/PDF (≤20MB). Returns extracted text plus \
                 audit_id for traceability."
                    .into(),
            method: "POST".into(),
            path: "/api/v1/syn/ocr/extract-json".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "image_base64": {
                        "type": "string",
                        "description": "Base64-encoded image bytes (no data: URI prefix)."
                    },
                    "filename": {
                        "type": "string",
                        "description": "Original filename — used for MIME detection. Defaults to upload.png.",
                        "default": "upload.png"
                    },
                    "doc_type": {
                        "type": "string",
                        "description": "Document type hint for the smart router.",
                        "enum": ["handwriting", "printed_thai", "mixed"]
                    },
                    "engine": {
                        "type": "string",
                        "description": "Manual engine override. Avoid unless the agent has a strong reason — the router otherwise picks based on doc_type and tenant policy.",
                        "enum": ["chandra-local", "paddleocr-local", "gemini-3-flash", "gemini-3.1-pro"]
                    },
                    "high_stakes": {
                        "type": "boolean",
                        "description": "Curator-only flag; routes to Gemini Pro when tenant has cloud_pro opt-in.",
                        "default": false
                    },
                    "hint_lang": {
                        "type": "string",
                        "description": "Optional language hint forwarded to the local sidecars (e.g. 'th', 'en')."
                    }
                },
                "required": ["image_base64"]
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
    fn test_ocr_extract_tool_shape() {
        let t = tools().into_iter().find(|t| t.name == "ocr_extract").unwrap();
        assert_eq!(t.method, "POST");
        assert_eq!(t.path, "/api/v1/syn/ocr/extract-json");
        let required = t.input_schema["required"].as_array().unwrap();
        assert!(required.iter().any(|v| v == "image_base64"));
    }
}
