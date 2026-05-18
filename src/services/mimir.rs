//! Mimir MCP tools — application layer integration.
//!
//! ocr_extract used to live here briefly in Sprint 50 Day-2 but moved to
//! services/syn.rs once syn-api was extracted into its own service.

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
        ToolDefinition {
            name: "search_pubmed".into(),
            description: "Search PubMed biomedical literature abstracts. Returns up to 50 results with semantic similarity scoring.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/pubmed".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query (drug, disease, symptom, medical topic)" },
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "limit": { "type": "integer", "description": "Max results (1-50, default 10)" }
                },
                "required": ["query", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "search_primekg".into(),
            description: "Search PrimeKG medical knowledge graph. Searches 129k+ drug-disease-gene entities with semantic embeddings.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Entity name or concept (e.g. 'metformin', 'diabetes', 'APOE4')" },
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "limit": { "type": "integer", "description": "Max results (1-50, default 10)" }
                },
                "required": ["query", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "search_clinical_wisdom".into(),
            description: "Search clinical guidelines, protocols, and wisdom from sleep, cardiology, ENT, pediatrics, and CPAP management domains.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/clinical".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Clinical question or topic (e.g. 'sleep apnea treatment', 'CPAP compliance')" },
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "limit": { "type": "integer", "description": "Max results (1-50, default 10)" }
                },
                "required": ["query", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "search_icd10".into(),
            description: "Search ICD-10-TM medical codes (Thai version). Searches 15k+ diagnostic and procedural codes.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/icd10".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "ICD-10 code search or medical term (e.g. 'asthma', 'G47.3')" },
                    "tenant_id": { "type": "string", "description": "Tenant ID" },
                    "limit": { "type": "integer", "description": "Max results (1-50, default 10)" }
                },
                "required": ["query", "tenant_id"]
            }),
        },
        // ─── PrimeKG graph-native tools (Sprint 2 W2.5) ─────────────────────
        //
        // Complement `search_primekg` (semantic vector search) with direct
        // graph operations. These are Cypher-backed against Neo4j; backend
        // endpoints land in a separate PR. See docs at:
        //   Asgard/docs/architecture/agent_rag_graph_solution_architecture.md
        //   docs/reference/PRIMEKG_DATA_REPORT.md
        ToolDefinition {
            name: "primekg_lookup_entity".into(),
            description: "Look up a single PrimeKG entity (drug, disease, phenotype, anatomy, gene/protein, etc.) by name and optional type. Returns canonical entity record with primekg id, name, source, and external cross-references (DrugBank/MONDO/UniProt/etc.). Use when you need the exact entity, not a fuzzy semantic search.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/entity".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Entity name (e.g. 'metformin', 'Type 2 diabetes mellitus'). Case-insensitive; matches en_label or synonyms." },
                    "type": {
                        "type": "string",
                        "description": "Optional entity-type filter. Drug/Disease/Phenotype/Anatomy/GeneProtein/BiologicalProcess/Pathway/CellularComponent/MolecularFunction/Exposure.",
                        "enum": ["Drug", "Disease", "Phenotype", "Anatomy", "GeneProtein", "BiologicalProcess", "Pathway", "CellularComponent", "MolecularFunction", "Exposure"]
                    },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["name", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "primekg_neighbors".into(),
            description: "Return graph neighbors of a PrimeKG entity, optionally filtered by relation type. Use to explore entity connections (e.g. 'what diseases is this drug indicated for', 'what phenotypes is this disease associated with'). Default hop=1; max hop=3 to bound graph traversal cost.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/neighbors".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "entity_id": { "type": "string", "description": "PrimeKG entity_id (preferred) or entity_index. Get from primekg_lookup_entity." },
                    "relation": {
                        "type": "string",
                        "description": "Optional relation filter. indication/contraindication/off-label/drug_drug/disease_protein/drug_protein/protein_protein/disease_phenotype_positive/disease_phenotype_negative/disease_disease/anatomy_protein_present/anatomy_protein_absent/etc."
                    },
                    "hop": {
                        "type": "integer",
                        "description": "Traversal depth (1-3). Default 1. hop=2 returns neighbors-of-neighbors; expensive — use sparingly.",
                        "minimum": 1,
                        "maximum": 3,
                        "default": 1
                    },
                    "limit": { "type": "integer", "description": "Max neighbors per hop (1-100, default 25)", "minimum": 1, "maximum": 100, "default": 25 },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["entity_id", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "primekg_drug_interactions".into(),
            description: "Return known drug-drug interactions for a given drug (PrimeKG drug_drug edges). Each result includes the interacting drug + display_relation describing the interaction mechanism. Used by eir-pharmacy for DDI safety screens — should be invoked before any prescription action.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/drug_interactions".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "drug_id": { "type": "string", "description": "PrimeKG entity_id of the drug (preferred). If you have a drug name, call primekg_lookup_entity first with type='Drug'." },
                    "drug_name": { "type": "string", "description": "Fallback: drug name to look up. Mutually exclusive with drug_id; drug_id takes precedence when both are set." },
                    "severity_filter": {
                        "type": "string",
                        "description": "Filter results by interaction severity (when available). minor/moderate/major/contraindicated.",
                        "enum": ["minor", "moderate", "major", "contraindicated"]
                    },
                    "limit": { "type": "integer", "description": "Max interactions to return (1-100, default 50)", "minimum": 1, "maximum": 100, "default": 50 },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["tenant_id"]
            }),
        },
        ToolDefinition {
            name: "primekg_disease_drugs".into(),
            description: "Return drugs associated with a given disease (PrimeKG indication / off-label / contraindication edges). Splits results by relation so caller can distinguish approved indications from cautions. For clinical decision support, the indication subset is the primary; contraindications are mandatory checks.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/disease_drugs".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "disease_id": { "type": "string", "description": "PrimeKG entity_id of the disease (preferred)." },
                    "disease_name": { "type": "string", "description": "Fallback: disease name to look up. Mutually exclusive with disease_id." },
                    "include_relations": {
                        "type": "array",
                        "description": "Which drug-disease relations to include. Default ['indication']; pass ['indication','contraindication','off-label use'] for full picture.",
                        "items": {
                            "type": "string",
                            "enum": ["indication", "contraindication", "off-label use"]
                        }
                    },
                    "limit": { "type": "integer", "description": "Max drugs per relation (1-100, default 25)", "minimum": 1, "maximum": 100, "default": 25 },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["tenant_id"]
            }),
        },
        ToolDefinition {
            name: "primekg_symptom_to_disease".into(),
            description: "Reverse-lookup from symptoms/phenotypes to candidate diseases (PrimeKG disease_phenotype_positive edges). Multiple symptoms can be passed; results rank diseases by how many of the input symptoms they manifest. Useful for differential diagnosis support; clinical decision still requires Eir specialty review.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/symptom_to_disease".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "symptoms": {
                        "type": "array",
                        "description": "List of symptom/phenotype names or PrimeKG phenotype ids. 1-10 symptoms supported.",
                        "items": { "type": "string" },
                        "minItems": 1,
                        "maxItems": 10
                    },
                    "min_match_count": {
                        "type": "integer",
                        "description": "Minimum number of input symptoms that must match before a disease is returned. Default 1; raise for tighter differential.",
                        "minimum": 1,
                        "default": 1
                    },
                    "limit": { "type": "integer", "description": "Max candidate diseases (1-50, default 20)", "minimum": 1, "maximum": 50, "default": 20 },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["symptoms", "tenant_id"]
            }),
        },
        ToolDefinition {
            name: "primekg_path".into(),
            description: "Find shortest path between two PrimeKG entities (e.g. drug → protein → disease mechanism chain). Returns ordered list of nodes + edges. Use for mechanism-of-action reasoning, adverse-event causal chains, or explaining how an entity connects to another. Capped at 4 hops to avoid combinatorial blow-up.".into(),
            method: "POST".into(),
            path: "/api/v1/knowledge/primekg/path".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "from_id": { "type": "string", "description": "Source PrimeKG entity_id" },
                    "to_id": { "type": "string", "description": "Destination PrimeKG entity_id" },
                    "max_hops": {
                        "type": "integer",
                        "description": "Maximum path length (1-4). Default 3. Longer paths blow up exponentially.",
                        "minimum": 1,
                        "maximum": 4,
                        "default": 3
                    },
                    "max_paths": {
                        "type": "integer",
                        "description": "Maximum number of distinct paths to return (1-10, default 3). Multiple paths can illuminate alternative mechanisms.",
                        "minimum": 1,
                        "maximum": 10,
                        "default": 3
                    },
                    "tenant_id": { "type": "string", "description": "Tenant ID" }
                },
                "required": ["from_id", "to_id", "tenant_id"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        assert_eq!(tools().len(), 11);
    }

    #[test]
    fn test_primekg_graph_tools_present() {
        let tools = tools();
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        for expected in [
            "primekg_lookup_entity",
            "primekg_neighbors",
            "primekg_drug_interactions",
            "primekg_disease_drugs",
            "primekg_symptom_to_disease",
            "primekg_path",
        ] {
            assert!(names.contains(&expected), "missing primekg tool: {expected}");
        }
    }

    #[test]
    fn test_all_tools_have_required_fields() {
        for t in tools() {
            assert!(!t.name.is_empty(), "tool name empty");
            assert!(!t.description.is_empty(), "tool {} description empty", t.name);
            assert!(matches!(t.method.as_str(), "GET" | "POST"), "tool {} bad method", t.name);
            assert!(t.path.starts_with("/api/"), "tool {} bad path", t.name);
            assert!(t.input_schema.is_object(), "tool {} input_schema not an object", t.name);
        }
    }

    #[test]
    fn test_all_post_tools_require_tenant_id() {
        // Tenant scope is non-negotiable for any data tool.
        for t in tools() {
            if t.method != "POST" {
                continue;
            }
            let schema = &t.input_schema;
            let required = schema.get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().any(|v| v.as_str() == Some("tenant_id")))
                .unwrap_or(false);
            assert!(required, "tool {} POST does not require tenant_id", t.name);
        }
    }
}
