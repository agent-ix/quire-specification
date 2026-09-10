# Functional capability ledger

Status: binding coordination ledger, 2026-09-09. This is the functional inventory for the active Quire/Filament workstreams. It records **where each capability is authored, who owns its next decision or delivery, and whether the capability is defined**. It is not a claim that a Draft requirement is implemented, qualified, or accepted. The coordinating ticket is [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9).

## Reading the ledger

`Defined` means the intended behavior is described sufficiently to assign a delivery. `Ambiguous` means a needed behavior is visible but its owning source or semantics have not been selected; it must not be implemented by implication. `Deferred` is an intentional exclusion from the first native profile. `Missing` means no active design provides the capability.

An editable clause has exactly one source language: native Quire. OCL, SysML v2 and FRETish are output-only mappings. The shared domain model is not a second clause language: it supplies declarations consumed by the native language.

| ID | Capability and functional boundary | Source authority / owner | Current basis | Status | Next accountable work |
| --- | --- | --- | --- | --- | --- |
| CAP-001 | Select and identify the sole editable formal-clause profile. | Native Quire language — A | FS01; quire-spec-language #1/#2 | Defined | A completes parser/compiler admission and qualification. |
| CAP-002 | Bind clauses to one declared model authority, exports and typed declarations. | Existing Filament semantic model and C's adapter; A consumes it | FR-002; model-contract; quire-spec-language #3; contract-ir #54 | Defined, delivery incomplete | C completes the reviewed typed-model seam; A rejects unbound/ambiguous declarations. |
| CAP-003 | Preserve source bytes, revision, digest and clause/expression spans through extraction and lowering. | Extraction/source correspondence — C; native locations — A | FR-003; model-contract; quire-rs #9 | Defined, delivery incomplete | C implements/reconciles the extraction contract; A retains exact locations. |
| CAP-004 | Evaluate optional values, Boolean control, bounded integers, finite ordered sequences, references, populations, operations and frames. | Native semantics — A | FR-005 through FR-013; state-core profile and state-semantics | Defined, delivery incomplete | A implements and qualifies the bounded native profile. |
| CAP-005 | Distinguish a field's required presence, optional absence, explicit null, and multiplicity. | Shared domain-model/extraction contract — D decides; C exposes; A consumes | model-contract; owner ruling presence/null/invalid rule; FCD #93 | **Ambiguous** | D must decide and specify the source representation and loss/refusal behavior in [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95) before #93 changes the IR. No native syntax feature is presumed. |
| CAP-006 | Bind typed native clauses to executable IR without making the IR another authored clause authority. | Contract-IR binding — B/C; native lowering — A | contract-ir #50/#54; FR-002/003 | Defined, delivery incomplete | B/C complete binding contract and A lowers native nodes against it. |
| CAP-007 | Exchange exact wire identities, canonical domains and explicit migrations. | Shared interchange — A/B/C by artifact role | FR-014 through FR-018; FS05; contract-ir #58 | Defined, delivery incomplete | Owners finish strict-reader, producer and migration obligations without collapsing identity dimensions. |
| CAP-008 | Emit OCL, SysML v2 and FRETish as non-authoritative output mappings, with explicit loss/refusal evidence. | Mapping/emitter owners — F/D/E | FS06; contract-ir #52/#55/#56/#57; FCD #37 | Defined, delivery incomplete | Mapping owners implement Rust emitters and own semantic/loss evidence; no foreign runtime is a qualification dependency. |
| CAP-009 | Represent scalar `any` in IR and project v1.2 to v1.1 only when information is preserved; otherwise refuse with loss. | Contract IR — D | FCD #93; existing v1.1 inability state #78 | Defined, delivery incomplete | D completes the IR v1.2 change after CAP-005 is decided. |
| CAP-010 | Preserve field and relationship loci from source models. | Extraction — C | quire-rs #420 and #418 | Defined, known gap | C resolves the existing tickets; no duplicate delivery ticket is needed. |
| CAP-011 | Define temporal/protocol semantics, richer collection kinds, recursion/reflection/I/O, ambient stores, and subtype/closure behavior. | No first-profile owner | native profile and owner ruling | Deferred | Keep refused/source-addressable; create a separately scoped bridge/profile only by an explicit future decision. |

## Binding decisions

1. **No inferred language feature.** A downstream IR or extraction need does not create native Quire syntax. CAP-005 is a shared-model/extraction decision, unless its owner explicitly elects native surface syntax as the authoring authority.
2. **Presence and cardinality are independent.** Required presence answers whether a field member must be authored; multiplicity answers how many values are permitted when it is present. For example, a required `0..*` field may be present and empty, while an optional `1..*` field may be absent but must have at least one value when present. Explicit null is a distinct channel if the selected source contract supports it.
3. **Loss is not a conversion.** If the selected source representation cannot carry a distinction, the implementation must preserve it or report a named refusal/loss. It may not derive independent authored presence from a v1.1 field shape.
4. **Every new delivery ticket must cite a CAP ID.** It must name the source authority, consumer(s), supported/refused outcome and the owning workstream. A ticket that cannot do so is a design decision, not implementation work.

## Required routing

| Workstream | Owns now | Must not absorb |
| --- | --- | --- |
| A — native Quire / `quire-specification`, `quire-spec-language` | Clause grammar, typing, evaluation and native source identity. | A parallel semantic model, a second clause source profile, or inferred field-presence syntax. |
| B — contract/verification | Executable clause binding and portable verification/evidence contracts. | Domain declaration producer semantics. |
| C — `quire-rs` extraction/adapters | Exact extraction, source correspondence and typed-model adapter realization. | Selecting new domain semantics without D's decision. |
| D — Filament/contract IR/frontends/backends | Shared model and IR semantics, frontend/backends, including the ownership decision for CAP-005. | Treating an output mapping as a native-language frontend. |

## Admission rule check

The carried-forward native-profile rules cover the intended bounded semantic surface in CAP-001 through CAP-004 and CAP-006 through CAP-011: finite numeric domains; absence/null/invalid; typed acyclic records and references; bounded ordered sequences; Boolean roots/no calls; source authority; operation instants; explicit finite bounds; and temporal deferral. They do **not** select where independently authored field presence originates. CAP-005 is therefore a real source-contract decision, not evidence that native Quire is incomplete.

## Change log

- 2026-09-09: created after the native-source-profile ruling. Initial routing: [quire-specification#9](https://github.com/agent-ix/quire-specification/issues/9) maintains this ledger, and [FCD #95](https://github.com/agent-ix/filament-core-data/issues/95) owns the CAP-005 decision that blocks FCD #93's presence delivery.
