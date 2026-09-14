# Functional capability ledger

Status: binding coordination ledger, reconciled 2026-09-14. This is the functional inventory for the active Quire/Filament workstreams. It records **where each capability is authored, who owns its next decision or delivery, and whether the capability is defined, implemented, or still open**. Each completion is tied to merged evidence; a specification or partial implementation is not qualification. The coordinating ticket is [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9).

## Reading the ledger

`Defined` means the intended behavior is described sufficiently to assign a delivery. `Implemented` names a merged implementation and does not imply broader qualification. `Partial` names merged behavior with an open owning ticket for the remainder. `Ambiguous` means a needed behavior is visible but its owning source or semantics have not been selected; it must not be implemented by implication. `Deferred` is an intentional exclusion from the first native profile. `Missing` means no active design provides the capability.

An editable clause has exactly one source language: native Quire. OCL, SysML v2 and FRETish are output-only mappings. The shared domain model is not a second clause language: it supplies declarations consumed by the native language.

| ID | Capability and functional boundary | Source authority / owner | Current basis | Status | Next accountable work |
| --- | --- | --- | --- | --- | --- |
| CAP-001 | Select and identify the sole editable formal-clause profile. | Native Quire language — A | FS01; AD-003; Contract-IR ADR-0053/PR #72; quire-spec-language #1/#2; [SL `v0.2.0`](https://github.com/agent-ix/quire-spec-language/releases/tag/v0.2.0) | **Defined; historical slice implemented** — FS01 accepts the sole source-language charter and composed identity hierarchy; individual semantic profiles remain independently adopted/implemented | A proceeds through FS02–FS06 without changing released historical meanings or overstating profile/backend support. |
| CAP-002 | Bind clauses to one declared model authority, exports and typed declarations. | Filament producer model/configuration — D; native package linking — A; existing extraction history — C | [FCD #95 / PR #99](https://github.com/agent-ix/filament-core-data/pull/99) at `4042882`; quire-specification FR-030/035 and PRs #38/#41/#42; closed SL #35; [QSL #93 / PR #94](https://github.com/agent-ix/quire-spec-language/pull/94) at `0c83dcc` | **Partial** — the static Producer 1.2 bundle and exact native adapter are accepted and implemented; protocol/observation assessment consumers remain open | Protocol/observation consumers adopt FR-287/288 and FR-262 under their existing lanes without inventing identities from binding slots or heuristic relationships. The static gate does not claim assessment observations, progress or closure. |
| CAP-003 | Preserve source bytes, revision, digest and clause/expression spans through extraction and lowering. | Historical extraction/source correspondence — C; producer/native correspondence — D; native locations — A | FR-003; [FCD #95 / PR #99](https://github.com/agent-ix/filament-core-data/pull/99); quire-rs #9; SL #68/#86; [QSL #93 / PR #94](https://github.com/agent-ix/quire-spec-language/pull/94) | **Partial** — Producer 1.2 loci/correspondence and native multi-unit refusal loci are implemented; historical extraction gaps remain open | C retains its existing source-correspondence tickets; A preserves the landed native loci and exact accepted producer selection. |
| CAP-004 | Evaluate optional values, Boolean control, bounded integers, finite ordered sequences, references, populations, operations and frames. | Native semantics — A | FS03; FR-033/034 and FR-038–046; accepted state-core/query/graph closure; closed SL #36/#37/#83/#84 | **Defined; partially implemented** — exact state semantics and portable vectors are accepted; native scalar/query/graph admission and bounded-scalar verification are implemented | qcir #69 adds exact definedness/object/graph/collection backend parity; standard matrix rows remain Planned until executable evidence binds them. |
| CAP-005 | Distinguish a field's required presence, optional absence, explicit null, and multiplicity. | Shared domain-model producer contract — D; native consumer boundary — A; historical extraction context — C | [FCD #95 / PR #99](https://github.com/agent-ix/filament-core-data/pull/99), FR-106, and [QSL #93 / PR #94](https://github.com/agent-ix/quire-spec-language/pull/94) | **Defined; static producer and native consumer implemented** — presence, multiplicity, null and default are independent authored axes; unrepresentable loss refuses | FCD #93 retains Semantic IR/frontend delivery and runtime-value corpus work. No native field-presence syntax or assessment support is presumed from the static bundle. |
| CAP-006 | Bind typed native clauses to executable IR without making the IR another authored clause authority. | Contract-IR binding — Agent E/current qcir owners; native lowering — A | contract-ir #50/#54; SL #83; qcir #64/#68/#69/#70/#71 | **Partial** — Boolean/integer/state-scalar lowering is implemented | qcir #69 owns definedness/object/graph/collection Kani; Agent E owns current #64/#68/#70/#71 bridge work and consumes Agent B's TL merges without editing TL-* lanes. |
| CAP-007 | Exchange exact wire identities, canonical domains and explicit migrations. | Shared interchange — A/B/C by artifact role | FR-014 through FR-018; FS05; QSpec PR #8 `8ab058b`; QSL #40/#69; protocol #48 / PR #52 `5ce7978` | **Defined; partially implemented** — FS05 accepts the strict reference/canonical/migration contract; compiled-protocol `/2`, source-map/domain controls and an independent strict `/2` consumer are implemented | Consumer-specific matrices, protocol-v1 regression #11 and public registration remain independently owned; no similar bytes or version fallback imply equivalence. |
| CAP-008 | Emit OCL, SysML v2 and FRETish as non-authoritative output mappings, with explicit loss/refusal evidence. | Mapping/emitter owners — F/D/E | FS06; contract-ir #52/#55/#56/#57; FCD #37 | Defined, delivery incomplete | Mapping owners implement Rust emitters and own semantic/loss evidence; no foreign runtime is a qualification dependency. |
| CAP-009 | Represent scalar `any` in IR and project v1.2 to v1.1 only when information is preserved; otherwise refuse with loss. | Contract IR — D | FCD #93; existing v1.1 inability state #78 | Defined, delivery incomplete | D completes the IR v1.2 change after CAP-005 is decided. |
| CAP-010 | Preserve field and relationship loci from source models. | Extraction — C | quire-rs #420 and #418 | Defined, known gap | C resolves the existing tickets; no duplicate delivery ticket is needed. |
| CAP-011 | Author and evaluate bounded temporal requirements, including future/past MLTL profiles and their typed predicates. | Native Quire temporal profile — standard/native owners; shared grammar/integration — A; TL-* libraries — Agent B; Contract-IR bridge — Agent E | FR-090–095; closed SL #38; [specification #40 / PR #44](https://github.com/agent-ix/quire-specification/pull/44); [TL #52](https://github.com/agent-ix/tl-syntax/issues/52); merged qcir PR #67; open qcir #64/#68/#70/#71 | **Partial** — native compiler admission, exact temporal/observation boundary authority, formula-v2 past syntax and the predicate-projection contract are implemented or accepted at their owning boundaries | Agent B remains exclusively on TL-* Plan-010; Agent E consumes merged TL interfaces for qcir #64/#68/#70/#71; observation implements the accepted progress/history facts. |
| CAP-012 | Author, model, and evaluate protocol/choreography behavior beyond single-invocation state clauses. | Native Quire protocol/choreography standard — QS/FS owners; shared grammar/integration — A; observation/progress — F; consumer implementation — protocol #14 | FR-049–062; FR-287/288; closed SL #39/#40; [specification PR #38](https://github.com/agent-ix/quire-specification/pull/38); [specification #28 / PR #41](https://github.com/agent-ix/quire-specification/pull/41); [specification #31 / PR #42](https://github.com/agent-ix/quire-specification/pull/42); [specification #40 / PR #44](https://github.com/agent-ix/quire-specification/pull/44); [protocol #14](https://github.com/agent-ix/quire-protocol/issues/14) | **Partial** — native producer, canonical result vocabulary, exact protocol identities, temporal/observation/result authorities and channel/graph/relationship/compensation rules exist; protocol/observation consumer qualification remains open | Protocol #4 consumes FR-216/220, #5 consumes FR-233/237/239, and #12 consumes FR-262 plus the accepted #40 observation/temporal inputs. Agent B is not assigned this lane. |
| CAP-013 | Compose state, temporal, protocol/choreography, and future profiles into one official language without profile-specific identity or source-authority drift. | Native Quire common package/profile contract — A; typed ecosystem bindings — D; protocol/TL/observation family owners | PR #15; AD-001/AD-003; FS02; FS03 state closure; FS05 interchange closure; specification PRs #38/#41/#42/#44; [FCD PR #99](https://github.com/agent-ix/filament-core-data/pull/99); closed SL #35–#40 and [QSL PR #94](https://github.com/agent-ix/quire-spec-language/pull/94); [SL `v0.2.0`](https://github.com/agent-ix/quire-spec-language/releases/tag/v0.2.0) | **Partial** — FS02's shared authorities, FS03's state semantics and FS05's interchange/evolution semantics are accepted; temporal/protocol/export profile closeout and assessment consumers remain open | Complete FS04 and FS06 plus the existing Kani, temporal, observation and protocol reference paths tracked by QS #1. |

## Binding decisions

1. **No inferred language feature.** A downstream IR or extraction need does not create native Quire syntax. CAP-005 is a shared-model/extraction decision, unless its owner explicitly elects native surface syntax as the authoring authority.
2. **Presence and cardinality are independent.** Required presence answers whether a field member must be authored; multiplicity answers how many values are permitted when it is present. For example, a required `0..*` field may be present and empty, while an optional `1..*` field may be absent but must have at least one value when present. Explicit null is a distinct channel if the selected source contract supports it.
3. **Loss is not a conversion.** If the selected source representation cannot carry a distinction, the implementation must preserve it or report a named refusal/loss. It may not derive independent authored presence from a v1.1 field shape.
4. **One approved crosswalk, not a second taxonomy.** The v1 capability/roadmap crosswalk is the coordination index for affected delivery tickets. A ticket records its source authority, consumer(s), supported/refused outcome and owning workstream where that crosswalk applies; this ledger does not impose a separate mandatory CAP-ID citation rule.

## Required routing

| Workstream | Owns now | Must not absorb |
| --- | --- | --- |
| A — native Quire / `quire-specification`, `quire-spec-language` | Common grammar, package composition, typing/evaluation integration and native source identity. | A parallel semantic model, a second clause source profile, or inferred field-presence syntax. |
| B — TL-* libraries | Temporal syntax, profiles and evaluators in the TL-* repositories. | Contract-IR bridge implementation, protocol consumer work or native-source authority. |
| C — `quire-rs` extraction/adapters | Existing extraction, source correspondence, and its bounded assurance campaign. | Selecting new domain semantics or receiving a new producer/schema or adapter delivery assignment by implication. |
| D — Filament/contract IR/frontends/backends | Shared model, authored-presence, producer/native correspondence, configuration and IR semantics; frontends/backends. | Treating an output mapping as a native-language frontend. |
| E — Contract-IR bridge | qcir #64/#68/#70/#71 implementation consuming merged TL-* contracts. | Editing Agent B's TL-* branches/files or creating a second editable temporal clause language. |
| Protocol epic #14 | Native protocol/choreography consumer, compensation/result implementation and qualification. | Agent B's TL-* lane or domain declaration producer semantics. |
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

The carried-forward rules cover the **state-core** semantic surface in CAP-001 through CAP-004 and CAP-006 through CAP-010: finite numeric domains; absence/null/invalid; typed acyclic records and references; bounded ordered sequences; source authority; operation instants; and explicit finite bounds. The released scalar MVP proves Boolean/integer/state-scalar agreement but not the object/graph/collection Kani profile. CAP-005 has D's accepted static Producer 1.2 implementation and A's exact native consumer; FCD #93 and runtime consumers retain the unimplemented remainder. CAP-011 through CAP-013 have accepted common architecture and partial implementations; their named remaining work is tracked above rather than described as wholly unimplemented.

## Change log

- 2026-09-09: created after the native-source-profile ruling. Initial routing: [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9) maintains this ledger, and [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95) owns the CAP-005 decision that blocks FCD #93's presence delivery.
- 2026-09-09: corrected after program-roadmap inspection. State-finite was incorrectly treated as product scope. CAP-011 (temporal), CAP-012 (protocol/choreography), and CAP-013 (profile composition) are now in-scope design work, not deferred features.
- 2026-09-10: FCD #95 selected the provisional baseline 1.2 source contract for CAP-005: presence is authored independently of multiplicity, explicit null remains distinct, legacy v1.1 cannot claim the new source information, and lossy projections refuse. The selection is specified but not yet reviewed, implemented, or promoted; FCD #93 remains preserved pending the named implementation boundary.
- 2026-09-10: corrected against the initial native Quire v1 composed cycle. CAP-011–013 are no longer recorded as missing/unowned: their temporal, protocol/choreography and composition candidates have named A/B/D/E/F owners and remain draft, unaccepted and unimplemented. CAP-002/003/005 distinguish C's historical extraction and current assurance work from the unassigned new producer/schema adapter route; D owns the producer proposal and A the native consumer boundary after acceptance. The CAP index is an approved crosswalk, not a second mandatory ticket taxonomy.
- 2026-09-13: reconciled against standard main `f8b9f9f`, SL release
  `v0.2.0`/`6eccb05`, qcir main `69dc1a3`, TL syntax main `e70f237`,
  protocol main `e53fb55`, observation main `649da78`, and FCD PR #97 merge
  `1d901d7`. Replaced blanket “unimplemented” statements with exact partial
  delivery and owning-ticket dispositions. This changes no semantic contract.
- 2026-09-13: FS01 accepts the native Quire charter, `ix:native` composed
  identity hierarchy and language/profile/backend separation. The historical
  `state-finite/0-draft` remains unchanged; the unused `state-finite/1-draft`
  proposal is superseded, and each composed definition retains its independently
  tracked semantic, implementation and qualification status.
- 2026-09-13: FS02 #29 closed through specification PR #38 at `e43d1ab`.
  Producer-qualified stages, the five-member settlement basis, projection
  refusal precedence, canonical-result identity and structured participation
  now have one authority. #28 remains the explicit role-instance/key-order
  prerequisite and #31 owns only non-duplicate residuals. Current execution
  lanes are reconciled: Agent B remains on TL-* libraries and Agent E owns the
  Contract-IR bridge tickets that consume B's merges.
- 2026-09-13: FS02 #28 closes through specification PR #41, defining
  authority-qualified opaque runtime keys,
  qualified static protocol-definition keys, exact occurrence dimensions and
  separate relationship checks for role/workflow/message/delivery/attempt/
  effect/commit/recovery subjects. Existing SL, protocol and observation code
  is recorded as partial evidence only; executable consumer adoption remains
  with those repositories. FS02 #31 now owns only its non-duplicate residuals,
  and Agent B remains exclusively on TL-* libraries.
- 2026-09-13: FS02 #31 closes through specification PR #42 at `b85df3e`.
  Channel premises now match the five-field compiled contract; FIFO and delivery
  cardinality are exact; graph identity is FR-288 with complete typed-content
  consistency; participant/internality, relationship outcomes/endpoints,
  compensation registration, retry counting and commit selection have explicit
  refusal/absence/order rules. Protocol #4/#5/#12 and Observation #18/#13 own
  executable adoption. FR-281 and shared result files remain with #40, and the
  #40 owner was warned to rebase/renumber its pre-#28 FR-287/288 collision.
- 2026-09-14: FS02 #40 closed through specification PR #44 at `983b0b2`.
  It retained FR-287/288 and added collision-free FR-289/290/291/292/293/294/
  295/296 authority for temporal boundaries, claim inputs, late data, input
  completeness and admitted-observation identity. FS02 now waits only on FCD
  #95 producer acceptance; executable TL, Contract-IR, observation and protocol
  adoption remains in the already named consumer tickets.
- 2026-09-14: FCD #95 / PR #99 accepted the static Producer 1.2 boundary at
  `4042882`, including exact digest/revision/locus/correspondence records,
  closed relationship direction and native-fixture qualification. QSL #93 /
  PR #94 then selected that exact merge at `0c83dcc` and passed 8/8 direct
  producer-correspondence tests plus the full local regression. FS02's static
  owner/interface gate is closed without claiming the separately open FCD #93
  Semantic IR/frontend work or assessment-half consumers.
- 2026-09-14: FS03 accepts the exact `quire.state.core/v1` →
  `quire.state.queries/v1` → `quire.state.graph/v1` semantic closure and
  TC-033/034/038–046 portable vectors. QSL #36/#37 and #83/#84 remain partial
  implementation evidence; qcir #69 owns expanded Kani parity. Every standard
  matrix row stays Planned until its executable evidence is bound.
- 2026-09-14: FS05 accepts exact edition/profile/wire separation, canonical
  domains, linked-package identity and explicit migration without changing
  historical bytes. QSL's strict `/2` producer/reader and source-map/domain
  controls plus protocol PR #52's independent `/2` consumer are implementation
  evidence; protocol-v1 #11 and all unexecuted standard matrices remain open.
