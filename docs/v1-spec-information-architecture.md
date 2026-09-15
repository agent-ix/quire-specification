# Complete V1 specification information architecture

This document is the Task-004 decision record for how the complete V1
specification corpus will be organized before its major expansion. It is based
on the default-branch revisions recorded in
`docs/v1-capability-inventory.md`, the 175-row atomic inventory, the current
artifact bodies mapped in `docs/v1-existing-spec-coverage.md`, and each
repository's existing root specification and architecture decisions.

It does not define the product architecture. Task-006 defines that architecture
once, across the complete scope. This document defines where that architecture,
its requirements, its objects, and its verification ownership will live so the
Task-007 `/specify` run does not enlarge an accidental flat structure or create
competing sources of truth.

## Decision rules

The campaign applies these rules to every owner repository:

1. Artifact identifiers remain immutable and globally unique within their
   repository. A file relocation never renumbers or reissues an identifier.
2. Existing Git history is preserved with repository-native moves. Historical
   reviews remain at their current paths and keep reviewing the revision they
   name.
3. A requirement has one normative owner. Subsystem indexes contain links and
   summaries, never copied normative clauses.
4. A shared domain object is defined once in `quire-specification`; an
   implementation repository references it and specifies only its local
   admission, representation, execution, or publication behavior.
5. A requirement maps to one primary TestMatrix. Integration seams can cite the
   same requirement, but cannot silently create a second coverage authority.
6. The master specification is an index and boundary document. It does not grow
   by embedding another exhaustive capability ledger.
7. Structural migrations land before new requirements in the affected
   repository. A migration changes no acceptance criterion or product meaning.
8. One repository has one writer during its structural migration. Downstream
   specification work rebases after the migration lands.

## Measured pressure and disposition

The following measurements are from the inspected revisions in the inventory.
Counts describe current files, not complete-V1 coverage.

| Repository | Current evidence | Decision before expansion |
| --- | --- | --- |
| `quire-specification` | 73 functional requirements and 90 object requirements are already grouped under five owner-aligned directories; 13 NFR, 11 IT and 76 TC artifacts remain flat; seven family matrices exist; the root `spec/spec.md` contains a long hand-maintained relationship list. | **Reorganize navigation and allocate new families; preserve current canonical paths.** Add subsystem indexes, new family directories, and matrix ownership before authoring the missing scope. Do not move the already grouped foundation/protocol/temporal/observation/output artifacts merely to rename the taxonomy. |
| `quire-spec-language` | 52 functional requirements and 141 test cases are flat. Specialized matrix/review areas already exist for model linking, packages, runtime, temporal, workflow, readiness, and lowering. The root specification mixes the historical compiler, composed admission, executable profiles, and newer package/runtime work. | **Reorganize physically before expansion.** Move current FR/TC files into source, model, expression, package, runtime, temporal, protocol, lowering, and integration families while preserving IDs and Git history. Consolidate the specialized matrix areas behind one subsystem index without merging their normative contents. |
| `quire-protocol` | 42 functional requirements, 8 NFR, 3 IT and 17 TC are flat. `ADR-001` explicitly requires the layout decision to be revisited after roughly 25 requirements; the repository has passed that trigger. Its complete-V1 scope adds dynamic topology, queues, looping, projection, realizability, refinement, conformance and workflow seams. | **Supersede ADR-001 and reorganize physically before expansion.** Group existing and new artifacts by the protocol subsystems below; retain one local TM authority and keep upstream object semantics in QSpec. |
| `quire-contract-ir` | The root index already separates `contract`, `interface`, `program`, `decisions`, `assurance`, `evidence`, and two matrices. It explicitly owns the target-neutral semantic substrate rather than execution. | **Preserve.** Extend its existing contract/interface families. Add a subsystem index only if Task-006 assigns more than one new contract family; do not create a parallel IR schema owner. |
| `quire-contract-codegen` | Its root already declares subsystem layout and has a populated `strategies` family plus explicit future moves for oracle, Kani, coverage, CLI and assurance artifacts after in-flight work. | **Preserve and finish its accepted local migration.** The complete-V1 campaign consumes that layout and adds backend-specific families under it; it does not redesign the codegen tree from QSpec. |
| `quire-contract-runtime` | Five FR, two NFR, one interface, fourteen TC and a single matrix form a small coherent runtime-helper boundary. | **Preserve.** New executable-semantics work is added under explicit value, state, evaluation and adapter families only when Task-006 allocation requires it; the runtime must not absorb compiler or workflow-orchestration authority. |
| `quire-observation` | Six FR, two NFR, one IT and six TC form a small, focused observation boundary. | **Preserve.** Add uncertainty, partial valuation, repair and out-of-order semantics as one coherent expansion ticket; introduce family directories in that ticket only if its approved requirements cross three distinct local subsystems. |
| `quire-verification` | 27 functional, 8 NFR, 4 IT and 45 TC artifacts already serve a broad portfolio; its working repository is concurrently owned by another agent. | **Plan a navigation reorganization, do not touch it from this branch.** The owning ticket adds subsystem indexes for methods, plans, results/counterexamples and qualification before relational, hyperproperty, model-testing and synthesis expansion. Existing paths remain canonical unless that owner chooses one atomic move. |
| `quire-cli` | 26 functional and 10 NFR artifacts are flat, with no separate TC directory in the inspected tree. Complete V1 adds lifecycle commands, diagnostics and workflow-facing execution. | **Reorganize before command expansion.** Establish command-lifecycle, diagnostics, execution, generation and integration families plus a real TestMatrix; keep the CLI a coordinator over library APIs. |
| `quire-analyze` | Six FR, two NFR, one interface and twelve TC already use the common small-library layout. | **Preserve.** Add static, relational, hyperproperty and model-analysis families when their architecture is assigned; retain a single analysis-result interface. |
| `quire-code-rs` | Thirteen FR, six NFR and two IT form a bounded Rust generation consumer. | **Preserve.** Add only typed model/contract generation families required by Task-006; do not duplicate general Contract Codegen proof-backend behavior. |
| `quire-wasm` | Five FR, two NFR and two stakeholder requirements form a minimal consumer surface. | **Preserve.** Add a bounded browser/tooling adapter specification after the native APIs are fixed; no independent semantics. |
| `tl-syntax`, `tl-parse`, `tl-mltl`, `tl-rewrite` | Each uses the established `requirements`, plans, reviews and assurance convention; concurrent TL work owns these repositories. | **Preserve.** Complete V1 consumes their versioned temporal APIs and assigns missing temporal work to their current owner. This campaign does not reorganize or edit active TL trees. |
| `filament-core-data` | The inspected tree already has a large, separately governed semantic-model and producer corpus. | **Preserve and do not edit.** QSpec/QSL specify the exact consumer correspondence and versioned handoff; Filament remains the authoritative model producer. |
| `quoin` | A large assurance corpus already exists, but Quire integration is gated on the announced Rust port. | **Preserve and do not edit before the Rust port lands.** Task-006 defines the required consumer contract; its implementation/specification ticket activates only against the Rust owner surface. |

## QSpec target topology

QSpec remains the normative language and shared-object owner. Its existing five
families remain canonical. The complete architecture is presented through the
following navigation views, aligned to the inventory without prejudging the
Task-006 interface design:

| View | Atomic inventory scope | Current canonical families | New growth location |
| --- | --- | --- | --- |
| Source, package and identity | `V1-SRC-*` | `functional/foundation`, `objects/foundation` | `functional/source-package`, `objects/source-package` |
| Type and model system | `V1-TYPE-*` | `functional/foundation`, `objects/foundation` | `functional/type-model`, `objects/type-model` |
| Expressions, queries and contracts | `V1-EXPR-*` | `functional/foundation`, `objects/foundation` | `functional/expressions`, `objects/expressions` |
| Temporal, clocks and observation | `V1-TEMP-*`, `V1-OBS-*` | `functional/temporal`, `functional/observation`, matching object families | Existing temporal/observation families |
| Protocol and choreography | `V1-PROTO-*` | `functional/protocol`, `objects/protocol` | Existing protocol family |
| Runtime, analysis and workflows | `V1-RUN-*`, `V1-AN-*`, `V1-WF-*` | portions of foundation/protocol | `functional/runtime`, `functional/analysis`, `functional/workflow` and matching object families |
| IR, proof backends and output mappings | `V1-IR-*`, `V1-PROOF-*`, `V1-OUT-*` | `functional/output-mapping`, `objects/output-mapping` | `functional/ir-proof` plus existing output-mapping family |
| Evidence, qualification, tooling and ecosystem | `V1-EVID-*`, `V1-QUAL-*`, `V1-TOOL-*`, `V1-ECO-*` | foundation and cross-cutting NFR/IT | `functional/tooling`, `functional/ecosystem`; NFR/IT stay in their artifact-class roots |

`spec/subsystems/index.md` will be the human navigation entry point and will
link each view to its canonical FR, NFR, IT, object, TC, matrix and architecture
artifacts. It does not own requirements. `spec/spec.md` will link that index and
retain the master boundary, identification policy and direct machine-readable
relationships required by Quire.

Current files are not moved because the existing owner-aligned structure is
already coherent and their stable links are widely consumed. New requirements
enter the named growth locations. Existing requirements whose complete-V1
meaning expands are amended in place. A requirement crossing views is assigned
to the view that owns its state transition or invariant and references its
inputs; it is never copied.

### QSpec object and matrix ownership

- QSpec owns shared semantic nouns and identity rules. QSL, Protocol,
  Observation, IR and runtime repositories own representations and operations,
  not competing object definitions.
- The 13 current object files with explicit open gaps are resolved before a new
  dependent object is authored. Task-006 decides any common base objects so
  Task-007 cannot independently invent them in different families.
- The seven existing family matrices remain primary for their current rows.
  New primary matrices are added for source/package, type/model,
  expressions/collections, runtime/workflow, analysis, IR/proof and
  tooling/ecosystem only where no existing matrix owns that inventory row.
- `docs/v1-capability-inventory.md` remains the scope ledger; matrices establish
  verification coverage and never become a second product-scope list.

## QSL target topology

QSL owns native syntax, static semantics, compilation, executable reference
semantics and versioned producer artifacts. Before complete-V1 requirements are
added, one QSL structural ticket will:

1. add `spec/subsystems/index.md` with source, model/type,
   expression/collection, package/linking, runtime/evaluation,
   temporal/observation, protocol/choreography, lowering/backend and
   diagnostics/tooling views;
2. move existing `spec/functional/FR-*` and `spec/test-cases/TC-*` into those
   family directories using Git-native moves;
3. update all repository-local links and the master relationship index in the
   same commit;
4. leave historical review paths unchanged and add a relocation map from every
   old path to its new canonical path;
5. retain the existing specialized matrix areas, link them from the subsystem
   index, and designate one primary matrix for each requirement; and
6. validate that every pre-migration ID and every relationship target still
   resolves before semantic authoring begins.

The migration does not merge source parsing with model linking, compilation
with execution, or target lowering with native semantics. Those are separate
views even when one public crate currently implements them.

## Protocol target topology

Protocol's accepted `ADR-001` selected a flat layout when the repository held
nine functional requirements and required reconsideration near 25. The current
42-FR corpus satisfies that reconsideration condition. The structural ticket
will supersede, not erase, ADR-001 and group current/new artifacts under:

1. artifact admission, identity and dependency locking;
2. roles, messages, channels, queues and communication state;
3. causal control, choice, loops and dynamic topology;
4. obligation activation and compensation/recovery;
5. replay, progress, closure and conformance;
6. projection, realizability, refinement and composition;
7. result, refusal, lineage and evidence handoff;
8. runtime/workflow adapters; and
9. qualification, resource and integration constraints.

FR, TC and any new object-local representation artifacts move together by
family. NFR and IT may remain in their artifact-class roots because they are
cross-cutting; the subsystem index links them to every governed family. TM-001
remains the single local coverage authority and is partitioned by sections, not
forked into competing matrices. All formal domain objects remain upstream QSpec
references.

## Cross-repository migration and writer order

Structural work is dependency work, not an invitation for parallel edits to the
same corpus. The campaign uses this order:

1. **QSpec information skeleton.** Add subsystem navigation, empty approved
   growth directories where needed, matrix ownership rules, and the object-gap
   repair order. This is the only QSpec structural writer.
2. **QSL structural migration.** Rebase on the accepted QSpec architecture
   names, move existing artifacts atomically, validate links, and land before
   new language requirements. No other QSL specification writer runs during
   the move.
3. **Protocol structural migration.** Coordinate with the current Protocol
   owner, supersede ADR-001, move the corpus atomically, and validate TM-001.
   No complete-V1 protocol requirements land until this migration lands.
4. **Other consumer indexes.** Verification and CLI reorganizations occur in
   their own owner tickets. Small libraries retain their current layouts and
   add family directories only with approved semantic work.
5. **Complete-V1 specification expansion.** Task-007 `/specify` authors and
   amends requirements only after Tasks 004-006 fix the information topology,
   dependency graph, terminology, interfaces, failure model and architecture.

Cross-repository work is parallel only after its normative QSpec inputs merge.
Every implementation ticket records the exact upstream revision it consumes.
An upstream semantic amendment lands before its producer requirement; producer
requirements land before consumer/adaptor requirements; tests and matrices land
with the requirement they verify.

## Structural acceptance gate

Before Task-007 can author the expanded specification corpus, Task-006 must use
this topology and answer all architectural decisions without adding another
scope bucket. Before semantic work begins in a reorganized repository, its
structural PR must prove:

- the same set of artifact IDs exists before and after the move;
- no acceptance criterion or normative sentence changed in the move;
- every local Markdown link and `ix://` target resolves at least as well as at
  baseline;
- each requirement has exactly one normative owner and one primary matrix;
- the master and subsystem indexes agree on canonical paths; and
- historical reviews, tags and released references remain intact.

This gate makes organization a bounded prerequisite. It cannot be used to defer
any of the 175 complete-V1 rows or to replace the comprehensive architecture
and specification work required by Tasks 006 and 007.
