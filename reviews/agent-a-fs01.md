---
id: SR-515
title: "FS01 review and dependency record"
type: SpecReview
analysis: base
scope: "FS01 native-source profile packet and dependency record"
review_set: subset
evaluated_revision: "66e032e"
review_date: "2026-09-07"
---

# Agent A: FS01 review and dependency record

## Summary

Historical review record for the initial FS01 native-source profile packet and
its dependency routing at `66e032e`. Later merged implementation does not alter
the open profile, ADR and standard-artifact decisions recorded here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The proposed profile and ADR direction required owner adoption before FS01 acceptance. | Decisions requested after review; Q11 |
| FND-002 | medium | Finite-reference contracts remained open prerequisites. | Q4; Q6 |

2026-09-07. Private review record. Thread
`01a07e35-9c55-79a1-9524-6969be34378f`; branch
`agent-a/fs01-interface-packet`; worktree
`/home/peter/dev/worktrees/formalization-a-spec`.

Active owning task: [FS01](https://github.com/agent-ix/quire-specification/issues/2),
under specification#1 on private board20. The issue claim records the scope.
FS02/FS03/FS05 contributions below are proposed boundary inputs, not acceptance
or dependency closure. The original language/core assignment remains active.

## Current evidence

| Repository | Inspected revision and current finding |
|---|---|
| quire-research | `15e317a9a6dd4156d53808f82eaa65b7e4c473de`, also remote main; maintained docs and JSON task/issue maps inspected; no shared planning edits |
| quire-specification | `66e032e`, private shell; no preexisting implementation or open PR at claim |
| quire-spec-language | `509197f`, private shell; AGPL direction but exact SPDX not selected; no open PR at claim |
| quire-contract-ir | `decc99a430a0894de489102dbab04e83d1fb804f`, also remote main; FR-023 and `src/binding.rs` implement `BoundPackage::from_json_bytes`; #50 is still open |
| quire-rs | `8b8020e665c61a11bc74f3f23b84617c80c0c442`, also remote main; #388 closed; `src/semantic/clauses.rs` preserves bodies, classifies namespaced tags unchecked, and reports fence line/column spans |

Live FS01 has parent specification#1 and no blocked-by edges; board20 and board21
are private. FS02, FS03, FS05, LC01–LC05, E01, E04 and CO01 were open with no
claim comments at initial inspection. Existing owners' statuses and dependencies
have not been rewritten. The updated Agent A charter was read after Agent C
joined: C owns consolidated CO01 and shared planning updates; A contributes
language findings and retains standard/compiler proposals. No additional agent
has been spawned or assigned work here.

C acknowledged the handoff and confirmed the LC04/LC05 split in principle in
[CO01](https://github.com/agent-ix/quire-research/issues/54#issuecomment-5577378715).
Exact file/ticket ownership and consumer expectations still precede integration edits.

Key source facts are inspection evidence, not newly run compiler tests:

- `quire-contract-ir/spec/contract/FR-023-executable-projection-binding.md` already
  selects the keyed binder. Its `quire.contract.executable-projection/v1` input,
  old canonical profiles and bound identity remain authoritative. It requires
  complete executable clause binding; unsupported clauses cannot be relabeled
  informational just to make the decoder pass.
- `quire-contract-ir/src/expression.rs` has no identity-bearing reference type;
  records are acyclic. Its Boolean/numeric/collection representation is broader
  than the inspected codegen/analyzer support. LC04 must qualify actual backends.
- `quire-contract-ir/spec/decisions/ADR-0053-formal-clause-source-profiles.md`
  remains **proposed**, owner `kreneskyp`. It recommends an OCL-based subset and
  total Boolean lowering; this native profile proposes a different language and
  guard-aware semantics. Only the owner can amend/accept the ADR.

## Decisions requested after review

**Owner disposition, 2026-09-07:** in the task conversation the owner approved
the first implementation profile and `AGPL-3.0-only` for new compiler code,
then delegated implementation order: “yes ok lets go AGPL I leave the
implementation order up to you!” A-D1 is approved as the LC01 implementation
target and A-D3's implementation license is settled. Standard document/artifact
terms remain deferred as explicitly explained before the response. Public
promotion stays on hold. This approval does not claim FS02/FS03/FS05 completion
or silently mutate the existing public ADR. Subsequent private issue records
carry this disposition for B/C.

| ID | Concrete proposal | Decision owner and gate |
|---|---|---|
| A-D1 | Review `proposals/state-core/profile.md` as the native first-profile direction, including explicit finite references, short-circuit implication, `if`/`let`, integer rules and deferred extensions. Experimental tag `ix:native`; defer public branding to release | Program/standard owner; FS01 acceptance, LC01 admitted grammar |
| A-D3 | Use `AGPL-3.0-only` for newly authored compiler/evaluator code; preserve imported grants. For new standard prose, grammar/schema/vector artifacts, use the W3C Software and Document License, 2023 version, subject to owner/legal notice review | Owner under E04; exact implementation terms before code; standard artifact selection before public release and reuse |
| A-D4 | Hold publication. Review the state-core file set as a future independently usable packet; require actual clean consumer, backend and rights evidence before a later explicit promotion | Owner under E01; no current public release authorized |
| A-D5 | Confirm source/package fixture representation with Agent B before shared schema freeze; method/plan/result envelopes remain B's | Agent B + shared-model owner; FS02/FS05 shared boundary acceptance |

These proposals do not silently resolve themselves by being committed. The
existing LICENSE-DECISION files remain unchanged pending the applicable decision.
Contributor ownership and patent/trademark policy remain release questions;
approval of the technical profile does not decide them.

## Language-side CO01 amendment map

| Existing owner/ticket | Scoped amendment proposal | Acceptance dependency and disposition |
|---|---|---|
| IR #52/#53, ADR-0053 | Add a native-language direction and separate external-language mappings; retain old OCL proposal as proposed history. Replace neither language tags nor existing source identities automatically | A-D1 and owner ADR review; prepared, not publicly posted |
| IR #50 / FR-023 | Acknowledge existing binder implementation; adapt native lowering to the existing public API and complete-clause rule. Keep old digest/profile bytes | LC02 + reviewed typed-model adapter; no duplicate binder |
| IR #55/#56/#57 | Keep OCL/SysML/FRET as separately qualified mapping/frontends with loss/refusal and pinned tool evidence | FS06 and existing owners; no TL edits here |
| IR #58 | Preserve all source obligations and source implication activation identity; agree language facts with B's method/result contract | Agent B contribution + reviewed source-to-probe correspondence |
| Quire #388 / `clauses.rs` | Extraction exists. A proposes compiler source-map input; C coordinates consumer/extraction adapter regression fixtures | Agree LC05 split with C before edits; no expression compiler in Markdown extractor |
| TL #24 | Preserve pinned capability/profile/API contract; request summary only if state-to-temporal integration is selected | TL owner; temporal/clock work remains outside A |
| Codegen #2–#6 | A owns native lowering and exact projection delta; C coordinates existing backend adapter/consumer work and regression fixtures | Agree LC04 split with C and existing delivery owners before edits; no broad backend support claim |
| Quoin #272/#287 and verification | Supply source/model/package IDs and fixtures only; B owns catalog compatibility, plans/results and evidence integration | A-D5; no edits to verification or Quoin workspaces |

Public-capable amendment text consists only of the relevant semantic/API change,
its synthetic example, compatibility effects and qualified tests. This private
matrix, its issue links and roadmap are not part of a future public patch.

## Q1–Q12 and example review

| Original question | Proposed disposition / remaining owner |
|---|---|
| Q1 unbounded logic | Not a global gate; finite evaluation/search/validity/cost claims separate; FS01/FS03 |
| Q2 identity/name | Own `ix:native` experimental language; public name deferred to release; mappings FS06 |
| Q3 rejection depth | Lexical/syntax errors distinct from recognized unsupported source; retain clause/span; LC01/LC02 |
| Q4 object environment | Identity-bearing refs and explicit finite complete current/pre/post populations; FS02/FS03/LC03 |
| Q5 collections | Ordered duplicate-preserving sequence first; set/bag/collect explicitly refused pending multiplicity rules; FS03/LC04 |
| Q6 subtyping | Refused initially; extension requires substitutability/cast rules, no inherited foreign semantics; FS03 |
| Q7 fuel | Execution ceiling differs from semantic depth; visited identity closure; incomplete on exhaustion; LC03 |
| Q8 IR compatibility | Six independent edition/profile/wire/canonicalization/backend/release versions; preserve old bytes; FS05 |
| Q9 temporal | Preserve TL profiles; refund/clock/observation semantics owned elsewhere; FS04/TL |
| Q10 crates | One Rust package initially with model/syntax/parse/compiler(bind/lower)/eval modules and CLI; split only for independent consumers; LC01 |
| Q11 ADR | Scoped proposed amendment above; existing proposed ADR still requires owner disposition |
| Q12 boards | Private20 formalization, private21 research; public18/private17 existing delivery statuses preserved |

The state example expects parent order true/false, cycle detection, and separate
invalid/incomplete/refused inputs. Its finite maximum is authored synthetic
profile data, not a silent repair of the real ConfigVersion requirement. A timed
refund is retained but unsupported by the state profile; wrong charge, wrong
amount, silence, untriggered and missing observation are distinct future temporal
vectors. A parser portfolio joins exact source/model/tool/package identities;
missing harness or stale package cannot become a source-language pass. B owns
those method and result expectations.

## Implementation sequence and evidence still required

1. FS01 owner-reviewed admission and per-artifact license decision; then LC01
   native lexer/parser/formatter, source diagnostics, malformed/unsupported
   controls, reproducible workspace, lint/test/fixture CI. Proposed baseline is
   the installed Rust 1.94.1; pin the exact toolchain when implementing. Parser
   starts with std only; any new dependency gets a pinned rights inventory.
2. FS02/FS03/FS05 reviewed contracts; LC02 deterministic linking,
   source-revision bridge and strict linked packages.
3. LC03 independent finite-population evaluator, guards, collections, frames,
   identity closure and adversarial/resource cases with independent expectations.
4. LC04 reviewed projection changes and reference/generated parity on each
   admitted feature; old canonical and source activation regression vectors.
5. LC05 original source → model → linked package → actual generated execution
   with standalone clean consumer and Quire extraction integration.

No step is completed by this review record. No compiler, hosted CI, formal proof
or released profile has run as part of this packet.
The packet checks establish fixture integrity and precise expectations only.

## Packet validation actually run

- Six fixture file checksums matched; the source digest and expression byte span
  matched the original UTF-8 source, including line/scalar-column coordinates.
- Review JSON decoded with duplicate members rejected; candidate/review local
  Markdown links resolved; `git diff --check` passed.
- Candidate text was checked for private research URLs, local workspace paths
  and session IDs. The candidate has no link to its private review records.
  This does not replace a later packaged-output disclosure/rights audit.

Reproducible fixture checks are in the fixture README. No claim is made that
the source parsed, linked, evaluated or produced the expected diagnostic cases.

## Operation-source fixture follow-up

The next review pass found that the original concrete subject covered only a
current-state invariant. Added two operation subjects (`CanAttemptUpdate` pre
and `VersionUnchanged` post), exact whole-clause and expression digests/spans,
and seven invocation cases.
The existing current subject now also carries its clause-region digest/span.

The cases distinguish an allowed change that violates the postcondition from a
forbidden frame change, mismatched invocation, missing pre observation, deleted
post context and incorrect creation inventory. Three separately identified
synthetic frame bindings prevent a test caller from implicitly redefining one
operation's accepted frame. They remain proposed binding inputs, not linked
model authority or live runtime capability.

The expanded source fixture is versioned `agent-a-source-subject/1-draft` so
strict consumers cannot mistake its new members for the prior `0-draft` shape.
Source/profile editions are unchanged and old fixture bytes remain at their
original commit. Shared production wire/profile freeze still requires B/C review.

Checks run on this revision: nine fixture file checksums; three source subjects
and six region digest/span/coordinate checks; seven invocation references
against their three declared frame bindings; six local Markdown links; and
`git diff --check`. No parser/linker/evaluator was executed.

Live PR/FS01/FS02/E01/E04/CO01 inspection still showed no owner profile/license
decision or B boundary response when this follow-up began. A generic goal
continuation is not that decision. This pass made concrete fixture progress;
LC01–LC05 and the full assignment remain uncompleted and dependent gates stay
open. C's confirmed-in-principle ownership split is preserved.

### Architecture review / operation identity correction

Following the owner's request to review the compiler architecture, LC01 now uses
a declarative Logos lexer, JSON string decoding from serde_json, compact original
byte spans, indexed source coordinates, explicit grouping and a token-preserving
formatter. The compiler review records remaining linker/evidence boundaries.

C's concrete operation fixture identity findings are corrected: 7 case-scoped
invocations bind 13 exact snapshot files and 3 exact proposed frame files. Each
reference includes authority, identity, revision namespace/value, byte digest
and fixture profile; each payload exists once. SHA256SUMS covers the full set.
The producer audit in the language workspace verified 23 artifact files and six
negative controls for stale bytes and same-identity changed content. This is
observed fixture validation, not execution of the seven logical expectations.
Model/operation linking remains unavailable, and C's independent review is pending.
