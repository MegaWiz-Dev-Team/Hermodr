# Data Licensing Notice — PrimeKG / DrugBank and Medical Terminologies

Hermodr is an **MCP tool catalog / bridge**. It exposes tools that *query*
knowledge bases (e.g. PrimeKG, SNOMED-mapped data) served by Mimir; it does
**not** contain or redistribute any licensed terminology release data.

The guiding rule (shared across Asgard repos):

> Code that *references* a terminology or knowledge graph is fine to be public.
> The raw *release data* is **not** redistributable and lives only in the
> runtime database, never in source control.

## PrimeKG / DrugBank / terminologies — licenses CLEARED (2026-05-30)

> **Status update 2026-05-30:** All medical KB licenses required for commercial
> deployment are **cleared/obtained** — DrugBank (PrimeKG upstream), SNOMED CT
> Affiliate, LOINC, ICD-10-TM 2017, TPC-Thai. The earlier "commercial gating /
> gate-before-ship" requirement is **no longer in force**; PrimeKG and the other
> KB MCP tools may be exposed in commercial customer deployments.

PrimeKG's graph schema is permissively licensed and is built from sources that
keep their own licenses (notably **DrugBank**, academic + commercial tiers).
Those licenses are now held, so commercial use is permitted. What remains:

- The raw release data is still **never committed to this repository** — it is
  ingested into Mimir's runtime DB from controlled storage. (Unchanged rule.)
- **Recurring obligation:** the **SNOMED CT Affiliate License (clause 6.2)**
  requires upgrading to a new International Edition **within 180 days** of
  release (biannual). Track as recurring maintenance, not a ship gate.
- The most restrictive upstream source license still governs *redistribution*
  (we don't redistribute — we serve queries at runtime).

## Operator responsibility

Operators must keep the appropriate licenses (DrugBank, SNOMED CT, ICD-10-TM,
TMT/TMLT, LOINC, UMLS) current for any data they ingest and expose through these
tools. As of 2026-05-30 these are held for Asgard's deployments. See Mimir's
`DATA_LICENSE.md` for the full table.