//! Service tool definitions module.

pub mod bifrost;
pub mod claude_cli;
pub mod eir;
pub mod eir_medical;
pub mod fenrir;
pub mod forseti;
pub mod heimdall;
pub mod huginn;
pub mod loki;
pub mod mimir;
pub mod mjolnir;
pub mod muninn;
pub mod odin;
pub mod ratatoskr;
// Sprint 50 B-50d: Syn — OCR / vision tools (chandra + PaddleOCR + Gemini)
pub mod syn;
// ADR-024: Asgard Analytics — dataset/query/plot tools (mimir-lab)
pub mod analytics;
// asgard_vor: resolve_place + supply-chain graph tools (upstream = asgard-vor)
pub mod vor_tools;
pub mod insurance;
pub mod vardr;
pub mod wazuh;
pub mod yggdrasil;
