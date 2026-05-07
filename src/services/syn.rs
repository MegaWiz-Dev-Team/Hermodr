//! Syn MCP tools — OCR / vision capabilities for Asgard agents.
//!
//! Exposes Syn's 4-tier hybrid OCR (chandra + PaddleOCR locally; Gemini
//! Flash + Pro on opt-in tenants) as a single `ocr_extract` MCP tool. All
//! routing, audit, cost guard, and PHI guards are enforced upstream by
//! syn-api — Hermodr is just the JSON-RPC surface.

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
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
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_extract_tool_present() {
        let t = tools();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].name, "ocr_extract");
        assert_eq!(t[0].method, "POST");
        assert_eq!(t[0].path, "/api/v1/syn/ocr/extract-json");
    }
}
