# Functional capability ledger

Status: binding coordination ledger, 2026-09-09. This is the functional inventory for the active Quire/Filament workstreams. It records **where each capability is authored, who owns its next decision or delivery, and whether the capability is defined**. It is not a claim that a Draft requirement is implemented, qualified, or accepted. The coordinating ticket is [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9).

## Reading the ledger

`Defined` means the intended behavior is described sufficiently to assign a delivery. `Ambiguous` means a needed behavior is visible but its owning source or semantics have not been selected; it must not be implemented by implication. `Deferred` is an intentional exclusion from the first native profile. `Missing` means no active design provides the capability.

An editable clause has exactly one source language: native Quire. OCL, SysML v2 and FRETish are output-only mappings. The shared domain model is not a second clause language: it supplies declarations consumed by the native language.

| ID | Capability and functional boundary | Source authority / owner | Current basis | Status | Next accountable work |
| --- | --- | --- | --- | --- | --- |
| CAP-001 | Select and identify the sole editable formal-clause profile. | Native Quire language — A | FS01; quire-spec-language #1/#2 | Defined | A completes parser/compiler admission and qualification. |
| CAP-002 | Bind clauses to one declared model authority, exports and typed declarations. | Filament producer model/configuration — D; native package linking — A; existing extraction history — C | FCD #95 baseline 1.2; quire-specification FR-030/035; quire-spec-language #3; contract-ir #54 | Defined, producer/consumer implementation pending | D's reviewed producer contract supplies the authoritative typed model/configuration selection and A rejects unbound/ambiguous declarations. Any new adapter realization is deliberately unassigned until the accepted producer/schema boundary names its implementation route; C's current assurance campaign is not that route. |
| CAP-003 | Preserve source bytes, revision, digest and clause/expression spans through extraction and lowering. | Historical extraction/source correspondence — C; producer/native correspondence — D; native locations — A | FR-003; FCD #95 producer correspondence; quire-rs #9 | Defined, delivery incomplete | C preserves the already-assigned extraction/source correspondence. D supplies the accepted producer-object/native-artifact correspondence and A retains exact native locations. New language or adapter delivery is not implied by that historical assignment. |
| CAP-004 | Evaluate optional values, Boolean control, bounded integers, finite ordered sequences, references, populations, operations and frames. | Native semantics — A | FR-005 through FR-013; state-core profile and state-semantics | Defined, delivery incomplete | A implements and qualifies the bounded native profile. |
| CAP-005 | Distinguish a field's required presence, optional absence, explicit null, and multiplicity. | Shared domain-model producer contract — D; native consumer boundary — A; historical extraction context — C | [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95) baseline 1.2 draft and PR #97; owner ruling presence/null/invalid rule | **Defined; acceptance and implementation pending** | D's selected contract carries authored presence independently of multiplicity. After acceptance, D owns the named producer/schema proposal and A the native consumer boundary; any adapter implementation allocation remains explicit future work, not an automatic C assignment. #93 resumes only against that accepted boundary. No native syntax feature is presumed. |
| CAP-006 | Bind typed native clauses to executable IR without making the IR another authored clause authority. | Contract-IR binding — B/C; native lowering — A | contract-ir #50/#54; FR-002/003 | Defined, delivery incomplete | B/C complete binding contract and A lowers native nodes against it. |
| CAP-007 | Exchange exact wire identities, canonical domains and explicit migrations. | Shared interchange — A/B/C by artifact role | FR-014 through FR-018; FS05; contract-ir #58 | Defined, delivery incomplete | Owners finish strict-reader, producer and migration obligations without collapsing identity dimensions. |
| CAP-008 | Emit OCL, SysML v2 and FRETish as non-authoritative output mappings, with explicit loss/refusal evidence. | Mapping/emitter owners — F/D/E | FS06; contract-ir #52/#55/#56/#57; FCD #37 | Defined, delivery incomplete | Mapping owners implement Rust emitters and own semantic/loss evidence; no foreign runtime is a qualification dependency. |
| CAP-009 | Represent scalar `any` in IR and project v1.2 to v1.1 only when information is preserved; otherwise refuse with loss. | Contract IR — D | FCD #93; existing v1.1 inability state #78 | Defined, delivery incomplete | D completes the IR v1.2 change after CAP-005 is decided. |
| CAP-010 | Preserve field and relationship loci from source models. | Extraction — C | quire-rs #420 and #418 | Defined, known gap | C resolves the existing tickets; no duplicate delivery ticket is needed. |
| CAP-011 | Author and evaluate bounded temporal requirements, including future/past MLTL profiles and their typed predicates. | Native Quire temporal profile — E; shared grammar/integration — A; TL is internal representation/evaluator | [Current temporal definition](../proposals/quire-v1/definitions/temporal-common.md), draft 1-draft.3; FR-090–095; tl-syntax#1 roadmap; tl-mltl | **Defined (draft); implementation and acceptance pending** | E completes selected temporal rules and all-clock observation correspondence with D/F; A integrates the native profile and its fail-closed lowering boundary. `tl-*` is not a competing authored source language. |
| CAP-012 | Author, model, and evaluate protocol/choreography behavior beyond single-invocation state clauses. | Native Quire protocol/choreography profile — B; shared grammar/integration — A; observation/progress — F | [Current protocol definition](../proposals/quire-v1/definitions/protocol-finite.md), draft 1-draft.4; choreography surface; FR-049–062 | **Defined (draft); implementation and acceptance pending** | B completes protocol/compensation result contracts, F supplies observation/progress correspondence, and A integrates the native profile. No second clause authority or unowned capability plan is required. |
| CAP-013 | Compose state, temporal, protocol/choreography, and future profiles into one official language without profile-specific identity or source-authority drift. | Native Quire common package/profile contract — A; typed ecosystem bindings — D; family contracts — B/E/F | quire-specification composed grammar, package contract, and FR-030–049/FR-130 | **Defined (draft); implementation and acceptance pending** | A selects versioned composition/profile rules; B/D/E/F supply their reserved cross-family contracts. Implementations consume the accepted composed contract rather than inventing per-profile identity or source rules. |

## Binding decisions

1. **No inferred language feature.** A downstream IR or extraction need does not create native Quire syntax. CAP-005 is a shared-model/extraction decision, unless its owner explicitly elects native surface syntax as the authoring authority.
2. **Presence and cardinality are independent.** Required presence answers whether a field member must be authored; multiplicity answers how many values are permitted when it is present. For example, a required `0..*` field may be present and empty, while an optional `1..*` field may be absent but must have at least one value when present. Explicit null is a distinct channel if the selected source contract supports it.
3. **Loss is not a conversion.** If the selected source representation cannot carry a distinction, the implementation must preserve it or report a named refusal/loss. It may not derive independent authored presence from a v1.1 field shape.
4. **One approved crosswalk, not a second taxonomy.** The v1 capability/roadmap crosswalk is the coordination index for affected delivery tickets. A ticket records its source authority, consumer(s), supported/refused outcome and owning workstream where that crosswalk applies; this ledger does not impose a separate mandatory CAP-ID citation rule.

## Required routing

| Workstream | Owns now | Must not absorb |
| --- | --- | --- |
| A — native Quire / `quire-specification`, `quire-spec-language` | Common grammar, package composition, typing/evaluation integration and native source identity. | A parallel semantic model, a second clause source profile, or inferred field-presence syntax. |
| B — protocol/verification | Native protocol/choreography semantics, compensation/result contracts and portable verification/evidence contracts. | Domain declaration producer semantics. |
| C — `quire-rs` extraction/adapters | Existing extraction, source correspondence, and its bounded assurance campaign. | Selecting new domain semantics or receiving a new producer/schema or adapter delivery assignment by implication. |
| D — Filament/contract IR/frontends/backends | Shared model, authored-presence, producer/native correspondence, configuration and IR semantics; frontends/backends. | Treating an output mapping as a native-language frontend. |
| E — temporal | Native temporal profiles, clock/history semantics and temporal correspondence. | A second editable temporal clause language. |
| F — observation | Correlation, completeness, progress and observation/window correspondence. | Replacing model or protocol authority. |

## Correction: product scope versus development slice

The state-finite profile is a development slice, not the product boundary. The
official native Quire language exists precisely because the program needs a
single source language with the combined feature surface that no adopted
external language provides. Its state, temporal, protocol/choreography and
future profiles may have distinct semantics and evaluators, but they are native
Quire capabilities with one authored-source authority.

`tl-syntax` and `tl-mltl` are valuable temporal syntax/representation and
evaluation work. They can be consumed behind a versioned native-Quire lowering
boundary; they cannot remain a second user-authored formal-clause language.
Likewise, FRETish remains an export-only mapping and cannot substitute for a
native temporal or choreography profile.

## State-core admission rule check

The carried-forward rules cover the **state-core** semantic surface in CAP-001 through CAP-004 and CAP-006 through CAP-010: finite numeric domains; absence/null/invalid; typed acyclic records and references; bounded ordered sequences; Boolean roots/no calls; source authority; operation instants; and explicit finite bounds. CAP-005 now has D's selected authored-presence baseline, though it is not accepted or implemented. CAP-011 through CAP-013 now have allocated, reviewed draft temporal, protocol/choreography and composition contracts; their remaining gaps are acceptance, cross-family corrections and implementation, not absent design ownership or intentional product exclusions.

## Change log

- 2026-09-09: created after the native-source-profile ruling. Initial routing: [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9) maintains this ledger, and [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95) owns the CAP-005 decision that blocks FCD #93's presence delivery.
- 2026-09-09: corrected after program-roadmap inspection. State-finite was incorrectly treated as product scope. CAP-011 (temporal), CAP-012 (protocol/choreography), and CAP-013 (profile composition) are now in-scope design work, not deferred features.
- 2026-09-10: FCD #95 selected the provisional baseline 1.2 source contract for CAP-005: presence is authored independently of multiplicity, explicit null remains distinct, legacy v1.1 cannot claim the new source information, and lossy projections refuse. The selection is specified but not yet reviewed, implemented, or promoted; FCD #93 remains preserved pending the named implementation boundary.
- 2026-09-10: corrected against the initial native Quire v1 composed cycle. CAP-011–013 are no longer recorded as missing/unowned: their temporal, protocol/choreography and composition candidates have named A/B/D/E/F owners and remain draft, unaccepted and unimplemented. CAP-002/003/005 distinguish C's historical extraction and current assurance work from the unassigned new producer/schema adapter route; D owns the producer proposal and A the native consumer boundary after acceptance. The CAP index is an approved crosswalk, not a second mandatory ticket taxonomy.
