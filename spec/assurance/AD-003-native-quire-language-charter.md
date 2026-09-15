---
id: AD-003
title: "Native Quire language charter and profile boundaries"
type: ArchitectureDescription
status: accepted
owner: quire-agent-a
system: native Quire language standard
relationships:
  - target: ix://agent-ix/quire-specification/AD-001
    type: references
  - target: ix://agent-ix/quire-specification/FR-001
    type: realizes
  - target: ix://agent-ix/quire-specification/FR-019
    type: realizes
  - target: ix://agent-ix/quire-specification/NFR-001
    type: references
---
# Native Quire language charter and profile boundaries

## System Boundary

Quire is the product and language name. “Native Quire” distinguishes the sole
editable formal-clause language from generated mappings. `ix:native` is its
stable machine identity; it is not a display-name placeholder. A future public
brand change requires an explicitly versioned edition and FS05 migration. It
cannot reinterpret source, definitions, reports or evidence already bound to
`ix:native`.

The language standard owns grammar, semantic profile definitions and their
composition rules. The compiler recognizes syntax, selects exact definitions,
checks and lowers admitted subjects. A backend advertises support for a requested
claim over an already checked subject. These three decisions are independent:
knowing a definition does not implement it, compiler admission does not imply a
backend, and a capable backend cannot admit source prohibited by its profile.

Native Quire is the only editable clause authority. Shared model packages supply
typed declarations and bindings, not another clause language. OCL 2.4, SysML v2,
FRETish and later external forms are output mappings with explicit preservation,
condition, loss or refusal results. TL representations and evaluators remain an
internal temporal boundary. No external mapping or IR is normative source.

The released `ix:native` / `0-draft` / `state-finite/0-draft` definition remains
an immutable historical profile. FS01 accepts and reserves the composed hierarchy
under `ix:native` / `1-draft`; the owning FS02–FS06 requirements still determine
when each semantic definition is accepted, implemented or qualified. The earlier
`state-finite/1-draft` replacement candidate is superseded as an adoption target.
Its carried-forward constraints survive in the composed state definitions; its
identity is neither enabled nor aliased to a composed profile.

## Views

The following views fix the selection pipeline, hierarchy, current delivery
status, refusal taxonomy, extension seams and retained acceptance examples.

### Selection and support stages

```mermaid
flowchart LR
  S[Source + exact edition/profile selections] --> R[Grammar recognition]
  R --> A[Semantic profile admission]
  M[Selected typed model/bindings] --> A
  A --> C[Checked native subject]
  C --> B[Requested backend capability check]
  B --> X[Execution, lowering or output mapping]
```

Each arrow preserves the exact definition, source and subject identities. A
failure stops only its dependent path and retains the originating location and
typed cause. No stage retries under an older edition, a wider profile or a more
capable backend.

### Accepted profile hierarchy

| Layer | Accepted identity boundary | Semantic ownership after FS01 |
| --- | --- | --- |
| Language edition | `ix:native` / `1-draft` | Shared grammar, package composition and evolution under FS01/FS05 |
| State core | `quire.state.core/v1` | Bounded scalar, presence/control and operation-anchor semantics under FS03 |
| State queries | `quire.state.queries/v1` requiring state core | Acyclic predicates and ordered duplicate-preserving queries under FS03 |
| State graph | `quire.state.graph/v1` requiring state queries | Finite typed universes, references, identity and reachability under FS02/FS03 |
| Complete value and expression system | `quire.value.complete/v1` requiring state core | Complete scalar, record, recursive-value, collection, function and equality semantics under AD-005 |
| Complete model system | `quire.model.complete/v1` requiring state graph and complete values | Conformance, redefinition, dispatch and systems-model correspondence under AD-006 |
| Temporal facet and roots | `quire.temporal.bounded-facet/v1` plus event-position, fixed-sample and timestamped-window roots | Clock/history/progress semantics under FS04; TL is a derived target |
| Infinite-trace temporal facet | `quire.temporal.infinite-trace/v1` | Unbounded property meaning and sound-provider contract under AD-002/009 |
| Protocol | `quire.protocol.complete/v1` | Static/dynamic choreography, compensation, conformance, projection, realizability, composition and refinement under AD-007 |
| Observation | `quire.observation.binding/v1`, `.progress/v1` and `.range/v1` | Correlation, completeness, progress and range authority under the observation owner |
| Runtime, method and backend facets | `quire.runtime.composed/v1`, `quire.verification.plan/v1`, `quire.contract-ir.complete/v1` | Execution, workflow, analysis and exact backend contracts under AD-008/009/010 |
| Package and diagnostics | `quire.package.composed/v1` and `quire.native.diagnostics/v1` | Exact composition and diagnostic interpretation under FS02/FS05 and the owning compiler interfaces |

Acceptance here fixes the identity roles and dependency direction. It does not
freeze the current draft revision labels or claim all profile implementations.
An owning semantic ticket freezes its exact revision and digest when it accepts
that definition.

### Construct and delivery status

| Construct or family | Profile disposition | Merged implementation at FS01 close | Backend / Kani disposition |
| --- | --- | --- | --- |
| Edition/profile selection, Boolean expressions and bounded integers | Historical `state-finite/0-draft` remains admitted; composed equivalents belong to state core | Released in quire-spec-language `v0.2.0` for the bounded scalar slice | Boolean, integer and state-scalar oracle lowering is merged; broader proof is tracked separately |
| `present`, guarded `value`, short-circuit control, `let` and `if` | State-core meaning retains distinct absence, invalidity and Boolean result channels | Released bounded state-scalar behavior is partial evidence, not producer presence qualification | Unsupported distinctions refuse; no truthiness or null-to-false approximation |
| `rational(n,d)`, exact decimal, quantities and profile-selected division/remainder | Complete value facet with explicit domains, normalization, rounding/loss and definedness | Bounded rational support remains incomplete; broader values are planned work | A frontend or backend lacking the selected exact construct refuses it; it never substitutes another numeric operation |
| Ordered sequences, `size`, `contains`, `forall`, `exists` | State core; authored maximum at most 10,000 per wrapper | Native finite-sequence admission/evaluation is merged | Broader backend proof remains tracked; bound or kind gaps refuse |
| General pure functions and complete collection algebra | Complete value facet; recursion requires proved termination and collections retain order/uniqueness/multiplicity | Named acyclic query support is merged; complete functions/collections remain planned | Backend support is per requested claim and cannot narrow native semantics |
| Object/reference identity, records, recursive values and model navigation | Complete model facet with explicit closed environments, conformance and dispatch | Native finite-graph admission/evaluation is merged for explicit bounded universes | Unavailable closure/correspondence or backend support refuses without approximation |
| Bounded future/past temporal syntax | Temporal profiles only; exact clock/root selection required | Native syntax and compiled-package production are merged | TL lowering/evaluation is limited to exact reviewed correspondences; unmatched clocks/history/operators refuse |
| Static/dynamic choreography, compensation and protocol analyses | Complete protocol profile; business attempts, receipts, effects and recovery remain distinct | Finite syntax/package support is merged; dynamic/analyzer work remains planned | No evaluator executes business actions; each analyzer negotiates exact capability |
| External OCL, SysML and FRETish forms | Output mappings only | Not source frontends | Each mapping reports preserved, conditional, unrepresented or refused meaning; no approximation |
| Unbounded temporal properties, IEEE float, explicit conversion and proved recursion | Admitted by their exact complete-V1 facets | No blanket implementation claim; delivery is tracked per capability | Unsupported selected frontend/backend capabilities refuse explicitly without narrowing V1 |
| Reflection, evaluator I/O, ambient stores and implicit lossy conversion | Integrity exclusions, not deferred product features | Not admitted | Explicit located refusal under every V1 facet |

### Required diagnostic dispositions

| Input state | Required disposition | Required locus |
| --- | --- | --- |
| Ungrammatical or lexically invalid source | Invalid syntax; no semantic subject | Exact token or source span and expected syntactic category when available |
| Recognized form prohibited by the selected profile | `unsupported_construct`; no checked subject | Exact construct span, owning declaration and selected profile |
| Recognized form whose type, definedness, bound, anchor or model contract fails | Typed checking refusal; no Boolean result | Exact expression/declaration span plus typed domain, path, anchor or binding cause |
| Valid checked subject with an unsupported requested backend capability | `unsupported_projection`; retain the checked subject and emit no substitute result | Source correspondence plus requested claim, target and backend selection |
| Known profile semantics not implemented by the selected compiler/backend | Explicit unsupported/unimplemented capability; never fall back or approximate | Selected definition and source correspondence at the refusing stage |
| Missing observation, population closure or required history | Incomplete/unavailable input, distinct from false | Required role/scope and source-linked obligation; input/runtime path where the missing fact has no source span |
| Resource exhaustion or cancellation | Incomplete execution, distinct from invalid source and false | Originating stage, charged work item and effective limit/cancellation source |

The versioned diagnostic catalog may refine these typed causes. It cannot merge
invalid, unsupported, unimplemented, incomplete and logically false outcomes.

### Extension points

| Extension point | Rule |
| --- | --- |
| Edition syntax | Add grammar only under a new exact edition definition; never steal a historical identifier or reinterpret old bytes. |
| Semantic profile | Add a versioned definition and explicit dependency edges; a child may add named forms but cannot weaken inherited rules. |
| Model/type capability | Extend the producer contract and native correspondence explicitly; downstream IR shape cannot invent authoring semantics. |
| Semantic family | Add a typed family AST/interface and package capability; do not flatten it into scalar truth or string dispatch. |
| Backend or analyzer | Advertise support for exact checked subjects and requested claims; unsupported requests preserve the source subject. |
| Output mapping | Version the mapping and report loss/correspondence per obligation; generated text remains non-authoritative. |
| Diagnostic interpretation | Version code/cause interpretation separately from language meaning and response wire layout. |

### Q1–Q12 disposition

| Question | Final FS01 disposition / owner |
| --- | --- |
| Q1 unbounded logic | No global decidability claim. The infinite-trace facet defines unbounded temporal properties and sound proved/refuted/inconclusive results; finite evidence alone cannot prove infinite satisfaction. |
| Q2 identity/name | Product/display name Quire, descriptive name native Quire, stable identity `ix:native`; any later public rename uses FS05 migration. |
| Q3 rejection depth | Lexical/syntax invalidity is distinct from recognized-but-unsupported source; both retain exact loci. |
| Q4 object environment | Explicit finite typed current/pre/post universes and identity-bearing references; FS02/FS03 own producer correspondence and semantics. |
| Q5 collections | Sequence, set, bag and ordered-set semantics, explicit conversions, map/collect, flatten and fold/reduce are required in the complete value facet; the historical bounded profile remains unchanged. |
| Q6 subtyping | The complete model facet defines closed conformance/substitutability, subsetting, redefinition and dispatch; ambiguous or unavailable closure refuses. |
| Q7 fuel | Runtime work ceilings are distinct from semantic maxima; exhaustion is incomplete and cannot truncate meaning. |
| Q8 compatibility | Edition, semantic profile, artifact wire, canonicalization, backend and release versions remain independent; FS05 owns migration. |
| Q9 temporal | Native temporal profiles own meaning; TL profiles are internal targets behind exact correspondence. FS04 and temporal owners close delivery. |
| Q10 crate layout | The current Rust package/module boundary remains implementation-owned; split only for an independently versioned consumer interface. |
| Q11 ADR-0053 | Contract-IR PR #72 accepted native Quire-only source authority; external languages are mappings and the reference/bounds gate remains explicit. |
| Q12 tracking | QS #1 and this capability ledger are the plan of record; owning tickets and ledgers are updated immediately after merge. |

### Example discriminators

| Example | Distinctions that must survive |
| --- | --- |
| State / ConfigVersion | Parent ordering true versus false; a cycle; missing/unknown population closure; dangling/foreign reference; resource exhaustion; original minimum-only integer and recursive parent design gate versus the bounded 0..1000 worked corpus. |
| Timed refund | Matching versus wrong order/payment/refund identity; correct versus wrong amount; observed failure/timeout versus silence; untriggered compensation; missing progress/completeness; attempt, receipt, business effect and achieved recovery remain separate. |
| Verification portfolio | Exact source, model, definition, tool, harness and package identities; missing harness, stale package, unsupported method and failed execution cannot become a source-language pass or adequate evidence. |

## Decisions

| Choice | Rejected alternative | Consequence |
| --- | --- | --- |
| Accept one native Quire language charter and the composed profile hierarchy | Adopt the temporary `state-finite/1-draft` replacement or separate family languages | Historical bytes remain stable while state, temporal and protocol grow through explicit dependent facets. |
| Accept profile identity separately from semantic revision acceptance and implementation | Mark every PR #15 draft implemented or leave all identities ambiguous | Owners can build against stable roles without overstating delivery; each semantic ticket freezes its exact definition. |
| Fix exact rational, inheritance and pre-state rules in the composed definition set | Leave the old amendment's open questions attached to a dead candidate | `rational(n,d)`, normalization, nonzero/range proofs, child-profile monotonicity and eligible pre-state reads now have one drafted meaning for FS03 delivery. |
| Retain the original ConfigVersion reference/bounds gate | Invent a maximum, erase `parent`, or treat the bounded worked example as the original model | The original subject refuses until its producer/native finite-domain and relationship correspondence is explicit. |
| Accept the PR #15 architecture boundary | Make an executable IR, TL form or backend the language | Normative language meaning remains independent of each consumer representation and support set. |

## Risks

| Risk / scenario | Required control | Owner / next work |
| --- | --- | --- |
| Identity acceptance is mistaken for complete semantic adoption | Every definition and ledger keeps separate defined, implemented and qualified status. | A; FS02–FS06 |
| The superseded first-profile candidate is silently enabled | Frontends refuse its unavailable identity; no alias maps it to historical or composed profiles. | A; compiler inventory/evolution tests |
| A backend capability widens source admission | Admission completes before backend selection; unsupported source never reaches lowering as an admitted subject. | Compiler/backend owners |
| A producer gap is repaired in the language by inference | Missing bounds, presence, relationships and closure refuse until their producer contract supplies them. | D/A; FS02/FS03 |
| Cross-family work collapses distinct state/refund/portfolio evidence | Preserve the discriminators above in specifications, cases and result contracts. | A with family/result owners |
