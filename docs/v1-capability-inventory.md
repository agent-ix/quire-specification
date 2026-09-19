# Quire V1 atomic capability inventory

Status: authoritative scope inventory, established by Plan-008 on 2026-09-15.
Tracking owners: [quire-specification #1](https://github.com/agent-ix/quire-specification/issues/1),
[#9](https://github.com/agent-ix/quire-specification/issues/9), and
[#63](https://github.com/agent-ix/quire-specification/issues/63), under
[quire-research #36](https://github.com/agent-ix/quire-research/issues/36).

This is the single master list of capabilities that the complete Quire V1
campaign must specify, implement where executable, verify, integrate and
qualify. The older 51-row matrix in `proposals/quire-v1/capabilities.md` and the
13 CAP coordination groups in `docs/functional-capability-ledger.md` remain
historical crosswalks and roll-ups. They do not remove, defer or combine a row
from this inventory. The current per-row specification evidence is the
[complete V1 atomic capability and requirement matrix](../spec/complete-v1/tests.md)
(TM-009), which names the owning requirement, the specification action and the
test cases for each row.

## Inventory contract

Each row is atomic: it names one independently observable semantic, product,
analysis, integration or integrity obligation. Every row has a current program
owner even where Task-005 must repair that owner's issue graph. A missing
implementation is work to schedule, not a scope disposition.

`Required` is product scope. `Integrity` is required behavior that rejects an
unsound claim, authority violation, unbounded request or prohibited side effect
with a typed, located outcome. Integrity rows are delivered and tested; they are
not aliases for “later,” “unsupported without accounting,” or silent omission.

The five delivery columns use these fixed values:

- `C`: complete evidence exists for this exact atomic obligation.
- `P`: partial or bounded evidence exists; the complete obligation remains open.
- `M`: evidence is missing and must be created by the delivery campaign.
- `N/A`: the stage does not apply to this obligation.

The live primary implementation and qualification ticket for every row is in
the [Task-009 implementation ticket manifest](v1-delivery-ticket-manifest.md).
That manifest assigns five autonomous Agent-A through Agent-E lanes and is
audited before Task-010 opens product implementation.

`Spec`, `Impl`, `Ver`, `Int`, and `Qual` mean normative specification,
production implementation, direct verification, cross-component integration,
and release/claim qualification respectively. Completion in one column never
implies completion in another.

`Int` and `Qual` are inception scores, fixed at Plan-008's first commit of
this inventory. `Int` is a Plan-008 code-surface census. `Qual` is uniformly
`M`. It records that every capability lacks release qualification evidence:
each row's qualification ticket in the
[ticket manifest](v1-delivery-ticket-manifest.md) is one of eight open lane
tickets — QSL #123 (89 rows), Protocol #82 (34), Contract IR #101 (16), QObs
#26 (14), Research #78 (12), TL Syntax #74 (11), Verification #38 (7) and
Codegen #50 (5) —
and `TC-231`, the aggregate qualification corpus verifying `FR-311`, carries
zero traced tests on any implementation default branch. Per-row measurement
of `Int` and `Qual` remains scope for the A07 lane.

`Impl` and `Ver` are row-level, established by citing implementing code and
an executing test that asserts the exact criterion it is tagged with. Merged
code contains mis-bound tags, and the implementation repositories reuse this
repository's `FR-0NN` numbers for different requirements, so each row's
evidence here rests on that cited code and test, read against the exact
criterion this ledger names, rather than a bare `FR-0NN-AC-M` tag spelling.

## 1. Source, package and identity

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-SRC-001 | Use native Quire as the sole editable formal-clause language. | Required | V1-001 | C | C | P | P | M | QSpec #1 / research #36 |
| V1-SRC-002 | Select an exact language edition independently of profiles and wire formats. | Required | V1-001, V1-051 | C | P | P | P | M | QSpec #1 / research #36 |
| V1-SRC-003 | Select an exact, ordered profile bundle. | Required | V1-001, V1-051 | C | P | P | P | M | QSpec #1 / research #36 |
| V1-SRC-004 | Link state, temporal, protocol and analysis declarations into one package. | Required | V1-001, V1-040 | C | P | P | P | M | QSL #1 / research #39 |
| V1-SRC-005 | Preserve exact source bytes and content digest. | Required | V1-002 | C | P | P | P | M | QSL #1 / research #39 |
| V1-SRC-006 | Preserve source revision independently of source bytes. | Required | V1-002 | C | P | P | P | M | QSL #1 / research #39 |
| V1-SRC-007 | Preserve lexical spans through parsing, linking, lowering and results. | Required | V1-002, V1-005 | C | P | P | P | M | QSL #68 / research #39 |
| V1-SRC-008 | Import one exact domain package by identity and `sha256-jcs` digest. | Required | V1-002, V1-003 | C | P | P | P | M | QSL #117 / research #39 |
| V1-SRC-009 | Bind qualified source names to typed domain declarations without inventing identities. | Required | V1-002, V1-003 | C | P | P | P | M | QSL #1 / QSL #117 |
| V1-SRC-010 | Keep semantic, source, artifact, run and evidence identities distinct. | Required | V1-002, V1-041 | C | P | P | P | M | QSpec #1 / research #39 |
| V1-SRC-011 | Preserve exact dependency manifests and implementation pins. | Required | V1-046 | C | P | P | P | M | QSpec #1 / research #40 |
| V1-SRC-012 | Migrate definitions explicitly without relabeling historical bytes or evidence. | Required | V1-046, V1-051 | C | P | P | P | M | QSpec #1 / research #40 |
| V1-SRC-013 | Discover edition, profile, declaration-family and consumer features exactly. | Required | V1-051 | C | P | P | P | M | QSL #1 / research #39 |
| V1-SRC-014 | Extend a typed declaration or semantic family through a versioned extension boundary. | Required | V1-051 | C | P | P | M | M | QSpec #1 / research #36 |
| V1-SRC-015 | Reject unknown syntax, profiles and feature spellings with a located typed cause. | Integrity | V1-005, V1-051 | C | C | P | P | M | QSL #1 / research #39 |

## 2. Type and model system

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-TYPE-001 | Resolve one declared type universe, from spec artifacts and native value types, across all semantic families. | Required | V1-003 | C | P | P | P | M | QSL #118 / research #39 |
| V1-TYPE-002 | Represent total Boolean values without truthiness coercion. | Required | V1-004 | C | C | C | P | M | QSL #1 / research #39 |
| V1-TYPE-003 | Represent inclusive finite signed-64 integer domains. | Required | V1-010 | C | C | C | P | M | QSL #1 / research #39 |
| V1-TYPE-004 | Represent exact normalized rational domains. | Required | V1-010; reserved `Rational` | C | M | M | M | M | research #66 / QSL #1 |
| V1-TYPE-005 | Represent bounded decimal syntax through exact, non-floating semantics. | Required | reserved `Decimal`; scope census | C | M | M | M | M | research #66 / QSL #1 |
| V1-TYPE-006 | Represent text values and their exact comparison domain. | Required | V1-014 | C | P | P | P | M | QSL #1 / research #39 |
| V1-TYPE-007 | Represent enumerations with declaration-qualified member identity. | Required | V1-014 | C | P | P | P | M | QSL #1 / QSL #118 |
| V1-TYPE-008 | Represent quantities with explicit dimensions and units. | Required | V1-004, V1-014; prose census | C | M | M | M | M | research #66 / QSpec #1 |
| V1-TYPE-009 | Convert compatible units exactly and reject dimension or precision loss. | Required | prose census | C | M | M | M | M | research #66 / QSpec #1 |
| V1-TYPE-010 | Represent independently authored presence, absence and explicit null. | Required | V1-013 | C | C | C | P | M | QSL #119 / research #39 |
| V1-TYPE-011 | Represent lower and upper multiplicity independently of presence. | Required | V1-013 | C | C | C | P | M | QSL #119 / research #39 |
| V1-TYPE-012 | Represent typed fixed records and tuple values. | Required | V1-014; reserved `tuple`, `Tuple` | C | C | C | P | M | research #66 / QSL #1 |
| V1-TYPE-013 | Compare records structurally under an explicit type-owned equality rule. | Required | V1-018 | C | C | C | M | M | research #66 / QSL #1 |
| V1-TYPE-014 | Represent finite ordered duplicate-preserving sequences. | Required | V1-015 | C | C | C | P | M | QSL #1 / research #39 |
| V1-TYPE-015 | Represent finite mathematical sets without sequence emulation. | Required | V1-016; reserved `set`, `Set` | C | C | C | M | M | research #65 / QSL #1 |
| V1-TYPE-016 | Represent finite bags with explicit multiplicity semantics. | Required | V1-016; reserved `bag`, `Bag` | C | C | C | M | M | research #65 / QSL #1 |
| V1-TYPE-017 | Represent finite ordered sets with distinct order and uniqueness semantics. | Required | V1-016; reserved `OrderedSet` | C | C | C | M | M | research #65 / QSL #1 |
| V1-TYPE-018 | Convert collection kinds only through explicit loss-checked operations. | Required | V1-016 | C | C | C | M | M | research #65 / QSL #1 |
| V1-TYPE-019 | Represent typed acyclic value containment. | Required | V1-014 | C | C | C | P | M | QSL #1 / QSL #119 |
| V1-TYPE-020 | Represent recursive value containment with finite runtime bounds. | Required | V1-016; prose census | C | C | C | M | M | research #65 / QSL #119 |
| V1-TYPE-021 | Represent finite object populations independently of value containment. | Required | V1-017 | C | P | P | P | M | QSL #1 / research #41 |
| V1-TYPE-022 | Represent explicit typed object references and identity equality. | Required | V1-017 | C | P | P | P | M | QSL #1 / QSL #120 |
| V1-TYPE-023 | Resolve object lookup only through an explicit closed typed environment. | Required | V1-018; reserved `allInstances` | C | P | P | P | M | research #66 / QSL #1 |
| V1-TYPE-024 | Traverse typed relationships and compute exact bounded reachability. | Required | V1-017 | C | P | P | P | M | QSL #1 / research #39 |
| V1-TYPE-025 | Preserve original and effective declaration identity through specialization. | Required | V1-019; prose census | C | M | M | M | M | research #66 / QSL #120 |
| V1-TYPE-026 | Type-check inheritance and conformance. | Required | V1-019 | C | M | M | M | M | research #66 / QSL #1 |
| V1-TYPE-027 | Type-check subsetting and redefinition without provenance loss. | Required | V1-019 | C | M | M | M | M | research #66 / QSL #1 |
| V1-TYPE-028 | Resolve dynamic dispatch against an explicit closed method set. | Required | V1-019 | C | M | M | M | M | research #66 / QSL #1 |
| V1-TYPE-029 | Consume parts, ports, interfaces, connections and allocations as typed model references. | Required | V1-019; prose census | C | M | M | M | M | research #66 / QSL #120 |
| V1-TYPE-030 | Reject implicit numeric, collection, record and reference casts. | Integrity | V1-012, V1-016, V1-018; reserved `cast` | C | P | P | P | M | research #66 / QSL #1 |
| V1-TYPE-031 | Represent mathematical unbounded integers while requiring finite domains only from bounded consumers. | Required | V1-011 | C | M | M | M | M | research #66 / QSL #1 |

## 3. Expressions, queries and contracts

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-EXPR-001 | Bind immutable local values with lexical scope. | Required | V1-004 | C | C | C | P | M | QSL #1 / research #39 |
| V1-EXPR-002 | Declare and call acyclic named typed predicates. | Required | V1-007 | C | C | C | P | M | QSL #1 / research #39 |
| V1-EXPR-003 | Declare and call total pure typed functions. | Required | V1-008; reserved `helper` | C | C | C | M | M | research #65 / QSL #1 |
| V1-EXPR-004 | Admit recursive pure functions only with checked termination and resource bounds. | Required | V1-008; reserved `rec` | C | C | C | M | M | research #65 / QSL #1 |
| V1-EXPR-005 | Evaluate checked integer addition, subtraction, multiplication and negation. | Required | V1-010 | C | C | P | P | M | QSL #1 / Contract IR #1 |
| V1-EXPR-006 | Evaluate integer division and remainder with exact zero and overflow outcomes. | Required | V1-012 | C | M | M | M | M | research #66 / QSL #1 |
| V1-EXPR-007 | Evaluate normalized rational arithmetic and division. | Required | V1-010 | C | M | M | M | M | research #66 / QSL #1 |
| V1-EXPR-008 | Evaluate finite-width floating values under an explicit IEEE profile. | Required | V1-012; prose census | C | M | M | M | M | research #66 / QSL #1 |
| V1-EXPR-009 | Preserve NaN, infinity, signed zero and rounding-mode semantics where the float profile admits them. | Required | V1-012 | C | M | M | M | M | research #66 / QSL #1 |
| V1-EXPR-010 | Evaluate collection size, membership and count. | Required | V1-015; scope census | C | C | C | P | M | research #65 / QSL #1 |
| V1-EXPR-011 | Evaluate bounded universal and existential quantification. | Required | V1-015 | C | C | C | P | M | QSL #1 / Contract IR #1 |
| V1-EXPR-012 | Filter a collection while preserving its kind and order contract. | Required | V1-015; scope census | C | C | C | P | M | research #65 / QSL #1 |
| V1-EXPR-013 | Map or collect collection elements with typed output bounds. | Required | V1-015; reserved `collect` | C | C | C | P | M | research #65 / QSL #1 |
| V1-EXPR-014 | Flatten nested collections with explicit kind, multiplicity and bound semantics. | Required | V1-016; reserved `flatten` | C | C | C | M | M | research #65 / QSL #1 |
| V1-EXPR-015 | Fold bounded collections with exact empty identities and checked intermediates. | Required | V1-015; scope census | C | C | C | P | M | research #65 / QSL #1 |
| V1-EXPR-016 | Compare scalar, enum, reference, record and collection values only under declared equality rules. | Required | V1-018 | C | P | P | P | M | research #66 / QSL #1 |
| V1-EXPR-017 | Navigate properties and relationships without ambient identity lookup. | Required | V1-017, V1-018 | C | P | P | P | M | QSL #1 / QSL #120 |
| V1-EXPR-018 | Evaluate invariants against one exact state anchor. | Required | V1-009 | C | C | P | P | M | QSL #1 / research #39 |
| V1-EXPR-019 | Evaluate operation preconditions against exact current and pre-state anchors. | Required | V1-009 | C | C | P | P | M | QSL #1 / research #39 |
| V1-EXPR-020 | Evaluate operation postconditions against exact pre/post anchors and result. | Required | V1-009 | C | C | C | P | M | QSL #1 / research #39 |
| V1-EXPR-021 | Evaluate immutable captures and aliases without anchor drift. | Required | V1-009 | C | P | P | P | M | QSL #1 / research #39 |
| V1-EXPR-022 | Enforce operation frames, created identities and deleted identities. | Required | V1-009 | C | P | P | P | M | QSL #1 / research #39 |
| V1-EXPR-023 | Report parse, type, admission, lowering, execution and observation failures as distinct stages. | Required | V1-005 | C | P | P | P | M | QSpec #1 / research #39 |
| V1-EXPR-024 | Reject reflection, ambient extension lookup and unreviewed user callbacks. | Integrity | V1-008 | C | C | P | P | M | research #65 / QSL #1 |
| V1-EXPR-025 | Reject mutation and I/O during declarative clause evaluation. | Integrity | V1-008, V1-039 | C | C | P | P | M | QSpec #1 / research #39 |
| V1-EXPR-026 | Report resource exhaustion and undefined arithmetic without Boolean approximation. | Integrity | V1-005, V1-010 | C | C | C | P | M | QSL #1 / Contract IR #1 |

## 4. Temporal, clocks and observation

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-TEMP-001 | Evaluate bounded `eventually`. | Required | V1-020 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-002 | Evaluate bounded `always`. | Required | V1-020 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-003 | Evaluate bounded `until`. | Required | V1-020 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-004 | Evaluate bounded `release`. | Required | V1-020 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-005 | Evaluate bounded previous, historically, once, since and triggered operators. | Required | V1-021 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-006 | Bind an explicit past-history origin and cutoff. | Required | V1-021, V1-025 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-007 | Evaluate over event-position clocks. | Required | V1-022 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-008 | Evaluate over fixed-period sampled clocks with epoch, period and unit. | Required | V1-022 | C | P | P | P | M | research #37 / TL owners |
| V1-TEMP-009 | Evaluate exact timestamped finite-window deadlines. | Required | V1-023 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-010 | Capture trigger values at one immutable activation instant. | Required | V1-023 | C | P | P | P | M | research #37 / QSL #1 |
| V1-TEMP-011 | Advance timeout progress during event silence from authoritative clock progress. | Required | V1-023 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-012 | Distinguish closed execution, valid open prefix and incomplete input. | Required | V1-024 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-013 | Report activation independently from verdict and settlement. | Required | V1-024, V1-041 | C | P | P | P | M | research #37 / research #38 |
| V1-TEMP-014 | Apply an explicit late-data cutoff and retain supersession lineage. | Required | V1-025 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-015 | Bind an authoritative completeness claim to its exact population and window. | Required | V1-025, V1-050 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-016 | Represent bounded clock uncertainty intervals. | Required | V1-026 | C | M | M | M | M | research #37 / QSpec #1 |
| V1-TEMP-017 | Repair out-of-order observations through explicit revisioned replay. | Required | V1-026 | C | P | P | P | M | research #37 / QObs #1 |
| V1-TEMP-018 | Evaluate partial proposition valuations without coercing missing values to false. | Required | V1-026 | C | M | M | M | M | research #37 / TL owners |
| V1-TEMP-019 | Lower native temporal syntax through a versioned TL correspondence. | Required | V1-028 | C | C | P | P | M | Contract IR #52 / research #39 |
| V1-TEMP-020 | Evaluate temporal atoms through native typed predicate semantics. | Required | V1-028 | C | C | P | P | M | Contract IR #52 / research #39 |
| V1-TEMP-021 | Refuse an infinite-trace or unbounded-liveness proof claim from finite evidence. | Integrity | V1-027 | C | P | P | P | M | research #37 / Verification #1 |
| V1-TEMP-022 | Refuse clock-unit, boundary and false-extension correspondence mismatches. | Integrity | V1-022, V1-028 | C | C | P | P | M | Contract IR #52 / research #37 |
| V1-TEMP-023 | Author and soundly analyze unbounded temporal properties under an explicit infinite-trace semantic profile. | Required | V1-027; prose census | C | M | M | M | M | TL owners / research #42 |
| V1-TEMP-024 | Admit the unbounded (no interval bound) temporal operator grammar edition only under the selected infinite-trace profile. | Required | V1-027; prose census | C | M | M | M | M | QSL #188 / TL owners |
| V1-TEMP-025 | Map infinite-trace proved/refuted/inconclusive/unsupported/failed results onto the existing execution, truth and settlement-basis vocabulary. | Required | V1-027; prose census | C | M | M | M | M | QSL #188 / TL owners |
| V1-TEMP-026 | Settle an infinite-trace claim with no registered liveness backend as unsupported with a warning naming the missing capability. | Integrity | V1-027; prose census | C | M | M | M | M | QSL #188 / TL owners |

## 5. Protocol and choreography

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-PROT-001 | Declare finite participant roles. | Required | V1-029 | C | C | P | P | M | research #38 / Protocol #14 |
| V1-PROT-002 | Identify runtime role instances independently of role definitions. | Required | V1-029 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-003 | Bind roles to model components and endpoints explicitly. | Required | V1-029 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-004 | Declare typed messages and payloads. | Required | V1-030 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-005 | Declare channels independently of participants. | Required | V1-030 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-006 | Distinguish send, receipt, message, delivery and attempt identities. | Required | V1-030 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-007 | Bind business correlation and related workflow identities explicitly. | Required | V1-030, V1-049 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-008 | Compose authored finite sequence. | Required | V1-031 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-009 | Compose owned labeled choice. | Required | V1-031 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-010 | Compose parallel branches and explicit join. | Required | V1-031 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-011 | Compose bounded iteration. | Required | V1-031 | C | C | P | P | M | research #38 / QSL #1 |
| V1-PROT-012 | Spawn roles dynamically under explicit finite population and lifetime bounds. | Required | V1-038 | C | M | M | M | M | research #67 / Protocol #14 |
| V1-PROT-013 | Model queues with explicit finite or symbolic resource bounds. | Required | V1-038 | C | P | P | P | M | research #67 / Protocol #14 |
| V1-PROT-014 | Model loop invariants and termination/resource obligations beyond literal bounded repeat. | Required | V1-038 | C | M | M | M | M | research #67 / Protocol #14 |
| V1-PROT-015 | Preserve authored causal edges independently of timestamp order. | Required | V1-032 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-016 | Enforce choice visibility for each deciding and affected role. | Required | V1-032 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-017 | Declare delivery cardinality, ordering and duplication premises. | Required | V1-032 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-018 | Attach per-instance state guards and temporal obligations. | Required | V1-033 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-019 | Attach immutable captures and inclusive timeout branches. | Required | V1-033 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-020 | Register compensation only after its matching successful effect. | Required | V1-034 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-021 | Distinguish effect, action, attempt, retry, commit and recovery identities. | Required | V1-034 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-022 | Enforce commit and recovery relationships. | Required | V1-034 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-023 | Distinguish failed, timed-out, retried, untriggered and partially recovered outcomes. | Required | V1-035 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-024 | Detect duplicate business effects independently of duplicate delivery. | Required | V1-035 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-025 | Replay a supplied finite trace reproducibly. | Required | V1-036, V1-047 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-026 | Assess global conformance to a selected protocol. | Required | V1-036 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-PROT-027 | Project a global protocol to sound local role obligations. | Required | V1-037 | C | M | M | M | M | research #38 / Protocol #14 |
| V1-PROT-028 | Analyze protocol realizability with explicit premises and counterexamples. | Required | V1-037 | C | M | M | M | M | research #38 / Protocol #14 |
| V1-PROT-029 | Analyze assume-guarantee composition. | Required | V1-037 | C | M | M | M | M | research #38 / Verification #1 |
| V1-PROT-030 | Check implementation refinement against a selected protocol. | Required | V1-037 | C | M | M | M | M | research #38 / Verification #1 |
| V1-PROT-031 | Observe or recommend workflow actions without executing external business side effects. | Integrity | V1-039 | C | P | P | P | M | research #38 / Protocol #14 |

## 6. Runtime, analysis and workflows

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-RUN-001 | Execute admitted state clauses in the native reference runtime. | Required | V1-047 | C | C | C | P | M | QSL #1 / research #39 |
| V1-RUN-002 | Execute admitted temporal obligations in replay mode. | Required | V1-047 | C | P | P | P | M | research #37 / research #39 |
| V1-RUN-003 | Execute admitted protocol obligations in replay mode. | Required | V1-047 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-RUN-004 | Incrementally monitor temporal and protocol obligations. | Required | V1-047 | C | P | P | P | M | research #37 / Protocol #14 |
| V1-RUN-005 | Retain independent execution, truth, activation and settlement dimensions. | Required | V1-041 | C | P | P | P | M | research #38 / Verification #1 |
| V1-RUN-006 | Retain independent participation, assumptions, completeness and provenance dimensions. | Required | V1-041 | C | P | P | P | M | research #38 / Verification #1 |
| V1-RUN-007 | Assess finite declared snapshots and event-time populations. | Required | V1-050 | C | P | P | P | M | research #37 / QObs #1 |
| V1-RUN-008 | Evaluate aggregates over reservation, refund and related workflow populations. | Required | V1-049, V1-050 | C | P | P | P | M | research #37 / Protocol #14 |
| V1-RUN-009 | Relate order, shipment, payment-attempt and refund workflow instances. | Required | V1-049 | C | P | P | P | M | research #38 / Protocol #14 |
| V1-RUN-010 | Run the state workflow reference example end to end. | Required | V1-042 | C | C | C | P | M | research #39 / QSL #1 |
| V1-RUN-011 | Run the timed-refund reference example end to end. | Required | V1-042 | C | P | P | P | M | research #39 / Protocol #14 |
| V1-RUN-012 | Run branching compensated choreography with fan-out and join end to end. | Required | V1-042 | C | P | P | P | M | research #39 / Protocol #14 |
| V1-RUN-013 | Simulate finite nondeterministic model behaviors without external side effects. | Required | prose census | C | M | M | M | M | research #41 / Verification #1 |
| V1-RUN-014 | Generate model-based test traces and shrink reproducible counterexamples. | Required | prose census | C | P | P | M | M | research #41 / Verification #1 |
| V1-RUN-015 | Analyze relational properties across two or more executions. | Required | V1-045 | C | M | M | M | M | research #42 / Verification #1 |
| V1-RUN-016 | Analyze hyperproperties over explicit finite trace sets. | Required | V1-045 | C | M | M | M | M | research #42 / Verification #1 |
| V1-RUN-017 | Author a verification-plan DSL that selects methods, bounds and evidence obligations without duplicating clauses. | Required | V1-045 | C | M | M | M | M | research #42 / Verification #1 |
| V1-RUN-018 | Analyze continuous and hybrid dynamics under an explicit numeric/time model. | Required | prose census | C | M | M | M | M | research #43 / Verification #1 |
| V1-RUN-019 | Perform bounded synthesis while distinguishing candidates from proof. | Required | prose census | C | M | M | M | M | research #43 / Verification #1 |
| V1-RUN-020 | Drive an external workflow through typed decisions while keeping effects in an authorized adapter. | Required | prose census; V1-039 boundary | C | M | M | M | M | research #43 / Protocol #14 |

## 7. IR, proof backends and output mappings

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-BACK-001 | Request backend capabilities independently for each clause or claim. | Required | V1-006 | C | P | P | P | M | Contract IR #1 / research #39 |
| V1-BACK-002 | Preserve supported sibling results when another requested capability refuses or exhausts. | Required | V1-006 | C | P | P | P | M | Contract IR #1 / Verification #1 |
| V1-BACK-003 | Lower typed clauses to versioned IR without creating a second authored language. | Required | V1-006, V1-040 | C | C | C | P | M | Contract IR #1 / QSL #1 |
| V1-BACK-004 | Execute Boolean, integer, rational, collection, graph, temporal and protocol IR with typed non-Boolean outcomes. | Required | V1-006 | C | P | P | P | M | Contract IR #1 / codegen #1 |
| V1-BACK-005 | Generate bounded Kani contracts and harnesses from model domains. | Required | V1-006, V1-048 | C | P | P | P | M | Contract IR #1 / codegen #1 |
| V1-BACK-006 | Replay every backend counterexample through the native runtime with the same verdict. | Required | V1-006, V1-047 | C | P | P | P | M | Contract IR #1 / QSL #1 |
| V1-BACK-007 | Emit OCL 2.4 with source-bound preserved, conditional, unrepresented and refused accounting. | Required | V1-043 | C | P | P | M | M | Contract IR #55 / research #40 |
| V1-BACK-008 | Emit SysML 2.0 and KerML 1.0 with source-bound loss accounting. | Required | V1-043 | C | P | P | M | M | Contract IR #56 / research #40 |
| V1-BACK-009 | Emit FRETish 3.1 with source-bound loss accounting. | Required | V1-043 | C | P | P | M | M | Contract IR #57 / research #40 |
| V1-BACK-010 | Produce deterministic immutable output packages. | Required | V1-043, V1-046 | C | P | P | M | M | Contract IR #58 / research #40 |
| V1-BACK-011 | Account for every source obligation in every requested output mapping. | Required | V1-043 | C | P | P | M | M | Contract IR #58 / research #40 |
| V1-BACK-012 | Keep external source-language imports out of the native authority chain. | Integrity | V1-044 | C | P | P | P | M | QSpec #1 / research #40 |
| V1-BACK-013 | Keep Java, Node and Electron runtimes out of production and qualification semantics. | Integrity | V1-044 | C | P | P | P | M | QSpec #1 / research #40 |
| V1-BACK-014 | Refuse a backend request it cannot implement exactly; never approximate. | Integrity | V1-005, V1-006 | C | C | C | P | M | Contract IR #1 / codegen #1 |
| V1-BACK-015 | Mint native-run-result/2 as a new closed wire carrying the separating-witness record, selected explicitly and never inferred from /1 content. | Required | V1-006, V1-047 | C | M | M | M | M | QSL #186 / Codegen #50 |
| V1-BACK-016 | Refuse silent substitution between native-run-result/1 and /2; a consumer requiring the witness record regenerates against /2. | Integrity | V1-006, V1-047 | C | M | M | M | M | QSL #186 / Codegen #50 |
| V1-BACK-017 | Check that a decomposed requirement's admitted result entails its parent's admitted result through a registered implication backend; absent that backend, the claim settles `unsupported` with a warning naming the required capability. | Required | V1-037; prose census | C | M | M | M | M | quire-analyze #40 / QSL #185 |
| V1-BACK-018 | Catalog exact lowering targets and the FR-290 capabilities each backend advertises for registration and negotiation. | Required | V1-006 | C | M | M | M | M | QSL #185 |

## 8. Evidence, qualification, tooling and ecosystem

| ID | Atomic capability | Class | Origin | Spec | Impl | Ver | Int | Qual | Existing program owner |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V1-EVID-001 | Bind requirements, assumptions and verification links to exact semantic subjects. | Required | V1-041, V1-048; prose census | C | P | P | P | M | Verification #1 / Quoin #519 |
| V1-EVID-002 | Preserve source, model, binding, implementation, method and tool provenance. | Required | V1-041, V1-048 | C | P | P | P | M | Verification #1 / Quoin #519 |
| V1-EVID-003 | Publish conformance vectors for every admitted semantic feature and integrity refusal. | Required | V1-048 | C | P | P | P | M | research #40 / QSpec #1 |
| V1-EVID-004 | Qualify language conformance independently from monitor correctness and system qualification. | Required | V1-048 | C | P | P | P | M | research #40 / Verification #1 |
| V1-EVID-005 | Reproduce results from exact source, dependency, bound, tool and option pins. | Required | V1-046, V1-048 | C | P | P | P | M | research #40 / Verification #1 |
| V1-EVID-006 | Preserve typed counterexamples as replayable evidence. | Required | V1-047, V1-048 | C | P | P | P | M | Verification #1 / Contract IR #1 |
| V1-EVID-007 | Retain the separating-witness record's exact deciding element, index, value path and trace position for every decisive settlement. | Required | V1-047, V1-048 | C | M | M | M | M | QSL #186 / Verification #38 |
| V1-EVID-008 | Retain one family-neutral separating witness or counterexample shape across collection, temporal and protocol settlement. | Required | V1-047, V1-048 | C | M | M | M | M | QSL #186 / Verification #38 |
| V1-EVID-009 | Leave an exhausted, cancelled or unsupported result witness-free rather than substituting a partial or synthesized witness. | Integrity | V1-047, V1-048 | C | M | M | M | M | QSL #186 / Verification #38 |
| V1-TOOL-001 | Expose parse, check, lower, execute, replay, monitor and inspect through stable Rust APIs. | Required | code-surface census | C | P | P | P | M | QSL #1 / research #39 |
| V1-TOOL-002 | Expose the same lifecycle through a stable CLI. | Required | code-surface census | C | P | P | P | M | quire-cli / research #39 |
| V1-TOOL-003 | Preserve a lossless concrete syntax tree for editors and source transformations. | Required | prose census | C | M | M | M | M | QSL #1 / research #39 |
| V1-TOOL-004 | Provide formatter, language-server and editor diagnostics over the lossless source model. | Required | prose census | C | M | M | M | M | QSL #1 / research #39 |
| V1-TOOL-005 | Provide model metadata, semantic views and deterministic rendering APIs. | Required | prose census | C | M | M | M | M | Research #77 / research #39 |
| V1-TOOL-006 | Load versioned Rust plugins only through explicit capability, purity and provenance contracts. | Required | prose census | C | M | M | M | M | research #43 / QSpec #1 |
| V1-TOOL-007 | Support ahead-of-time and JIT execution without changing language meaning or qualification identity. | Required | prose census | C | M | M | M | M | research #43 / Verification #1 |
| V1-TOOL-008 | Package reusable domain libraries with exact edition/profile/dependency compatibility. | Required | prose census | C | P | P | M | M | research #43 / QSpec #1 |
| V1-TOOL-009 | Expose a WASM consumer with the same admitted semantics and refusal behavior. | Required | code-surface census | C | P | P | M | M | quire-wasm / research #39 |
| V1-TOOL-010 | Expose code-analysis and source-generation consumers without granting them semantic authority. | Required | code-surface census | C | P | P | M | M | quire-analyze, quire-code-rs / research #39 |
| V1-TOOL-011 | Confirm a superseding specification edition admits every result its prior edition admitted. | Required | V1-037; prose census | C | M | M | M | M | QSL #191 |
| V1-TOOL-012 | Preserve every parent profile's refusal as its child profile's refusal under profile layering. | Required | V1-037; prose census | C | M | M | M | M | QSL #192 |

## Source census

The inventory was formed from four independent source classes. A source being
listed here means it has been accounted for, not that its prior scope decision
remains authoritative.

| Source class | Enumerated input | Atomic accounting |
| --- | --- | --- |
| Historical capability groups | Every `V1-001` through `V1-051` row in `proposals/quire-v1/capabilities.md` | Every group identifier appears in one or more `Origin` cells above; compound groups were split. |
| Current compiler refusals | `helper`, `rec`, `set`, `bag`, `Set`, `Bag`, `OrderedSet`, `collect`, `flatten`, `cast`, `tuple`, `Tuple`, `Decimal`, `Rational`, `allInstances` from QSL `0c2f13e:src/token.rs` | Each spelling maps to V1-EXPR-003/004/013/014, V1-TYPE-004/005/012/015/016/017/023/030. Case variants share semantics but remain enumerated here. |
| Prose-only exclusions | richer numerics and units; recursive values; inherited/redefined models; uncertainty/repair; projection/realizability/refinement; dynamic protocols; relational and hyperproperty analysis; simulation, synthesis and workflow driving; editor/CST/tool APIs; plugins/JIT/libraries | Each named item has a Required row above and therefore cannot disappear behind prose. |
| Existing artifacts and delivery surfaces | Current QSpec artifacts, Rust repositories, default-branch public APIs and live/closed GitHub evidence | Counted below and mapped in Task-003; current partial code never defines product scope. |

### Specification artifact census

At the Plan-008 baseline the repository contains 2 StR, 8 US, 73 functional FR,
13 NFR, 11 IT, 90 object-layer FR, 76 TC, 7 Test Matrices and 4
ArchitectureDescriptions. Task-003 maps every one of those artifacts to the
atomic rows and classifies it as reusable, repairable, splittable, superseded or
missing coverage. These counts are evidence baselines, not coverage claims.

### Rust implementation surface census

The revisions below are the locally inspected `origin/main` references used for
the inventory. A newer merge must update both the revision and the affected
atomic rows; worktree-only changes do not count.

| Repository | Inspected revision | Public responsibility represented in the inventory |
| --- | --- | --- |
| `quire-spec-language` | `0c2f13e261c0` | Native source, linking/checking, state execution, temporal/protocol syntax and lowering. |
| `quire-contract-ir` | `5ea7730689cb` | Derived IR, backend requests, temporal joins, proof/output package contracts. |
| `quire-contract-codegen` | `73c82ade7b1e` | Oracle generation and bounded Kani harness generation. |
| `quire-contract-runtime` | `7caefac3b718` | Contract execution and runtime integration surface. |
| `quire-protocol` | `12502def62a1` | Protocol linking, replay, assessment and result composition. |
| `quire-observation` | `93bc565072fb` | Observation identity, topology, windows, completeness and progress. |
| `tl-syntax` | `db11e196b8b3` | Versioned temporal formula representation. |
| `tl-parse` | `2bc030dae8fd` | Temporal parsing. |
| `tl-mltl` | `22862189ac4e` | Finite future/past temporal evaluation. |
| `tl-rewrite` | `c416951281c3` | Semantics-preserving temporal rewrites. |
| `filament-core-data` | `eb0d5819e1ca` | Semantic IR schema and the extraction-frontend lift of spec artifacts into domain packages. |
| `quire-cli` | `092ff2245b40` | User-facing command lifecycle. |
| `quire-analyze` | `5f37678928ae` | Static/source analysis consumer. |
| `quire-code-rs` | `e1b7fc303f3d` | Rust code-generation consumer. |
| `quire-wasm` | `28227bee1b99` | WASM consumer. |
| `quoin` | `2e7b3301394c` | Assurance method/plan/evidence integration; Rust port prerequisite remains explicit. |

## Machine-auditable invariants

The table contract supports deterministic checks without interpreting prose:

1. every inventory row begins with one unique `V1-` ID;
2. every row has class `Required` or `Integrity`;
3. every delivery cell is one of `C`, `P`, `M`, or `N/A`;
4. every row names an existing program owner;
5. all 51 historical group IDs and all 15 compiler refusal spellings occur in
   the source census or an `Origin` cell;
6. the strings `Later`, `Deferred`, `Future-only`, `Research-only`, `Unowned`
   and `Unticketed` are invalid as row dispositions;
7. Task-003 may make a status more precise only with cited artifact or code
   evidence; Task-005 and Task-009 replace broad program owners with exact
   reconciled epics and implementation tickets.

No stage is complete merely because this inventory exists. Its purpose is to
make omitted work and over-broad completion claims mechanically visible before
the architecture and specification are expanded.
