# Quire V1 existing-specification coverage audit

Status: Plan-008 Task-003 working audit, 2026-09-15. Scope authority:
[`v1-capability-inventory.md`](v1-capability-inventory.md).

This audit compares each atomic capability with the actual behavior and
acceptance criteria in the current QSpec tree. It does not infer coverage from
an artifact title, accepted parent architecture, implemented code or closed
ticket. The exact existing artifacts in `Coverage` were read at the Plan-008
baseline. `Delta` is the required specification action before the comprehensive
`/specify` cycle can be complete.

## Classification and matrix keys

- `Reuse`: an existing requirement has exact behavior and acceptance criteria.
- `Amend`: the right owner exists but does not cover the full atomic obligation.
- `Split`: the existing owner combines independently testable behavior.
- `Resolve`: current normative statements disagree or an accepted architecture
  excludes a capability now required by the master inventory.
- `New`: no normative behavior and acceptance criteria exist.

`CF`, `NS`, `T`, `P`, `O`, `X`, and `V` denote the composed-foundation,
native-source, temporal, protocol, observation, output-mapping and
verification-reference Test Matrices. `New` in the matrix column means the
information-architecture gate must allocate a matrix rather than append rows to
an unrelated family.

## 1. Source, package and identity

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-SRC-001 | AD-001/003; FR-001/019/020 | Reuse | Make the atomic inventory, not the historical group matrix, FR-020's exact release input. | CF, NS |
| V1-SRC-002 | AD-003; FR-001/030/032 | Reuse | Add complete-edition cases for every new declaration family. | CF |
| V1-SRC-003 | AD-003; FR-001/030/130 | Amend | Replace the fixed bounded profile list with a compositional bundle covering all inventory subsystems. | CF |
| V1-SRC-004 | AD-001; FR-020/030/130 | Amend | Extend the linked package beyond state/temporal/protocol to analysis, workflow and tooling declarations. | CF |
| V1-SRC-005 | FR-003/030; FR-202/203 | Reuse | Preserve current byte/digest authority in every new family. | CF, NS |
| V1-SRC-006 | FR-003/030; FR-202 | Reuse | Preserve source revision independently in every new family. | CF, NS |
| V1-SRC-007 | FR-003/036/047/049; FR-203/274 | Resolve | Decide FR-003's permitted layout deletion versus FR-036's no-newline-conversion rule, then extend locus retention through every path. | CF, NS |
| V1-SRC-008 | FR-002/030/035; package contract | Reuse | Extend exact import cases to richer model capabilities. | CF |
| V1-SRC-009 | FR-002/030/035 | Reuse | Add conformance/redefinition/dispatch binding cases. | CF |
| V1-SRC-010 | AD-001; FR-004/015/030/035; FR-200/201 | Amend | Keep the domain package `sha256-jcs` domain distinct from package fingerprints and register new analysis, plan, plugin and generated-artifact identities. | CF |
| V1-SRC-011 | FR-030/032; NFR-021/051/060 | Split | Create one cross-subsystem manifest requirement rather than restating pins inside consumers. | CF |
| V1-SRC-012 | FR-016/032 | Reuse | Add migrations for the newly admitted type and family definitions. | CF, V |
| V1-SRC-013 | FR-014/020/031/032/130 | Amend | Define discovery for all atomic language and consumer capabilities, not four family flags. | CF |
| V1-SRC-014 | AD-001/003 extension tables; FR-032/040 | Amend | Specify the uniform declaration/family extension protocol and compatibility proof. | CF |
| V1-SRC-015 | FR-001/019/037/047; FR-270–274 | Reuse | Extend the diagnostic catalog to every new construct and stage. | CF, NS |

## 2. Type and model system

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-TYPE-001 | AD-001; FR-002/030/035 | Reuse | Extend the common universe to every newly admitted model/type form. | CF |
| V1-TYPE-002 | FR-005/006/033/045 | Reuse | Preserve total Boolean semantics across new functions and analyses. | CF, NS |
| V1-TYPE-003 | FR-007/044; FR-206 | Reuse | Separate the finite signed-64 profile from mathematical unbounded integers. | CF, NS |
| V1-TYPE-004 | FR-039/044; FR-205/206 | Reuse | Add runtime/backend and cross-family acceptance, not new core meaning. | CF |
| V1-TYPE-005 | FR-019/044 only reject float-like forms; no decimal semantics | New | Define exact decimal representation, bounds, literals, equality, arithmetic and conversions. | New |
| V1-TYPE-006 | Shared grammar/model prose only | New | Define text domain, literals, equality, ordering, normalization and bounds. | New |
| V1-TYPE-007 | Model-reference prose only | New | Define enum declaration/member identity, equality, ordering and evolution. | New |
| V1-TYPE-008 | FR-007/039/044 mention exact units but define no dimension model | Amend | Add dimension, unit identity, quantity typing and arithmetic rules. | New |
| V1-TYPE-009 | FR-007/044 reject incompatible or implicit conversions | New | Define explicit exact unit conversion and loss/refusal behavior. | New |
| V1-TYPE-010 | FR-019/045; AD-006 artifact presence declarations | Reuse | Add all new value/container forms to presence/null behavior. | CF, NS |
| V1-TYPE-011 | FR-019/035/045; AD-006 artifact multiplicity declarations | Reuse | Add richer collection and recursive-value multiplicity cases. | CF, NS |
| V1-TYPE-012 | FR-019 admits acyclic record paths but `tuple` is refused | Amend | Define record/tuple construction, field identity, typing, equality and bounds. | New |
| V1-TYPE-013 | FR-009/019/043 explicitly deny structural equality | Resolve | Add typed structural equality while preserving object-identity separation. | New |
| V1-TYPE-014 | FR-008/041 | Reuse | Preserve sequence semantics while generalizing collection interfaces. | CF, NS |
| V1-TYPE-015 | FR-008/019/040 only refuse sets | Amend | Define finite set identity, equality, iteration order exposure and operations. | New |
| V1-TYPE-016 | FR-008/019/040 only refuse bags | Amend | Define bag multiplicity, equality, iteration and operations. | New |
| V1-TYPE-017 | FR-019/040 only refuse unsupported kinds | Amend | Define ordered-set uniqueness plus stable order semantics. | New |
| V1-TYPE-018 | FR-019/041 reject implicit conversions only | Amend | Define explicit conversion results and loss conditions among all collection kinds. | New |
| V1-TYPE-019 | FR-019/043 | Reuse | Preserve acyclic value containment as one admitted profile. | CF, NS |
| V1-TYPE-020 | FR-019/043 reject recursive containment | Resolve | Define finite recursive values, cycle/identity distinction and traversal bounds. | New |
| V1-TYPE-021 | FR-011/035/043/112; FR-263–266 | Reuse | Generalize beyond explicit-member-only observation profile where required by complete analyses. | CF, O |
| V1-TYPE-022 | FR-009/043; FR-204 | Reuse | Extend reference typing through richer model conformance. | CF, NS |
| V1-TYPE-023 | FR-019/043 prohibit ambient lookup; no explicit lookup expression | Amend | Define explicit closed-environment lookup and all-instances query with completeness premises. | New |
| V1-TYPE-024 | FR-010/043; FR-204 | Reuse | Extend navigation/reachability over new relationship and inheritance semantics. | CF, NS |
| V1-TYPE-025 | AD-001 model boundary only | New | Define original/effective declaration identities and normalization provenance. | New |
| V1-TYPE-026 | AD-003 and FR-019 refuse subtyping | Resolve | Define conformance and substitutability rules under versioned profiles. | New |
| V1-TYPE-027 | Scope-audit prose only | New | Define subsetting/redefinition validity, precedence and correspondence. | New |
| V1-TYPE-028 | Scope-audit prose only | New | Define closed dispatch sets, method selection and ambiguity. | New |
| V1-TYPE-029 | FR-035/049/050 bind a narrow component/endpoint subset | Amend | Define typed consumption of parts, ports, interfaces, connections and allocations. | New |
| V1-TYPE-030 | FR-019/039/043/044 | Reuse | Extend exact cast refusal/explicit-conversion separation to new types. | CF, NS |
| V1-TYPE-031 | FR-019/044 require finite maxima and refuse minimum-only domains | Resolve | Define mathematical unbounded integers plus per-consumer finite-domain admission. | New |

## 3. Expressions, queries and contracts

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-EXPR-001 | FR-006/037/038/045 | Reuse | Extend lexical values to general functions and new types. | CF |
| V1-EXPR-002 | FR-033/034/038 | Reuse | Preserve predicate identity and family bindings. | CF |
| V1-EXPR-003 | FR-019/033/040 only permit acyclic Boolean predicates | Amend | Define general total pure typed functions, calls and result types. | New |
| V1-EXPR-004 | FR-019/033 refuse every recursive call | Resolve | Define checked termination, recursion bounds and nontermination outcomes. | New |
| V1-EXPR-005 | FR-007/044 | Reuse | Keep exact definedness and add unbounded/decimal/float profile dispatch. | CF, NS |
| V1-EXPR-006 | FR-007 specifies integer div/rem; FR-019/037/039/044 refuse them | Resolve | Select exact profile-specific division/remainder semantics and eliminate contradictory V1 admission. | CF, NS |
| V1-EXPR-007 | FR-039/044; FR-205/206 | Reuse | Add complete runtime/backend/cross-family criteria. | CF |
| V1-EXPR-008 | FR-019/044 reject floating point | Resolve | Define explicit IEEE profiles and consumer capability boundaries. | New |
| V1-EXPR-009 | No normative NaN/infinity/signed-zero/rounding behavior | New | Define exceptional float values, comparison, ordering and rounding modes. | New |
| V1-EXPR-010 | FR-041 covers size/contains/count | Reuse | Generalize behavior across set, bag and ordered-set kinds. | CF |
| V1-EXPR-011 | FR-008/041 | Reuse | Generalize quantification across collection kinds and unbounded consumers. | CF, NS |
| V1-EXPR-012 | FR-041 covers sequence filter | Amend | Define result-kind and ordering rules for every collection kind. | CF |
| V1-EXPR-013 | FR-041 covers sequence map; FR-008 historical collect refusal | Resolve | Unify map/collect semantics and add all collection result bounds. | CF, NS |
| V1-EXPR-014 | FR-008/019/041 explicitly refuse flattening | Resolve | Define flatten result kind, multiplicity/order and checked total bounds. | New |
| V1-EXPR-015 | FR-041 specifies count/sum only | Amend | Define a typed fold/reduce catalog, empty identities, associativity and intermediate bounds. | New |
| V1-EXPR-016 | FR-009/019/043 cover object/reference denial only | Amend | Define the complete scalar, enum, record and collection equality matrix. | New |
| V1-EXPR-017 | FR-009/010/043 | Reuse | Extend navigation to inherited/redefined relationships and explicit lookup. | CF, NS |
| V1-EXPR-018 | FR-012/046 and state contract | Reuse | Add complete example/qualification cases. | CF, NS |
| V1-EXPR-019 | FR-012/042/046 and state contract | Reuse | Add complete example/qualification cases. | CF, NS |
| V1-EXPR-020 | FR-012/042/046 and state contract | Reuse | Add complete example/qualification cases. | CF, NS |
| V1-EXPR-021 | FR-006/034/042/045/046 | Reuse | Apply immutable capture rules to functions, temporal and protocol consumers uniformly. | CF |
| V1-EXPR-022 | FR-013/046 | Reuse | Extend frames to richer model behaviors and workflow adapters. | CF, NS |
| V1-EXPR-023 | AD-003; FR-047; FR-270–274 | Amend | Add missing analysis, tooling, plugin and workflow-adapter stages/cause kinds. | CF |
| V1-EXPR-024 | FR-019/033/053/055/057 integrity rules | Reuse | Add plugin/function adverse cases without weakening purity. | CF, P |
| V1-EXPR-025 | AD-001/003; FR-019/033/039/049; NFR-020 | Reuse | Define the separate authorized workflow-adapter effect boundary. | CF, P |
| V1-EXPR-026 | FR-007/031/044/047; NFR-002/010/040 | Reuse | Extend non-Boolean outcomes to every new evaluator/analyzer. | CF, NS, T |

## 4. Temporal, clocks and observation

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-TEMP-001 | FR-048/091; FR-250/254/255 | Reuse | Add complete native/runtime/TL correspondence coverage. | T, CF |
| V1-TEMP-002 | FR-048/091; FR-250/254/255 | Reuse | Add complete native/runtime/TL correspondence coverage. | T, CF |
| V1-TEMP-003 | FR-048/091; FR-250/254/255 | Reuse | Add complete native/runtime/TL correspondence coverage. | T, CF |
| V1-TEMP-004 | FR-048/091; FR-250/254/255 | Reuse | Add complete native/runtime/TL correspondence coverage. | T, CF |
| V1-TEMP-005 | FR-048/092; FR-254/255/257/293 | Reuse | Add complete native/runtime/TL correspondence coverage for all five operators. | T, CF |
| V1-TEMP-006 | FR-092/094; FR-257/293 | Reuse | Preserve the execution-origin/history-cutoff distinction in richer clocks. | T, O |
| V1-TEMP-007 | FR-090/091/094; FR-252 | Reuse | Add end-to-end conformance evidence. | T |
| V1-TEMP-008 | FR-090/091/094/095; FR-252 | Reuse | Add end-to-end conformance and missing-sample evidence. | T |
| V1-TEMP-009 | FR-090/091/094; FR-252/255/256/289 | Reuse | Add end-to-end finite-window runtime evidence. | T, O |
| V1-TEMP-010 | FR-048/093; FR-230–232/253 | Reuse | Generalize captures to new value types and protocol scopes. | T, P |
| V1-TEMP-011 | FR-094/113; FR-256/268/289 | Reuse | Add executable silence/progress integration. | T, O |
| V1-TEMP-012 | FR-091/094/113; FR-241/258/284/295 | Reuse | Keep all closure and completeness axes independent in every consumer. | T, O, P |
| V1-TEMP-013 | FR-093/115; FR-231/244 | Reuse | Reconcile the obligation/result activation object overlap during object repair. | T, P, O |
| V1-TEMP-014 | FR-094/113; FR-240/268/294 | Reuse | Add executable revisioned replay and supersession coverage. | T, O |
| V1-TEMP-015 | FR-112/113; FR-263–266/268/295 | Reuse | Extend completeness beyond the current explicit-members profile where required. | O, T |
| V1-TEMP-016 | FR-113-AC-2 only says uncertainty remains distinguishable | Amend | Define interval representation, clock arithmetic, ordering and result impact. | New |
| V1-TEMP-017 | FR-094/113 classify late contradictions but say no automatic repair | Resolve | Define explicit revisioned out-of-order repair and retained historical results. | New |
| V1-TEMP-018 | FR-091/092/095 require total Boolean valuations | Resolve | Define partial valuations and their sound multi-valued/incomplete evaluation. | New |
| V1-TEMP-019 | AD-002; FR-095; FR-259; IT-040/041 | Reuse | Update stale implementation notes and extend correspondence for new clocks/operators. | T |
| V1-TEMP-020 | FR-034/048/095 | Reuse | Extend native predicate evaluation to richer value/function semantics. | T, CF |
| V1-TEMP-021 | AD-001; FR-091/092/060 | Reuse | Keep finite-evidence refusal while separately specifying sound symbolic/unbounded claims. | T, New |
| V1-TEMP-022 | FR-090/095; FR-251/252/259/289; IT-041 | Reuse | Extend mismatch cases to uncertainty and partial valuations. | T |
| V1-TEMP-023 | AD-001/AD-003 and FR-020 currently exclude general infinite-trace analysis; no positive unbounded-property semantics exist | New | Specify an explicit infinite-trace profile, unbounded temporal syntax/meaning, sound analysis-provider contract, non-conclusive outcomes and finite-to-infinite evidence boundary. | T, New |

## 5. Protocol and choreography

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-PROT-001 | FR-049/050; FR-210/211/217 | Reuse | Add dynamic-role interaction without changing static role identity. | P, CF |
| V1-PROT-002 | FR-035/050; FR-212/287 | Reuse | Add dynamic lifecycle and population cases. | P |
| V1-PROT-003 | FR-035/049/050 | Reuse | Extend to parts, ports, interfaces, connections and allocations. | P, New |
| V1-PROT-004 | FR-049/050/051; FR-214 | Reuse | Extend payload typing to all new model/value forms. | P, CF |
| V1-PROT-005 | FR-049/050/054; FR-215/216 | Amend | Preserve channel/entity versus premise distinction; define FIFO key scope and whether missing duplication premises refuse. | P |
| V1-PROT-006 | FR-049/051; FR-221–225/287 | Reuse | Retain exact identities in dynamic protocols and adapters. | P, CF |
| V1-PROT-007 | FR-035/050/051/111; FR-262/287 | Reuse | Extend related-instance examples and relationship algebra. | P, O |
| V1-PROT-008 | FR-049/052; FR-220/228 | Reuse | Generalize sequence over dynamic subprotocol calls. | P, CF |
| V1-PROT-009 | FR-049/052/053; FR-227/228 | Reuse | Extend owned choice to projection/realizability premises. | P |
| V1-PROT-010 | FR-049/052; FR-220/228 | Amend | Close the admitted join-policy vocabulary, outstanding-branch behavior and dynamic branch interaction. | P |
| V1-PROT-011 | FR-049/052; FR-228/239 | Reuse | Extend beyond literal maximum to invariant/termination obligations. | P |
| V1-PROT-012 | FR-049/050/052 explicitly require finite static roles; AD-001 excludes unrestricted choreography | Resolve | Define dynamic role spawn/lifetime, binding, population and closure semantics. | New |
| V1-PROT-013 | FR-054/NFR-020 define delivery premises and bounded processing, not queue semantics | Amend | Define queue state, capacity, ordering, overflow and symbolic bounds. | New |
| V1-PROT-014 | FR-052 rejects unbounded/zero-progress loops | Resolve | Define loop invariants, variants, termination/resource claims and inconclusive outcomes. | New |
| V1-PROT-015 | FR-049/052/113; FR-220 | Reuse | Apply causal identity to dynamic and analyzed protocols. | P, O |
| V1-PROT-016 | FR-053; FR-227 | Amend | Define what evidence discharges choice visibility, then feed it into projection and realizability. | P |
| V1-PROT-017 | FR-054; FR-215/216 | Reuse | Extend delivery premises to explicit queue and failure models. | P |
| V1-PROT-018 | FR-034/049/055; FR-230 | Amend | Define whole-execution-origin obligation identity and generalize guards to new functions/types. | P, CF |
| V1-PROT-019 | FR-049/055/093; FR-232 | Amend | Define capture-set order/duplicate-expression behavior and preserve inclusive boundaries in dynamic protocols. | P, T |
| V1-PROT-020 | FR-049/056; FR-233/234 | Reuse | Add workflow-adapter handoff while keeping registration observational. | P |
| V1-PROT-021 | FR-049/051/056–058; FR-224–239/287 | Reuse | Preserve the corrected authority-qualified identities while adding dynamic and adapter paths. | P |
| V1-PROT-022 | FR-057; FR-237/238 | Reuse | Add analyzer and adapter correspondence. | P |
| V1-PROT-023 | FR-055/058/061; FR-234–236/240–247 | Amend | Close the compensation lifecycle vocabulary/transitions and preserve the full outcome partition in every consumer. | P |
| V1-PROT-024 | FR-051/058; FR-224/226/287 | Reuse | Use the corrected delivery/effect keys and add executable duplicate cases. | P |
| V1-PROT-025 | FR-059/114; NFR-021; IT-021 | Reuse | Complete executable protocol replay and cross-consumer evidence. | P, O |
| V1-PROT-026 | FR-059; FR-240/247/290/291/292 | Reuse | Complete executable global-conformance semantics and qualification. | P |
| V1-PROT-027 | FR-060 separates the claim but AD-001 and matrix classify projection as later | Resolve | Define projection inputs, algorithm contract, soundness result and counterexample. | New |
| V1-PROT-028 | FR-060 separates the claim but defines no realizability semantics | Amend | Define realizability premises, result, witnesses and counterexamples. | New |
| V1-PROT-029 | No normative assume-guarantee semantics | New | Define contract composition, compatibility, circularity and proof obligations. | New |
| V1-PROT-030 | FR-060 names refinement only as a claim boundary | Amend | Define implementation model, refinement relation, result and counterexample. | New |
| V1-PROT-031 | AD-001/003; FR-049/056; NFR-020 | Reuse | Add a positive typed decision-to-authorized-adapter interface without evaluator side effects. | P, New |

## 6. Runtime, analysis and workflows

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-RUN-001 | StR-001; US-003; FR-005–013/041–046 | Reuse | Add qualification criteria over the complete state language. | NS, CF |
| V1-RUN-002 | US-008; FR-090–095/114; IT-040 | Reuse | Complete native temporal runtime and exact bridge parity evidence. | T, O |
| V1-RUN-003 | US-006; FR-050–061/114; IT-020/021 | Reuse | Complete protocol runtime and result integration evidence. | P, O |
| V1-RUN-004 | AD-001/002; FR-094/114; NFR-040/050 | Reuse | Complete incremental temporal/protocol monitoring and restart evidence. | T, O, P |
| V1-RUN-005 | FR-031/061/115; FR-240–244/284 | Reuse | Apply dimensions to every new method and adapter. | P, O |
| V1-RUN-006 | FR-061/115; FR-245/246/248/280–282/285 | Reuse | Add complete assumption/participation/provenance contracts for new methods. | P, O, V |
| V1-RUN-007 | FR-011/035/043/112; FR-263–266 | Reuse | Add richer population selectors with explicit completeness. | O, CF |
| V1-RUN-008 | FR-041/049/112/113 | Amend | Define cross-workflow aggregate subject, grouping and duplicate semantics. | O, P |
| V1-RUN-009 | FR-035/049/050/111; FR-262/287 | Reuse | Add end-to-end related-instance runtime cases. | O, P |
| V1-RUN-010 | StR-001; FR-005–013/041–046; ConfigVersion example | Reuse | Expand the reference corpus to all state types/operators. | NS, CF |
| V1-RUN-011 | AD-001; FR-049/093/094; timed-refund example | Amend | Specify the exact executable package, inputs and expected result corpus. | CF, T, P, O |
| V1-RUN-012 | AD-001; FR-049–059; choreography example | Amend | Specify the complete fan-out/join/compensation executable corpus. | CF, P, O |
| V1-RUN-013 | AD-001 says general simulation is a separate consumer; no behavior/AC | New | Define finite simulator inputs, nondeterminism, schedules, bounds and results. | New |
| V1-RUN-014 | NFR-040 names model-based-test-generation only as a method | New | Define generator domains, oracle, shrink/replay and provenance. | New |
| V1-RUN-015 | AD-001 and FR-060 only preserve claim separation | New | Define relational syntax/subjects, paired executions and result semantics. | New |
| V1-RUN-016 | Historical matrix classifies hyperproperties as later | Resolve | Define finite-trace-set hyperproperty syntax, quantification and analysis results. | New |
| V1-RUN-017 | AD-001 keeps methods/plans outside clause AST; no DSL contract | New | Define linked plan language, method selection, bounds, dependencies and evidence outputs. | New |
| V1-RUN-018 | No normative continuous/hybrid behavior | New | Define state/time model, differential/algebraic constructs, solver contracts and sound result envelopes. | New |
| V1-RUN-019 | FR-020 excludes synthesis; no normative behavior | Resolve | Define bounded synthesis request, candidate/witness status and proof separation. | New |
| V1-RUN-020 | AD-001 excludes orchestration; no positive adapter contract | Amend | Define typed decisions, authorization, idempotency, effect receipts and feedback without evaluator-owned effects. | New |

## 7. IR, proof backends and output mappings

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-BACK-001 | AD-001/003; FR-031/060; FR-290–292 | Reuse | Extend request kinds to every new runtime/analyzer/exporter. | CF, P |
| V1-BACK-002 | FR-031/061/094/115 | Reuse | Apply sibling preservation to all new backends. | CF, P |
| V1-BACK-003 | AD-001; FR-030/031/095/120 | Reuse | Define uniform typed IR extension seams for all new type/family nodes. | CF, T, X |
| V1-BACK-004 | Current QSpec specifies exact Boolean/integer/rational/query/graph/temporal slices but no complete IR execution contract | Amend | Define the complete IR family, typed outcomes and consumer capability matrix. | New |
| V1-BACK-005 | StR-001; FR-007/044; NFR-010; no general Kani contract requirement | Amend | Define Kani profile, domain derivation, harness contract, tool pins and non-applicable constructs. | New |
| V1-BACK-006 | FR-114 replay agreement and ConfigVersion history are narrower | Amend | Define counterexample serialization and native-runtime verdict parity for every proof backend. | New |
| V1-BACK-007 | AD-004; FR-120–122/125; FR-297–299; IT-060 | Reuse | Expand target mapping coverage as language capabilities grow; preserve per-obligation loss. | X |
| V1-BACK-008 | AD-004; FR-120/121/123/125; FR-297–299; IT-060 | Reuse | Expand target mapping coverage as language/model capabilities grow. | X |
| V1-BACK-009 | AD-004; FR-120/121/124/125; FR-297–299; IT-060 | Reuse | Expand target mapping coverage as temporal/protocol capabilities grow. | X |
| V1-BACK-010 | AD-004; FR-125; FR-299; NFR-060 | Reuse | Apply package invariants to new target profiles. | X |
| V1-BACK-011 | AD-004; FR-121; FR-269/298 | Reuse | Require inventory-complete accounting for every expanded mapper. | X |
| V1-BACK-012 | AD-001/003/004; FR-019/120–124 | Reuse | Extend source-authority adverse cases to every new external mapping. | CF, X |
| V1-BACK-013 | AD-003/004; NFR-052/061 | Reuse | Add one cross-subsystem Rust containment requirement and executable-path inventory. | X, O, New |
| V1-BACK-014 | AD-001/003/004; FR-019/031/044/095/121 | Reuse | Extend exact-refusal cases to all newly admitted language constructs and methods. | CF, T, X |

## 8. Evidence, qualification, tooling and ecosystem

| Atomic ID | Coverage read | Class | Delta | Matrix |
| --- | --- | --- | --- | --- |
| V1-EVID-001 | FR-017/031/041/061/062/115; verification-reference artifacts | Amend | Define a uniform semantic-subject-to-requirement/assumption/method link for every subsystem. | V, CF |
| V1-EVID-002 | AD-001; FR-004/030/061/062; NFR-021/051/060 | Reuse | Extend provenance fields and acceptance cases to new methods/tools/adapters. | V, P |
| V1-EVID-003 | FR/TC matrices cover current bounded families only | Amend | Add feature-level conformance vectors for all 176 capabilities and integrity boundaries. | All/New |
| V1-EVID-004 | AD-001; FR-020/061/062/115; NFR-003/004 | Reuse | Add explicit language, runtime, monitor, analyzer and consuming-system qualification profiles. | V, New |
| V1-EVID-005 | FR-016/030/032; NFR-021/051/060 | Reuse | Consolidate exact reproducibility fields and apply them to every method. | V, New |
| V1-EVID-006 | FR-007/031/059/062; FR-248/249; no general counterexample contract | Amend | Define typed counterexample identity, serialization, minimization, replay and lineage. | V, New |
| V1-TOOL-001 | AD-001 pipeline and current code surface; no stable lifecycle API requirement | New | Specify Rust parse/check/lower/execute/replay/monitor/inspect API boundaries and compatibility. | New |
| V1-TOOL-002 | Current `quire-cli` exists; QSpec has no CLI lifecycle requirement | New | Specify commands, inputs/outputs, exit statuses, diagnostics and version discovery. | New |
| V1-TOOL-003 | AD-001 says formatting consumes syntax structures; no lossless CST contract | New | Define trivia/token preservation, source edits, recovery and identity behavior. | New |
| V1-TOOL-004 | FR-036/047 cover diagnostics only; no formatter/LSP/editor behavior | New | Define formatting stability, incremental diagnostics, language-server and editor APIs. | New |
| V1-TOOL-005 | Model metadata/view/rendering named only in historical scope prose | New | Define typed metadata queries, view subjects, deterministic rendering and loss. | New |
| V1-TOOL-006 | AD-003 refuses ambient extensions; no plugin contract | Resolve | Define explicit Rust plugin capability, purity, isolation, version, rights and provenance boundaries. | New |
| V1-TOOL-007 | No normative AOT/JIT behavior | New | Define semantic equivalence, cache identity, invalidation, resource and qualification rules. | New |
| V1-TOOL-008 | FR-002/030 cover imported model/package dependencies only | Amend | Define reusable semantic/domain library packaging, compatibility and migration. | New |
| V1-TOOL-009 | `quire-wasm` code surface has no QSpec owner | New | Define supported WASM API, bounds, deterministic behavior and exact refusal parity. | New |
| V1-TOOL-010 | `quire-analyze` and `quire-code-rs` exist without QSpec consumer contracts | New | Define non-authoritative analysis/codegen inputs, outputs, identities and loss behavior. | New |

## Cross-cutting conflicts and stale exclusions

The audit found six classes of normative repair that must precede bulk
authoring. These are current-text findings, not assumptions from issue labels:

1. **The scope authority is contradictory.** FR-020 says general realizability,
   synthesis and named research are not required V1 inventory. AD-001 calls
   sound local projection “later.” AD-003 refuses sets, bags, subtyping,
   recursion, float and unbounded logic in its selected V1 profiles. The atomic
   inventory now requires sound semantics or an explicit integrity boundary for
   each. AD-001/003 and FR-019/020/033/040 must be amended together.
2. **Integer division is internally split.** FR-007 defines truncation and
   remainder semantics with acceptance cases; FR-019, FR-037, FR-039 and FR-044
   refuse the same forms in composed profiles. The architecture must preserve
   historical profile meaning while selecting an explicit complete-V1 profile.
3. **Collection coverage is sequence-only.** FR-008/041 specify sequences and
   eight queries, while FR-019/040 reject sets, bags, conversions and flattening.
   No current requirement defines the missing collection kinds.
4. **The model layer is deliberately narrow.** Current requirements resolve
   authoritative declarations and finite object graphs, but do not define
   structural record equality, recursive values, conformance, inheritance,
   redefinition, dispatch or full typed systems-model references.
5. **Advanced methods are labels, not semantics.** FR-031/060 and FR-290–292
   preserve requested-claim identity for projection, realizability and
   refinement, but provide no algorithms, soundness premises, witness or
   counterexample contracts. Relational, hyperproperty, synthesis, simulation
   and model-testing behavior has no normative owner.
6. **Product tooling is absent from the standard.** Existing Rust CLI, WASM,
   analysis and code-generation repositories are code facts, not QSpec
   requirements. The current specification has no lossless CST, formatter,
   language-server, plugin, JIT, library or workflow-adapter contract.

## Current object-layer open gaps

Thirteen of the 90 current object artifacts contain an explicit `## Open gap`.
These are current default-branch facts; earlier object-review findings that were
subsequently repaired are not carried forward as live defects.

| Object | Current unresolved boundary | Owning atomic rows |
| --- | --- | --- |
| FR-203 source region | FR-003 layout deletion versus FR-036 newline-conversion prohibition. | V1-SRC-005/007 |
| FR-207 runtime value path | JSON field/index path, serialized step tags and ordering across observations. | V1-EXPR-023, V1-EVID-002 |
| FR-270 diagnostic catalog | Catalog digest domain and identity-kind classification. | V1-EXPR-023, V1-SRC-010 |
| FR-271 diagnostic code vocabulary | Seven meaning-only failures have no stable code spellings. | V1-EXPR-023 |
| FR-272 typed diagnostic cause | Cause payload fields and ordering lack stable names. | V1-EXPR-023 |
| FR-274 diagnostic location kind | Location discriminators and JSON path structure are undefined. | V1-SRC-007, V1-EXPR-023 |
| FR-215 protocol channel | FIFO key scope and missing duplication-premise disposition. | V1-PROT-005/013/017 |
| FR-227 owned choice | No evidence contract discharges role visibility. | V1-PROT-009/016/027/028 |
| FR-228 bounded control construct | Join-policy vocabulary and termination-declaration/finish-record identity. | V1-PROT-010/011/014 |
| FR-230 protocol obligation | Whole-execution-origin instance identity and key order. | V1-PROT-018, V1-TEMP-013 |
| FR-232 activation capture | Capture-set order and duplicate-expression behavior. | V1-PROT-019, V1-TEMP-010 |
| FR-234 compensation lifecycle | Closed state vocabulary, terminal states and resumption after incomplete input. | V1-PROT-020/023 |

Task-004 must allocate these gaps to stable subsystem owners before adding new
objects. Task-006 must resolve their semantic questions together with the
complete architecture; the audit itself does not invent answers.

## Existing artifact disposition

No current StR, US, FR, NFR, IT, object, TC or matrix is deleted by this audit.
The complete corpus is retained under these actions:

| Artifact family | Baseline count | Audit disposition |
| --- | ---: | --- |
| Stakeholder requirements | 2 | Reuse and amend StR-002 to name the complete product boundary. |
| User stories | 8 | Reuse; add stories for language authoring/tooling, model testing/analysis and workflow driving. |
| Functional requirements | 73 | Reuse exact state/temporal/protocol/observation/mapping semantics; amend or split the scope and extension owners above; add missing families. |
| Non-functional requirements | 13 | Reuse current bounded/reproducible/contained behavior; add quality constraints for new runtimes, analyzers and tooling. |
| Integration requirements | 11 | Reuse current handoffs; add model, analysis, proof, tooling and workflow-adapter integrations. |
| Object-layer FR | 90 | Preserve IDs and links; re-audit the current revision, resolve the 13 artifacts that still declare open gaps, and organize expansion without reviving revision-bound historical findings. |
| Test cases | 76 | Reuse exact cases; add cases for every amended/new acceptance criterion. |
| Test Matrices | 7 | Preserve family matrices; reorganize only after Task-004 determines scalable ownership; add complete atomic coverage. |
| ArchitectureDescriptions | 4 | Preserve AD-002/004 refinements; comprehensively amend AD-001/003 and add subsystem views only where Task-004/006 justify them. |
| Prior SpecReviews | 131 | Retain as revision-bound history; none reviews the Plan-008 complete-V1 scope. |

## Audit gate

The coverage map contains one row for every atomic inventory ID. Task-004 must
now decide the specification information architecture before any of the `New`,
`Amend`, `Split` or `Resolve` actions expand the corpus. Task-006 then reconciles
all architecture decisions in one coherent design. `/specify` remains blocked
until those gates and the epic reconciliation are complete.
