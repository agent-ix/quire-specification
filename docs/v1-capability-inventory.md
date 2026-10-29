# Quire V1 atomic capability inventory

This is the master list of capabilities that Quire V1 specifies. The
older 51-row matrix in `proposals/quire-v1/capabilities.md` is a historical
crosswalk; it does not remove or combine a row from this inventory.

## Inventory contract

Each row is atomic: it names one independently observable semantic, product,
analysis, integration or integrity obligation.

`Required` is product scope. `Integrity` is required behavior that rejects an
unsound claim, authority violation, unbounded request or prohibited side effect
with a typed, located outcome.

## 1. Source, package and identity

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-SRC-001 | Use native Quire as the sole editable formal-clause language. | Required | V1-001 |
| V1-SRC-002 | Select an exact language edition independently of profiles and wire formats. | Required | V1-001, V1-051 |
| V1-SRC-003 | Select an exact, ordered profile bundle. | Required | V1-001, V1-051 |
| V1-SRC-004 | Link state, temporal, protocol and analysis declarations into one package. | Required | V1-001, V1-040 |
| V1-SRC-005 | Preserve exact source bytes and content digest. | Required | V1-002 |
| V1-SRC-007 | Preserve lexical spans through parsing, linking, lowering and results. | Required | V1-002, V1-005 |
| V1-SRC-008 | Import one exact domain package by identity. | Required | V1-002, V1-003 |
| V1-SRC-009 | Bind qualified source names to typed domain declarations without inventing identities. | Required | V1-002, V1-003 |
| V1-SRC-010 | Keep semantic, source, artifact, run and evidence identities distinct. | Required | V1-002, V1-041 |
| V1-SRC-013 | Discover edition, profile, declaration-family and consumer features exactly. | Required | V1-051 |
| V1-SRC-014 | Extend a typed declaration or semantic family through an explicit extension boundary. | Required | V1-051 |
| V1-SRC-015 | Reject unknown syntax, profiles and feature spellings with a located typed cause. | Integrity | V1-005, V1-051 |
| V1-SRC-016 | Define an extension definition's grammar schema and typed-node schema, and refuse a missing member, a keyword or node-kind collision and a dependency cycle. | Integrity | V1-051 |

## 2. Type and model system

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-TYPE-001 | Resolve one declared type universe, from spec artifacts and native value types, across all semantic families. | Required | V1-003 |
| V1-TYPE-002 | Represent total Boolean values without truthiness coercion. | Required | V1-004 |
| V1-TYPE-003 | Represent inclusive finite signed-64 integer domains. | Required | V1-010 |
| V1-TYPE-004 | Represent exact normalized rational domains. | Required | V1-010; reserved `Rational` |
| V1-TYPE-005 | Represent bounded decimal syntax through exact, non-floating semantics. | Required | reserved `Decimal`; scope census |
| V1-TYPE-006 | Represent text values and their exact comparison domain. | Required | V1-014 |
| V1-TYPE-007 | Represent enumerations with declaration-qualified member identity. | Required | V1-014 |
| V1-TYPE-008 | Represent quantities with explicit dimensions and units. | Required | V1-004, V1-014; prose census |
| V1-TYPE-009 | Convert compatible units exactly and reject dimension or precision loss. | Required | prose census |
| V1-TYPE-010 | Represent independently authored presence, absence and explicit null. | Required | V1-013 |
| V1-TYPE-011 | Represent lower and upper multiplicity independently of presence. | Required | V1-013 |
| V1-TYPE-012 | Represent typed fixed records and tuple values. | Required | V1-014; reserved `tuple`, `Tuple` |
| V1-TYPE-013 | Compare records structurally under an explicit type-owned equality rule. | Required | V1-018 |
| V1-TYPE-014 | Represent finite ordered duplicate-preserving sequences. | Required | V1-015 |
| V1-TYPE-015 | Represent finite mathematical sets without sequence emulation. | Required | V1-016; reserved `set`, `Set` |
| V1-TYPE-016 | Represent finite bags with explicit multiplicity semantics. | Required | V1-016; reserved `bag`, `Bag` |
| V1-TYPE-017 | Represent finite ordered sets with distinct order and uniqueness semantics. | Required | V1-016; reserved `OrderedSet` |
| V1-TYPE-018 | Convert collection kinds only through explicit loss-checked operations. | Required | V1-016 |
| V1-TYPE-019 | Represent typed acyclic value containment. | Required | V1-014 |
| V1-TYPE-020 | Represent recursive value containment with finite runtime bounds. | Required | V1-016; prose census |
| V1-TYPE-021 | Represent finite object populations independently of value containment. | Required | V1-017 |
| V1-TYPE-022 | Represent explicit typed object references and identity equality. | Required | V1-017 |
| V1-TYPE-023 | Resolve object lookup only through an explicit closed typed environment. | Required | V1-018; reserved `allInstances` |
| V1-TYPE-024 | Traverse typed relationships and compute exact bounded reachability. | Required | V1-017 |
| V1-TYPE-025 | Preserve original and effective declaration identity through specialization. | Required | V1-019; prose census |
| V1-TYPE-026 | Type-check inheritance and conformance. | Required | V1-019 |
| V1-TYPE-027 | Type-check subsetting and redefinition without provenance loss. | Required | V1-019 |
| V1-TYPE-028 | Resolve dynamic dispatch against an explicit closed method set. | Required | V1-019 |
| V1-TYPE-029 | Consume parts, ports, interfaces, connections and allocations as typed model references. | Required | V1-019; prose census |
| V1-TYPE-030 | Reject implicit numeric, collection, record and reference casts. | Integrity | V1-012, V1-016, V1-018; reserved `cast` |
| V1-TYPE-031 | Represent mathematical unbounded integers while requiring finite domains only from bounded consumers. | Required | V1-011 |
| V1-TYPE-032 | Represent closed sum types through declared union members with typed payload arity and member-qualified equality. | Required | prose census; reserved `union` |
| V1-TYPE-033 | Admit union member payload recursion exactly when it escapes, on the same named-edge basis as record and tuple fields. | Required | prose census |
| V1-TYPE-034 | Check a `case` expression's arm set for duplicate, unknown, mis-arity and missing members against its scrutinee's declared union before evaluation. | Required | prose census |

## 3. Expressions, queries and contracts

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-EXPR-001 | Bind immutable local values with lexical scope. | Required | V1-004 |
| V1-EXPR-002 | Declare and call acyclic named typed predicates. | Required | V1-007 |
| V1-EXPR-003 | Declare and call total pure typed functions. | Required | V1-008; reserved `helper` |
| V1-EXPR-004 | Admit recursive pure functions only with checked termination and resource bounds. | Required | V1-008; reserved `rec` |
| V1-EXPR-005 | Evaluate checked integer addition, subtraction, multiplication and negation. | Required | V1-010 |
| V1-EXPR-006 | Evaluate integer division and remainder with exact zero and overflow outcomes. | Required | V1-012 |
| V1-EXPR-007 | Evaluate normalized rational arithmetic and division. | Required | V1-010 |
| V1-EXPR-008 | Evaluate finite-width floating values under an explicit IEEE profile. | Required | V1-012; prose census |
| V1-EXPR-009 | Preserve NaN, infinity, signed zero and rounding-mode semantics where the float profile admits them. | Required | V1-012 |
| V1-EXPR-010 | Evaluate collection size, membership and count. | Required | V1-015; scope census |
| V1-EXPR-011 | Evaluate bounded universal and existential quantification. | Required | V1-015 |
| V1-EXPR-012 | Filter a collection while preserving its kind and order contract. | Required | V1-015; scope census |
| V1-EXPR-013 | Map or collect collection elements with typed output bounds. | Required | V1-015; reserved `collect` |
| V1-EXPR-014 | Flatten nested collections with explicit kind, multiplicity and bound semantics. | Required | V1-016; reserved `flatten` |
| V1-EXPR-015 | Fold bounded collections with exact empty identities and checked intermediates. | Required | V1-015; scope census |
| V1-EXPR-016 | Compare scalar, enum, reference, record and collection values only under declared equality rules. | Required | V1-018 |
| V1-EXPR-017 | Navigate properties and relationships without ambient identity lookup. | Required | V1-017, V1-018 |
| V1-EXPR-018 | Evaluate invariants against one exact state anchor. | Required | V1-009 |
| V1-EXPR-019 | Evaluate operation preconditions against exact current and pre-state anchors. | Required | V1-009 |
| V1-EXPR-020 | Evaluate operation postconditions against exact pre/post anchors and result. | Required | V1-009 |
| V1-EXPR-021 | Evaluate immutable captures and aliases without anchor drift. | Required | V1-009 |
| V1-EXPR-022 | Enforce operation frames, created identities and deleted identities. | Required | V1-009 |
| V1-EXPR-023 | Report parse, type, admission, lowering, execution and observation failures as distinct stages. | Required | V1-005 |
| V1-EXPR-024 | Reject reflection, ambient extension lookup and unreviewed user callbacks. | Integrity | V1-008 |
| V1-EXPR-025 | Reject mutation and I/O during declarative clause evaluation. | Integrity | V1-008, V1-039 |
| V1-EXPR-026 | Report resource exhaustion and undefined arithmetic without Boolean approximation. | Integrity | V1-005, V1-010 |
| V1-EXPR-027 | Evaluate unbounded universal and existential quantification. | Required | V1-015 |
| V1-EXPR-028 | Admit inductive verification obligations over unbounded and recursive domains. | Required | V1-038 |
| V1-EXPR-029 | Admit compositional verification obligations over independently verified interface-connected components. | Required | V1-037 |

## 4. Temporal, clocks and observation

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-TEMP-001 | Evaluate bounded `eventually`. | Required | V1-020 |
| V1-TEMP-002 | Evaluate bounded `always`. | Required | V1-020 |
| V1-TEMP-003 | Evaluate bounded `until`. | Required | V1-020 |
| V1-TEMP-004 | Evaluate bounded `release`. | Required | V1-020 |
| V1-TEMP-005 | Evaluate bounded previous, historically, once, since and triggered operators. | Required | V1-021 |
| V1-TEMP-006 | Bind an explicit past-history origin and cutoff. | Required | V1-021, V1-025 |
| V1-TEMP-007 | Evaluate over event-position clocks. | Required | V1-022 |
| V1-TEMP-008 | Evaluate over fixed-period sampled clocks with epoch, period and unit. | Required | V1-022 |
| V1-TEMP-009 | Evaluate exact timestamped finite-window deadlines. | Required | V1-023 |
| V1-TEMP-010 | Capture trigger values at one immutable activation instant. | Required | V1-023 |
| V1-TEMP-011 | Advance timeout progress during event silence from authoritative clock progress. | Required | V1-023 |
| V1-TEMP-012 | Distinguish closed execution, valid open prefix and incomplete input. | Required | V1-024 |
| V1-TEMP-013 | Report activation independently from verdict and settlement. | Required | V1-024, V1-041 |
| V1-TEMP-014 | Apply an explicit late-data cutoff and retain supersession lineage. | Required | V1-025 |
| V1-TEMP-015 | Bind an authoritative completeness claim to its exact population and window. | Required | V1-025, V1-050 |
| V1-TEMP-016 | Represent bounded clock uncertainty intervals. | Required | V1-026 |
| V1-TEMP-017 | Repair out-of-order observations through explicit revisioned replay. | Required | V1-026 |
| V1-TEMP-018 | Evaluate partial proposition valuations without coercing missing values to false. | Required | V1-026 |
| V1-TEMP-019 | Lower native temporal syntax through a versioned TL correspondence. | Required | V1-028 |
| V1-TEMP-020 | Evaluate temporal atoms through native typed predicate semantics. | Required | V1-028 |
| V1-TEMP-021 | Refuse an infinite-trace or unbounded-liveness proof claim from finite evidence. | Integrity | V1-027 |
| V1-TEMP-022 | Refuse clock-unit, boundary and false-extension correspondence mismatches. | Integrity | V1-022, V1-028 |
| V1-TEMP-023 | Author and soundly analyze unbounded temporal properties under an explicit infinite-trace semantic profile. | Required | V1-027; prose census |
| V1-TEMP-024 | Admit the unbounded (no interval bound) temporal operator grammar edition only under the selected infinite-trace profile. | Required | V1-027; prose census |
| V1-TEMP-025 | Map infinite-trace proved/refuted/inconclusive/unsupported/failed results onto the existing execution, truth and settlement-basis vocabulary. | Required | V1-027; prose census |
| V1-TEMP-026 | Settle an infinite-trace claim with no registered liveness backend as unsupported with a warning naming the missing capability. | Integrity | V1-027; prose census |

## 5. Protocol and choreography

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-PROT-001 | Declare finite participant roles. | Required | V1-029 |
| V1-PROT-002 | Identify runtime role instances independently of role definitions. | Required | V1-029 |
| V1-PROT-003 | Bind roles to model components and endpoints explicitly. | Required | V1-029 |
| V1-PROT-004 | Declare typed messages and payloads. | Required | V1-030 |
| V1-PROT-005 | Declare channels independently of participants. | Required | V1-030 |
| V1-PROT-006 | Distinguish send, receipt, message, delivery and attempt identities. | Required | V1-030 |
| V1-PROT-007 | Bind business correlation and related workflow identities explicitly. | Required | V1-030, V1-049 |
| V1-PROT-008 | Compose authored finite sequence. | Required | V1-031 |
| V1-PROT-009 | Compose owned labeled choice. | Required | V1-031 |
| V1-PROT-010 | Compose parallel branches and explicit join. | Required | V1-031 |
| V1-PROT-011 | Compose bounded iteration. | Required | V1-031 |
| V1-PROT-012 | Spawn roles dynamically under explicit finite population and lifetime bounds. | Required | V1-038 |
| V1-PROT-013 | Model queues with explicit finite or symbolic resource bounds. | Required | V1-038 |
| V1-PROT-014 | Model loop invariants and termination/resource obligations beyond literal bounded repeat. | Required | V1-038 |
| V1-PROT-015 | Preserve authored causal edges independently of timestamp order. | Required | V1-032 |
| V1-PROT-016 | Enforce choice visibility for each deciding and affected role. | Required | V1-032 |
| V1-PROT-017 | Declare delivery cardinality, ordering and duplication premises. | Required | V1-032 |
| V1-PROT-018 | Attach per-instance state guards and temporal obligations. | Required | V1-033 |
| V1-PROT-019 | Attach immutable captures and inclusive timeout branches. | Required | V1-033 |
| V1-PROT-020 | Register compensation only after its matching successful effect. | Required | V1-034 |
| V1-PROT-021 | Distinguish effect, action, attempt, retry, commit and recovery identities. | Required | V1-034 |
| V1-PROT-022 | Enforce commit and recovery relationships. | Required | V1-034 |
| V1-PROT-023 | Distinguish failed, timed-out, retried, untriggered and partially recovered outcomes. | Required | V1-035 |
| V1-PROT-024 | Detect duplicate business effects independently of duplicate delivery. | Required | V1-035 |
| V1-PROT-025 | Replay a supplied finite trace reproducibly. | Required | V1-036, V1-047 |
| V1-PROT-026 | Assess global conformance to a selected protocol. | Required | V1-036 |
| V1-PROT-027 | Project a global protocol to sound local role obligations. | Required | V1-037 |
| V1-PROT-028 | Analyze protocol realizability with explicit premises and counterexamples. | Required | V1-037 |
| V1-PROT-029 | Analyze assume-guarantee composition. | Required | V1-037 |
| V1-PROT-030 | Check implementation refinement against a selected protocol. | Required | V1-037 |
| V1-PROT-031 | Observe or recommend workflow actions without executing external business side effects. | Integrity | V1-039 |

## 6. Runtime, analysis and workflows

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-RUN-001 | Execute admitted state clauses in the native reference runtime. | Required | V1-047 |
| V1-RUN-002 | Execute admitted temporal obligations in replay mode. | Required | V1-047 |
| V1-RUN-003 | Execute admitted protocol obligations in replay mode. | Required | V1-047 |
| V1-RUN-004 | Incrementally monitor temporal and protocol obligations. | Required | V1-047 |
| V1-RUN-005 | Retain independent execution, truth, activation and settlement dimensions. | Required | V1-041 |
| V1-RUN-006 | Retain independent participation, assumptions, completeness and provenance dimensions. | Required | V1-041 |
| V1-RUN-007 | Assess finite declared snapshots and event-time populations. | Required | V1-050 |
| V1-RUN-008 | Evaluate aggregates over reservation, refund and related workflow populations. | Required | V1-049, V1-050 |
| V1-RUN-009 | Relate order, shipment, payment-attempt and refund workflow instances. | Required | V1-049 |
| V1-RUN-010 | Run the state workflow reference example end to end. | Required | V1-042 |
| V1-RUN-011 | Run the timed-refund reference example end to end. | Required | V1-042 |
| V1-RUN-012 | Run branching compensated choreography with fan-out and join end to end. | Required | V1-042 |
| V1-RUN-013 | Simulate finite nondeterministic model behaviors without external side effects. | Required | prose census |
| V1-RUN-014 | Generate model-based test traces and shrink reproducible counterexamples. | Required | prose census |
| V1-RUN-015 | Analyze relational properties across two or more executions. | Required | V1-045 |
| V1-RUN-016 | Analyze hyperproperties over explicit finite trace sets. | Required | V1-045 |
| V1-RUN-017 | Author a verification-plan DSL that selects methods, bounds and evidence obligations without duplicating clauses. | Required | V1-045 |
| V1-RUN-018 | Analyze continuous and hybrid dynamics under an explicit numeric/time model. | Required | prose census |
| V1-RUN-019 | Perform bounded synthesis while distinguishing candidates from proof. | Required | prose census |
| V1-RUN-020 | Drive an external workflow through typed decisions while keeping effects in an authorized adapter. | Required | prose census; V1-039 boundary |

## 7. IR, proof backends and output mappings

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-BACK-001 | Request backend capabilities independently for each clause or claim. | Required | V1-006 |
| V1-BACK-002 | Preserve supported sibling results when another requested capability refuses or exhausts. | Required | V1-006 |
| V1-BACK-003 | Lower typed clauses to versioned IR without creating a second authored language. | Required | V1-006, V1-040 |
| V1-BACK-004 | Execute Boolean, integer, rational, collection, graph, temporal and protocol IR with typed non-Boolean outcomes. | Required | V1-006 |
| V1-BACK-005 | Generate bounded Kani contracts and harnesses from model domains. | Required | V1-006, V1-048 |
| V1-BACK-006 | Replay every backend counterexample through the native runtime with the same verdict. | Required | V1-006, V1-047 |
| V1-BACK-007 | Emit OCL 2.4 with source-bound preserved, conditional, unrepresented and refused accounting. | Required | V1-043 |
| V1-BACK-008 | Emit SysML 2.0 and KerML 1.0 with source-bound loss accounting. | Required | V1-043 |
| V1-BACK-009 | Emit FRETish 3.1 with source-bound loss accounting. | Required | V1-043 |
| V1-BACK-010 | Produce deterministic immutable output packages. | Required | V1-043, V1-046 |
| V1-BACK-011 | Account for every source obligation in every requested output mapping. | Required | V1-043 |
| V1-BACK-012 | Keep external source-language imports out of the native authority chain. | Integrity | V1-044 |
| V1-BACK-013 | Keep Java, Node and Electron runtimes out of production and qualification semantics. | Integrity | V1-044 |
| V1-BACK-014 | Refuse a backend request it cannot implement exactly; never approximate. | Integrity | V1-005, V1-006 |
| V1-BACK-015 | Mint native-run-result/2 as a new closed wire carrying the separating-witness record, selected explicitly and never inferred from /1 content. | Required | V1-006, V1-047 |
| V1-BACK-016 | Refuse silent substitution between native-run-result/1 and /2; a consumer requiring the witness record regenerates against /2. | Integrity | V1-006, V1-047 |
| V1-BACK-017 | Check that a decomposed requirement's admitted result entails its parent's admitted result through a registered implication backend; absent that backend, the claim settles `unsupported` with a warning naming the required capability. | Required | V1-037; prose census |
| V1-BACK-018 | Catalog exact lowering targets and the FR-290 capabilities each backend advertises for registration and negotiation. | Required | V1-006 |
| V1-BACK-019 | Register an SMT backend (cvc5, with Z3 as an independent cross-check) advertising unbounded quantification, induction, composition and mathematical-integer capabilities. | Required | V1-006; prose census |
| V1-BACK-020 | Emit Verus `requires`/`ensures` and loop invariants from QSL state contracts onto named implementation Rust functions. | Required | V1-006; prose census |
| V1-BACK-021 | Declare a model-to-implementation abstraction relation from model objects, populations and operation frames to implementation state. | Required | V1-006; prose census |
| V1-BACK-022 | Refuse emission onto implementation code that references a model element with no bound abstraction relation, naming the element. | Integrity | V1-BACK-021; prose census |
| V1-BACK-023 | Record a proved settlement's certification: `certified` when the core's Alethe checker verifies the cvc5 certificate for the core's own query, `uncertified` when the certificate is absent or unverifiable. | Required | V1-BACK-019; prose census |
| V1-BACK-024 | Return the same verdict from the SMT encoding and the native runtime on a shared differential corpus. | Required | V1-BACK-019; prose census |

## 8. Evidence, qualification, tooling and ecosystem

| ID | Atomic capability | Class | Origin |
| --- | --- | --- | --- |
| V1-EVID-001 | Bind requirements, assumptions and verification links to exact semantic subjects. | Required | V1-041, V1-048; prose census |
| V1-EVID-003 | Publish conformance vectors for every admitted semantic feature and integrity refusal. | Required | V1-048 |
| V1-EVID-004 | Qualify language conformance independently from monitor correctness and system qualification. | Required | V1-048 |
| V1-EVID-006 | Preserve typed counterexamples as replayable evidence. | Required | V1-047, V1-048 |
| V1-EVID-007 | Retain the separating-witness record's exact deciding element, index, value path and trace position for every decisive settlement. | Required | V1-047, V1-048 |
| V1-EVID-008 | Retain one family-neutral separating witness or counterexample shape across collection, temporal and protocol settlement. | Required | V1-047, V1-048 |
| V1-EVID-009 | Leave an exhausted, cancelled or unsupported result witness-free rather than substituting a partial or synthesized witness. | Integrity | V1-047, V1-048 |
| V1-TOOL-001 | Expose parse, check, lower, execute, replay, monitor and inspect through stable Rust APIs. | Required | code-surface census |
| V1-TOOL-002 | Expose the same lifecycle through a stable CLI. | Required | code-surface census |
| V1-TOOL-003 | Preserve a lossless concrete syntax tree for editors and source transformations. | Required | prose census |
| V1-TOOL-004 | Provide formatter, language-server and editor diagnostics over the lossless source model. | Required | prose census |
| V1-TOOL-005 | Provide model metadata, semantic views and deterministic rendering APIs. | Required | prose census |
| V1-TOOL-006 | Load Rust plugins only through explicit capability and purity contracts. | Required | prose census |
| V1-TOOL-007 | Support ahead-of-time and JIT execution without changing language meaning or qualification identity. | Required | prose census |
| V1-TOOL-008 | Package reusable domain libraries selectable by edition and profile. | Required | prose census |
| V1-TOOL-010 | Expose code-analysis and source-generation consumers without granting them semantic authority. | Required | code-surface census |
| V1-TOOL-011 | Confirm a superseding specification edition admits every result its prior edition admitted. | Required | V1-037; prose census |
| V1-TOOL-012 | Preserve every parent profile's refusal as its child profile's refusal under profile layering. | Required | V1-037; prose census |

## Source census

The inventory was formed from three independent source classes. A source being
listed here means it has been accounted for, not that its prior scope decision
remains authoritative.

| Source class | Enumerated input | Atomic accounting |
| --- | --- | --- |
| Historical capability groups | Every `V1-001` through `V1-051` row in `proposals/quire-v1/capabilities.md` | Every group identifier appears in one or more `Origin` cells above; compound groups were split. |
| Current compiler refusals | `helper`, `rec`, `set`, `bag`, `Set`, `Bag`, `OrderedSet`, `collect`, `flatten`, `cast`, `tuple`, `Tuple`, `Decimal`, `Rational`, `allInstances` in the QSL tokenizer | Each spelling maps to V1-EXPR-003/004/013/014, V1-TYPE-004/005/012/015/016/017/023/030. Case variants share semantics but remain enumerated here. |
| Prose-only exclusions | richer numerics and units; recursive values; inherited/redefined models; uncertainty/repair; projection/realizability/refinement; dynamic protocols; relational and hyperproperty analysis; simulation, synthesis and workflow driving; editor/CST/tool APIs; plugins/JIT/libraries | Each named item has a Required row above and therefore cannot disappear behind prose. |

## Table invariants

1. every inventory row begins with one unique `V1-` ID;
2. every row has class `Required` or `Integrity`;
3. all 51 historical group IDs and all 15 compiler refusal spellings occur in
   the source census or an `Origin` cell.
