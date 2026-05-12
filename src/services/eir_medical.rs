//! Eir Medical MCP tools — RAG + clinical research API wrappers.
//!
//! Distinct from `eir.rs` (which wraps OpenEMR FHIR for the existing Eir gateway).
//! This module exposes **medical knowledge & deep-research tools** that the Eir
//! agent (and any other MCP client) can call to retrieve evidence:
//!
//!   - `pubmed_search`           — NCBI E-utilities (free, 3 req/s without key)
//!   - `pubmed_fetch`            — Fetch full abstract by PMID
//!   - `clinical_trials_search`  — ClinicalTrials.gov API v2
//!   - `fda_drug_search`         — openFDA drug labeling/approval
//!   - `icd10_lookup`            — Clinical Tables NLM ICD-10-CM API
//!   - `drug_interaction_check`  — RxNav / DrugBank free tier
//!   - `medcalc`                 — pure-compute clinical calculators
//!                                 (CHADS2, MELD, Wells, GCS, eGFR, etc.)
//!   - `web_fetch`               — generic URL fetch with rate-limit + html→text
//!
//! Architecture rationale (Asgard Sprint 42 / MultiAgent Architecture v2):
//!   - These tools are STATELESS or wrap external APIs with rate limits.
//!   - Centralizing them in Hermodr lets multiple agents (Eir, future
//!     specialists) share the same rate-limit budget + caching layer.
//!   - Mimir keeps STATEFUL tools (primekg_search, clinical_kb_search,
//!     memvid_search) in-process for low latency.
//!
//! Deployment (Helm):
//!   hermodr-pubmed   SERVICE_NAME=eir_medical UPSTREAM_URL=https://eutils.ncbi.nlm.nih.gov
//!   hermodr-trials   SERVICE_NAME=eir_medical UPSTREAM_URL=https://clinicaltrials.gov
//!   hermodr-fda      SERVICE_NAME=eir_medical UPSTREAM_URL=https://api.fda.gov
//!   ...one per upstream for independent rate-limiting + scaling.
//!
//! Rate limits to respect:
//!   - NCBI E-utilities: 3 req/s without API key, 10 req/s with key
//!   - ClinicalTrials.gov v2: ~50 req/s burst, no documented quota
//!   - openFDA: 240 req/min unauthenticated
//!   - RxNav: ~20 req/s (NIH guideline, no hard cap)

use crate::registry::ToolDefinition;

pub fn tools() -> Vec<ToolDefinition> {
    vec![
        // ─── PubMed (NCBI E-utilities) ─────────────────────────────────────
        ToolDefinition {
            name: "pubmed_search".into(),
            description: "Search PubMed via NCBI esearch.fcgi. Returns up to N PMIDs matching the query. Use for medical literature lookup.".into(),
            method: "GET".into(),
            // NOTE: real upstream path is /entrez/eutils/esearch.fcgi; templated as /esearch
            //       so deployment can split base URL more cleanly.
            path: "/entrez/eutils/esearch.fcgi?db=pubmed&retmode=json&term={term}&retmax={retmax}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "term": { "type": "string", "description": "PubMed query (supports MeSH, field tags, boolean ops)" },
                    "retmax": { "type": "integer", "description": "max PMIDs to return", "default": 20 }
                },
                "required": ["term"]
            }),
        },
        ToolDefinition {
            name: "pubmed_fetch".into(),
            description: "Fetch PubMed article metadata (title, abstract, authors, MeSH) by PMID via efetch.fcgi.".into(),
            method: "GET".into(),
            path: "/entrez/eutils/efetch.fcgi?db=pubmed&rettype=abstract&retmode=xml&id={pmid}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "pmid": { "type": "string", "description": "PubMed ID (numeric, e.g. '21645374')" }
                },
                "required": ["pmid"]
            }),
        },

        // ─── ClinicalTrials.gov v2 ──────────────────────────────────────────
        ToolDefinition {
            name: "clinical_trials_search".into(),
            description: "Search ClinicalTrials.gov for studies matching a condition + intervention. Returns NCT IDs + brief summary.".into(),
            method: "GET".into(),
            path: "/api/v2/studies?query.cond={condition}&query.intr={intervention}&pageSize={page_size}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "condition": { "type": "string", "description": "disease/condition (e.g. 'breast cancer')" },
                    "intervention": { "type": "string", "description": "drug/procedure (e.g. 'tamoxifen')" },
                    "page_size": { "type": "integer", "default": 10 }
                },
                "required": ["condition"]
            }),
        },
        ToolDefinition {
            name: "clinical_trial_detail".into(),
            description: "Fetch detailed ClinicalTrials.gov record by NCT ID — eligibility, phases, outcomes, sponsors.".into(),
            method: "GET".into(),
            path: "/api/v2/studies/{nct_id}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "nct_id": { "type": "string", "description": "NCT ID (e.g. 'NCT04303299')" }
                },
                "required": ["nct_id"]
            }),
        },

        // ─── openFDA (drug labels, approvals, adverse events) ───────────────
        ToolDefinition {
            name: "fda_drug_search".into(),
            description: "Search openFDA drug-label API. Returns approval status, indications, contraindications, dosing.".into(),
            method: "GET".into(),
            path: "/drug/label.json?search={query}&limit={limit}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "drug name or query (e.g. 'metformin', 'openfda.brand_name:Lipitor')" },
                    "limit": { "type": "integer", "default": 5 }
                },
                "required": ["query"]
            }),
        },

        // ─── ICD-10-CM lookup (US/international, English) ──────────────────
        //
        // NLM Clinical Tables. English-only. Use for international research
        // workflows or when a Thai agent needs cross-reference to US codes.
        // For Thai clinical work, prefer `icd10_tm_lookup` (Mimir-backed,
        // anamai-moph-2010 source-version, bilingual th/en).
        ToolDefinition {
            name: "icd10_lookup".into(),
            description: "ICD-10-CM lookup (US/international, English only). Uses NLM Clinical Tables API. For Thai clinical use, prefer `icd10_tm_lookup` instead.".into(),
            method: "GET".into(),
            path: "/api/icd10cm/v3/search?sf=code,name&terms={term}&maxList={max}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "term": { "type": "string", "description": "English search term (e.g. 'diabetes type 2')" },
                    "max": { "type": "integer", "default": 10 }
                },
                "required": ["term"]
            }),
        },

        // ─── ICD-10-TM lookup (Thai, MoPH-anamai) ──────────────────────────
        //
        // Mimir-backed (Sprint 48). Source = anamai-moph-2010 (Department
        // of Health, MoPH) by default; future moph-tm-2017 slot when the
        // B-48a licensing letter completes. Bilingual (th + en labels);
        // BGE-M3 multilingual semantic search via Qdrant `icd10-th`.
        //
        // Use for any Thai clinical workflow — claims to สปสช., HOSxP /
        // MoPH HIS, medical certificates, insurance UW with Thai diagnoses.
        ToolDefinition {
            name: "icd10_tm_lookup".into(),
            description: "Thai ICD-10-TM lookup (MoPH anamai source). Bilingual th/en — pass Thai or English query. Returns code + th_label + en_label + chapter + DRG mapping. Use for Thai clinical workflows, claims, insurance UW.".into(),
            method: "GET".into(),
            path: "/api/v1/icd10/lookup?q={q}&locale={locale}&mode={mode}&limit={limit}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "q": {
                        "type": "string",
                        "description": "Thai or English search term — e.g. 'ไข้หวัดใหญ่', 'pneumonia', or an ICD code like 'J18.9'"
                    },
                    "locale": {
                        "type": "string",
                        "description": "Which label fields to search. Defaults to 'both'.",
                        "enum": ["en", "th", "both"],
                        "default": "both"
                    },
                    "mode": {
                        "type": "string",
                        "description": "Search strategy. Defaults to 'auto' (exact code → prefix → naive fulltext).",
                        "enum": ["auto", "exact", "prefix", "naive"],
                        "default": "auto"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Max results (default 10, max 50).",
                        "default": 10
                    }
                },
                "required": ["q"]
            }),
        },

        // ─── Drug interactions (RxNav free tier) ───────────────────────────
        ToolDefinition {
            name: "drug_interaction_check".into(),
            description: "Check drug-drug interactions via RxNav. Submit list of RxCUIs — returns interaction warnings + severity.".into(),
            method: "GET".into(),
            path: "/REST/interaction/list.json?rxcuis={rxcuis}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "rxcuis": { "type": "string", "description": "space-separated RxCUI list (e.g. '207106 152923')" }
                },
                "required": ["rxcuis"]
            }),
        },
        ToolDefinition {
            name: "rxcui_lookup".into(),
            description: "Find RxCUI for a drug name. Used as a precursor to drug_interaction_check.".into(),
            method: "GET".into(),
            path: "/REST/rxcui.json?name={drug_name}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "drug_name": { "type": "string", "description": "generic or brand name (e.g. 'warfarin')" }
                },
                "required": ["drug_name"]
            }),
        },

        // ─── Generic web fetch ──────────────────────────────────────────────
        // Note: Hermodr forwards path-only by default. For arbitrary URLs, the
        // deployment that registers `web_fetch` should set UPSTREAM_URL='' and
        // use a special proxy mode that reads the URL from the input args.
        // Implementation TODO in proxy.rs — for now this is a placeholder.
        ToolDefinition {
            name: "web_fetch".into(),
            description: "Fetch arbitrary URL with HTML→text extraction. Rate-limited per-domain. Use for HemOnc.org, MedlinePlus, Drugs.com, etc.".into(),
            method: "GET".into(),
            path: "/__hermodr_internal__/web_fetch?url={url}&max_chars={max_chars}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "absolute URL (https only)" },
                    "max_chars": { "type": "integer", "default": 10000, "description": "truncate response to N chars" }
                },
                "required": ["url"]
            }),
        },

        // ─── MedCalc — pure-compute clinical calculators ────────────────────
        // Implemented inline in Hermodr (no upstream HTTP) by interpreting the
        // path as a calculator name. Requires proxy.rs internal handler.
        ToolDefinition {
            name: "medcalc".into(),
            description: "Compute common clinical scores: CHADS2, CHA2DS2-VASc, Wells (DVT/PE), MELD, MELD-Na, GCS, eGFR (CKD-EPI), CrCl (Cockcroft-Gault), BMI, BSA. Inputs vary by formula.".into(),
            method: "POST".into(),
            path: "/__hermodr_internal__/medcalc/{formula}".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "formula": { "type": "string", "enum": ["chads2","cha2ds2vasc","wells_dvt","wells_pe","meld","meld_na","gcs","egfr_ckdepi","crcl_cg","bmi","bsa"] },
                    "inputs": { "type": "object", "description": "formula-specific inputs (e.g. age, weight, creatinine)" }
                },
                "required": ["formula", "inputs"]
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_count() {
        // 11 tools: pubmed×2, ct.gov×2, fda, icd10 (CM), icd10_tm (Thai),
        // rxnav×2, web_fetch, medcalc
        assert_eq!(tools().len(), 11);
    }

    #[test]
    fn test_icd10_tm_tool_present() {
        let names: Vec<_> = tools().into_iter().map(|t| t.name).collect();
        assert!(names.contains(&"icd10_tm_lookup".into()), "icd10_tm_lookup missing");
        assert!(names.contains(&"icd10_lookup".into()), "icd10_lookup (CM) still present");
    }

    #[test]
    fn test_unique_names() {
        let names: Vec<_> = tools().into_iter().map(|t| t.name).collect();
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len(), "duplicate tool name");
    }

    #[test]
    fn test_required_fields_present() {
        for tool in tools() {
            assert!(!tool.name.is_empty());
            assert!(!tool.description.is_empty());
            assert!(!tool.method.is_empty());
            assert!(!tool.path.is_empty());
        }
    }
}
