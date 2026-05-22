# Data Licensing Notice — PrimeKG / DrugBank and Medical Terminologies

Hermodr is an **MCP tool catalog / bridge**. It exposes tools that *query*
knowledge bases (e.g. PrimeKG, SNOMED-mapped data) served by Mimir; it does
**not** contain or redistribute any licensed terminology release data.

The guiding rule (shared across Asgard repos):

> Code that *references* a terminology or knowledge graph is fine to be public.
> The raw *release data* is **not** redistributable and lives only in the
> runtime database, never in source control.

## PrimeKG / DrugBank — commercial gating

PrimeKG's graph schema is permissively licensed, but it is **built from sources
that keep their own licenses**. Notably, **DrugBank** content is under an
academic license and **commercial use requires a paid DrugBank license**.

Consequences for any deployment using the PrimeKG MCP tools:

- The PrimeKG/DrugBank-derived data is ingested into Mimir's database from
  controlled storage; it is never committed to this repository.
- For **commercial customer deployments**, DrugBank-derived content must be
  gated at request-admission via the platform's
  `ai_models.metadata.commercial_use` / data-source enforcement, or replaced
  with a commercially-licensed source.
- The most restrictive upstream source license governs redistribution.

## Operator responsibility

Operators are responsible for obtaining the appropriate licenses (DrugBank,
SNOMED CT, ICD-10-TM, TMT/TMLT, LOINC, UMLS) for any data they ingest and
expose through these tools. See Mimir's `DATA_LICENSE.md` for the full table.