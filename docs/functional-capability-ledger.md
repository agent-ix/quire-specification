# Functional capability ledger

Status: binding coordination roll-up, reconciled 2026-09-15. The sole scope
authority is the [atomic V1 capability inventory](v1-capability-inventory.md).
This ledger records **where each broad workstream is authored, who owns its next
decision or delivery, and whether merged evidence exists**. A CAP row is not a
scope boundary and cannot combine away an atomic inventory row. Each completion
is tied to merged evidence; a specification or partial implementation is not
qualification. The coordinating ticket is
[quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9);
the 2026-09-15 fact reconciliation is
[quire-specification#60](https://github.com/agent-ix/quire-specification/issues/60),
and the complete V1 repair is
[quire-specification#63](https://github.com/agent-ix/quire-specification/issues/63).

## Reading the ledger

`Defined` means the intended behavior is described sufficiently to assign a
delivery. `Implemented` names a merged implementation and does not imply broader
qualification. `Partial` names merged behavior with an open owning ticket for
the remainder. `Ambiguous` means a needed behavior is visible but its owning
source or semantics have not been selected; it must not be implemented by
implication. `Missing` means no active design provides the capability and
therefore identifies required work. There is no deferral status in the V1
inventory.

An editable clause has exactly one source language: native Quire. OCL, SysML v2 and FRETish are output-only mappings. A spec's domain model is declared in its artifacts and is not a second clause language: the domain package lifted from those artifacts supplies declarations consumed by the native language ([AD-006](../spec/assurance/AD-006-model-graph-state-contracts.md)).

| ID | Capability and functional boundary | Source authority / owner | Current basis | Status | Next accountable work |
| --- | --- | --- | --- | --- | --- |
| CAP-001 | Select and identify the sole editable formal-clause profile. | Native Quire language — A | FS01; AD-003; Contract-IR ADR-0053/PR #72; quire-spec-language #1/#2; [SL `v0.2.0`](https://github.com/agent-ix/quire-spec-language/releases/tag/v0.2.0); current QSL `6c8c401` | **Defined; historical slice implemented** — FS01 accepts the sole source-language charter and composed identity hierarchy; individual semantic profiles remain independently adopted/implemented | Preserve the accepted source/profile/backend separation while Protocol and output-mapping consumers complete their independently bounded implementations. |
| CAP-002 | Bind clauses to one imported domain package and its typed declarations. | Spec artifacts declare the model; QSL model intake and native package linking — A; artifact extraction — C; semantic IR lift — FCD extraction-frontend | AD-006; QSpec FR-030/035 and PRs #38/#41/#42; closed SL #35; [QObs #18 / PR #20](https://github.com/agent-ix/quire-observation/pull/20) at `8d54458`; Protocol #4 / PR #67 at `2a3ef19` | **Partial** — Protocol's exact versioned finite linker and observation-side qualified runtime subjects are implemented; domain package intake and full protocol assessment remain open | QSL model intake builds declarations from the domain package under AD-006. Protocol #6/#7/#12 complete assessment consumption without inventing identities from binding slots or heuristic relationships; the static/linking gates do not claim protocol conformance, progress or closure. |
| CAP-003 | Preserve source bytes, revision, digest and clause/expression spans through extraction and lowering. | Artifact extraction spans — C; native locations and model intake loci — A | FR-003; AD-006; quire-rs #9; SL #68/#86 | **Partial** — native multi-unit refusal loci are implemented; extraction span gaps remain open | C retains its existing source-correspondence tickets; A preserves the landed native loci and carries artifact spans through model intake. |
| CAP-004 | Evaluate optional values, Boolean control, bounded integers, finite ordered sequences, references, populations, operations and frames. | Native semantics — A; bounded backend parity — Contract IR/codegen | FS03; FR-033/034 and FR-038–046; closed SL #36/#37/#83/#84; [Contract IR #69](https://github.com/agent-ix/quire-contract-ir/issues/69) through PRs #87–#92 (`29c1432` integration/replay); [codegen #44 / PR #47](https://github.com/agent-ix/quire-contract-codegen/pull/47) (`73c82ad`) | **Defined; selected bounded profile implemented** — native scalar/query/graph admission plus exact definedness/arithmetic, finite graph and bounded collection Kani/codegen parity are implemented with typed non-Boolean outcomes and native replay | Keep standard matrix rows Planned until their repository-owned executable evidence is bound; unsupported or over-bound constructs remain explicit refusal/inconclusive outcomes rather than Boolean approximations. |
| CAP-005 | Distinguish a field's required presence, optional absence, explicit null, and multiplicity. | Spec artifact `Presence` and `Multiplicity` columns — QSpec; extraction — C; semantic IR `Field.presence` — FCD extraction-frontend; model intake and meaning — A | AD-006 | **Defined; delivery incomplete** — presence, multiplicity, null and default are independent authored axes; unrepresentable loss refuses | quire-rs extracts the columns, the domain package carries them and QSL model intake checks them. No native field-presence syntax is presumed. |
| CAP-006 | Bind typed native clauses to executable IR without making the IR another authored clause authority. | Contract-IR binding and backend profiles — Contract IR/codegen; native lowering — A | Contract IR #50 / PR #59 (`04eb6f8`), closed #63/#64/#70, [#69](https://github.com/agent-ix/quire-contract-ir/issues/69) and [#71 / PR #78](https://github.com/agent-ix/quire-contract-ir/pull/78) (`69ec82b`); QSL #83; codegen #44 / PR #47 (`73c82ad`) | **Implemented for the selected scalar, bounded-Kani and temporal-join slices** — strict derived clause binding, Boolean/integer/state-scalar lowering, definedness/graph/collection proof artifacts, native replay, total-Boolean projection and native/TL result joining are merged | Contract IR #50 remains the tracking owner until the serialized-package CLI consumer is qualified; #58 owns formalization coverage/addressing and #55/#56/#57 own output exporters. No backend may infer a second authored source or approximate unsupported constructs. |
| CAP-007 | Exchange exact wire identities, canonical domains and explicit migrations. | Shared interchange — A/B/C by artifact role | FR-014 through FR-018; FS05; QSpec PR #8 `8ab058b`; QSL #40/#69 and current `6c8c401`; Protocol #48 / PR #52 `5ce7978`; [Protocol #11 / PR #75](https://github.com/agent-ix/quire-protocol/pull/75) (`98c8055`) | **Defined; implemented for the current compiled-protocol handoffs** — FS05 accepts the strict reference/canonical/migration contract; compiled-protocol `/1` and `/2`, source-map/domain controls, strict consumer admission and the immutable `/1` handoff are implemented | Consumer-specific matrices and public registration remain independently owned; no similar bytes or version fallback imply equivalence. Protocol must keep exact pin/admission behavior while completing epic #14. |
| CAP-008 | Emit OCL, SysML v2/KerML v1 and FRETish as non-authoritative output mappings, with explicit loss/refusal evidence. | QSpec common profile/records — A; qcir shared foundation #95; qcir Rust emitters #55/#56/#57 | FS06; AD-004; FR-120–125/297–299; [qcir #95 / PR #98](https://github.com/agent-ix/quire-contract-ir/pull/98) at `5ea7730`; TM-008 | **Defined; common substrate implemented** — qcir implements exact request admission, four-way identity-bearing records, bounded atomic packages and downstream observer separation; target-specific OCL 2.4, SysML 2.0/KerML 1.0 and FRETish 3.1 correspondence remains Planned | Qcir #55/#56/#57 consume the merged common seam, implement independent Rust emitters and bind executable correspondence evidence; #58 accounts for their real producer results. No foreign runtime is a qualification dependency. |
| CAP-009 | Represent scalar `any` in IR and project v1.2 to v1.1 only when information is preserved; otherwise refuse with loss. | Contract IR — D | FCD #93; existing v1.1 inability state #78 | Defined, delivery incomplete | D completes the IR v1.2 change. |
| CAP-010 | Preserve field and relationship loci from source models. | Extraction — C | quire-rs #420 and #418 | Defined, known gap | C resolves the existing tickets; no duplicate delivery ticket is needed. |
| CAP-011 | Author and evaluate bounded temporal requirements, including future/past MLTL profiles and their typed predicates. | Native Quire temporal profile — standard/native owners; TL-* libraries — Agent B; Contract-IR bridge — Contract IR | FS04; FR-090–095; closed SL #38; [specification #40 / PR #44](https://github.com/agent-ix/quire-specification/pull/44); QObs PRs #19–#21 through `93bc565`; TL syntax/parse/MLTL/rewrite owner revisions `842d825`/`00c1b8d`/`2286218`/`fe03f29`; qcir PR #77 `202210c`; [qcir #71 / PR #78](https://github.com/agent-ix/quire-contract-ir/pull/78) `69ec82b` | **Defined; owner and bridge implementations complete for the selected slice** — compiler admission, observation topology/boundaries, TL future/past owner APIs, predicate projection and exact native/TL correspondence/result joining are implemented | Protocol #12 owns downstream observation/temporal consumption after its canonical result prerequisite; Agent B retains TL-* maintenance. Standard TM-004/TM-007 rows remain Planned until their own executable bindings land. |
| CAP-012 | Author, model, and evaluate protocol/choreography behavior beyond single-invocation state clauses. | Native Quire protocol/choreography standard — QS/FS owners; QSL activation owner — Agent E; completed observation/progress owner — QObs #1; consumer implementation — Protocol epic #14 | FR-049–062, FR-236/287/288/300; QSpec PRs #38/#41/#42/#44 and [#58](https://github.com/agent-ix/quire-specification/pull/58) `478afa2`; QSL current `6c8c401` and open #111; QObs #1 at `93bc565`; Protocol #14 at `98c8055`; qverify #32 / PR #33 at `18e66cb` | **Partial** — source contracts, identities, channels, graph/compensation rules, observation authority, strict linking, compensation assessment, shared result foundations, immutable compiler handoff and E02 owner contract are merged; conformance/result composition and qualification remain open | Agent E completes QSL #111, which unblocks Protocol #6; #7 follows. Agent B owns Protocol #8 after its TL lane; #12/#9 wait on #8, then #10/#63/#13 close reproduction, scenario evidence and qualification. |
| CAP-013 | Compose state, temporal, protocol/choreography, and future profiles into one official language without profile-specific identity or source-authority drift. | Native Quire common package/profile contract and model intake — A; protocol/TL/observation family owners | PR #15; AD-001/AD-003/AD-004; FS02; FS03–FS06 closures; QSpec PRs #38/#41/#42/#44/#58; QSL `6c8c401`; QObs `93bc565`; qcir Kani `29c1432`, temporal join `69ec82b`; codegen `73c82ad`; Protocol `98c8055` | **Partial** — standard authorities, bounded Kani parity, temporal joining, observation inputs, strict Protocol linking/compensation and E02 owner contract are implemented; Protocol closeout, formalization coverage and the three output exporters remain open | Complete QSL #111 and Protocol epic #14 in recorded dependency order, then Contract IR #58 and #55/#56/#57. QSpec #55 remains blocked on `spec-artifacts-process#81`; no local matrix rename is valid meanwhile. |

## Binding decisions

1. **No inferred language feature.** A downstream IR or extraction need does not create native Quire syntax. CAP-005 is declared in spec artifacts and carried by extraction, not by native surface syntax.
2. **Presence and cardinality are independent.** Required presence answers whether a field member must be authored; multiplicity answers how many values are permitted when it is present. For example, a required `0..*` field may be present and empty, while an optional `1..*` field may be absent but must have at least one value when present. Explicit null is a distinct channel if the selected source contract supports it.
3. **Loss is not a conversion.** If the selected source representation cannot carry a distinction, the implementation must preserve it or report a named refusal/loss. It may not derive independent authored presence from a v1.1 field shape.
4. **One atomic scope authority.** The atomic V1 inventory is the scope source;
   this CAP ledger and the historical 51-group proposal are derived navigation
   views. Every implementation ticket records its atomic capability IDs, source
   authority, consumers, exact outcomes and owning workstream.

## Reconciled default-branch snapshot

This snapshot is evidence for the 2026-09-15 statuses above, not a dependency
substitute. Consumers still pin the exact owner revision required by their
reviewed contract.

| Repository | Verified `main` revision | Governing disposition |
| --- | --- | --- |
| `quire-spec-language` | `6c8c401bb807a417215fae7593ef27fd9bab051e` | FR-300 owner binding #111 remains open. |
| `quire-contract-ir` | `c269a1a6fc6c7adc0770637b4fcd273e5ca05e9c` | Kani #69 and temporal #71 closed; #50/#55/#56/#57/#58 remain open at their narrower boundaries. |
| `quire-contract-codegen` | `73c82ade7b1e8f61d4968e32e3df29ac5e3d36c5` | Integrated bounded-Kani corpus #44 closed through PR #47. |
| `quire-protocol` | `98c805534e4b68bea6140a54f16b58805b05a037` | Immutable compiler handoff #11 closed; epic #14 serial/integration/qualification remainder stays open. |
| `quire-observation` | `93bc565072fb05d1ecf29f15725b75c0a77378fb` | Observation owner/topology/boundary epic complete. |
| `quire-verification` | `18e66cb08fd5fb5e0ccf49d045dfac0523a0613d` | E02 draft-2 cross-repository contract #32 closed. |
| `filament-core-data` | `7242b683bfa849595476a57685898c160024405f` | Semantic IR and extraction-frontend #93 remains open. |
| `quoin` | `2e7b3301394c06e0c0b3522797df1bf9ea82861a` | Protocol intake #519 remains behind Rust-port epic #373. |

## Required routing

| Workstream | Owns now | Must not absorb |
| --- | --- | --- |
| A — native Quire / `quire-specification`; Protocol #6 after its owner gate | Common grammar, package composition, model intake and model semantics, typing/evaluation integration, native source identity, tracking reconciliation, and the Protocol serial core after QSL #111. | A parallel semantic model, a second clause source profile, or inferred field-presence syntax. |
| B — TL-* libraries, then Protocol #8 | Temporal syntax, profiles and evaluators in the TL-* repositories; canonical Protocol result composition only after that lane. | Contract-IR bridge implementation or native-source authority. |
| C — `quire-rs` extraction/adapters | Existing extraction, extraction rules for spec-declared model features, source correspondence, and its bounded assurance campaign. | Selecting new domain semantics or receiving a new schema or adapter delivery assignment by implication. |
| D — contract IR/semantic IR/frontends/backends | Semantic IR schema and the extraction-frontend lift of spec artifacts into domain packages; IR semantics; frontends/backends. | Owning domain model semantics, or treating an output mapping as a native-language frontend. |
| E — QSL FR-300 owner binding | QSL #111 / existing PR #110, publishing the complete typed activation binding that unblocks Protocol #6. | Reconstructing the value in Protocol, editing Agent B's Protocol #8/TL work, or creating a second temporal authority. |
| Protocol epic #14 | Native protocol/choreography consumer, compensation/result implementation and qualification. Agent A resumes #6 after QSL #111; Agent B owns #8 after TL-* work. | Domain model semantics, QSL owner reconstruction, or Contract-IR/TL authority. |
| Completed observation owner — QObs #1 | Correlation, completeness, progress, finite explicit topology and observation/window authority artifacts delivered at QObs `93bc565`. | Replacing model or protocol authority, or evaluating graph/temporal/protocol truth. |

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

The carried-forward rules cover the **state-core** semantic surface in CAP-001 through CAP-004 and CAP-006 through CAP-010: finite numeric domains; absence/null/invalid; typed acyclic records and references; bounded ordered sequences; source authority; operation instants; and explicit finite bounds. The bounded-Kani campaign now proves the selected definedness/arithmetic, object/reference/graph and collection/query profile through one integrated codegen corpus with native replay; this does not expand that profile or mark the standard's still-unbound matrix rows executed. CAP-005 is declared in spec artifacts; extraction, lift, model intake and runtime consumers retain its delivery. CAP-011's selected temporal owner/bridge slice is implemented, while CAP-012/013 retain their named Protocol, formalization-coverage and exporter work.

## Change log

- 2026-09-09: created after the native-source-profile ruling. Initial routing: [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9) maintains this ledger.
- 2026-09-09: corrected after program-roadmap inspection. State-finite was incorrectly treated as product scope. CAP-011 (temporal), CAP-012 (protocol/choreography), and CAP-013 (profile composition) are now in-scope design work, not deferred features.
- 2026-09-10: corrected against the initial native Quire v1 composed cycle. CAP-011–013 are no longer recorded as missing/unowned: their temporal, protocol/choreography and composition candidates have named A/B/E/F owners and remain draft, unaccepted and unimplemented. The CAP index is an approved crosswalk, not a second mandatory ticket taxonomy.
- 2026-09-13: reconciled against standard main `f8b9f9f`, SL release
  `v0.2.0`/`6eccb05`, qcir main `69dc1a3`, TL syntax main `e70f237`,
  protocol main `e53fb55`, and observation main `649da78`. Replaced blanket “unimplemented” statements with exact partial
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
  completeness and admitted-observation identity. Executable TL, Contract-IR, observation and protocol
  adoption remains in the already named consumer tickets.
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
- 2026-09-14: FS04 accepts the exact temporal/clock/history and observation/
  progress/completeness closure plus portable TC-110–118/140–147 controls.
  QObs PR #19, the merged TL owner APIs and qcir predicate PR #77 are partial
  implementation evidence; qcir #71 and observation #18/#13 remain open and
  TM-004/TM-007 stay Planned.
- 2026-09-14: QObs #18 / PR #20 implemented FR-287 qualified runtime
  subjects, FR-262 typed relationships and bounded four-state correlation at
  `8d54458`. Its local gate passed 42/42 matrix rows and FR-005 9/9. QObs #13
  remains the named owner for topology/traversal bounds and admitted
  scope/clock window anchoring; no protocol, temporal or TL-* completion is
  inferred.
- 2026-09-14: QObs #13 / PR #21 closed the observation epic at `93bc565`.
  Plan-001 is 15/15 complete; the final local gate passed 46/46 Rust tests,
  Quire coverage 52/52 and SR-036/SR-037. FR-006/TC-006 prove arbitrary valid
  finite explicit topology retention and exact event-position, fixed-sample
  and timestamped-event boundary anchoring without adding graph/temporal
  evaluation or changing production Rust, schemas, identity preimages or
  `resources/native-v1/`. Qcir #71 and protocol #14 remain the accountable
  temporal/result consumers; Agent B retains TL-* maintenance.
- 2026-09-15: QSpec #60 reconciled this ledger to current default branches.
  Contract IR #69 and #71, codegen #44, Protocol #11, qverify #32 and QSpec
  #57 are recorded complete at their merged revisions; none of their remaining
  downstream consumers is credited complete. QSL #111, Protocol epic #14,
  Contract IR #50/#55/#56/#57/#58 and Quoin #519 retain exact open
  owners. Agent E moves to QSL #111, while Agent B retains TL-* then Protocol
  #8. QSpec #55 is blocked on `spec-artifacts-process#81` because the installed
  module structurally requires `Coverage Status` while its classifier selects
  `Status`; no invalid consuming-repository rename was merged.
- 2026-09-15: Contract IR #95 / PR #98 implemented the accepted FS06 common
  request, mapping-record and immutable-package boundary at `5ea7730`. Its
  TC-043 binds all 14 local acceptance criteria without claiming target
  correspondence. CAP-008 therefore advances only to common-substrate
  implementation; qcir #55/#56/#57 and downstream #58 remain open, and
  TM-008/IT-060 remain Planned.
