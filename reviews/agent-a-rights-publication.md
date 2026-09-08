# Agent A: artifact rights and publication review

Private proposal, 2026-09-07. Contributes to E01/E04; does not close either.

## Artifact inventory and proposed terms

| Artifact | Source/inbound status | Proposed outbound disposition |
|---|---|---|
| `proposals/state-core/*.md` | Newly authored synthesis of inspected requirements; no copied standards prose or grammar | W3C Software and Document License, 2023 version, pending owner selection |
| `proposals/state-core/fixtures/*` | Newly authored synthetic source, model expectations, identities and population data; hashes/maps mechanically calculated from these bytes | Same proposed reusable terms; not conformance-certified or currently licensed for release |
| `reviews/agent-a-*.md` | Private coordination, research links and approval records | Private; outside proposed public artifact set |
| Future native compiler/evaluator/tests | No implementation added by this packet | Proposed `AGPL-3.0-only`, exact notice/owner decision required before source |
| Existing contract IR/runtime/codegen | Inspected first-party manifests/notices retain `MIT OR Apache-2.0`; no source copied | Preserve existing grants; later exact dependency closure audit |
| Existing Quire | Inspected `AGPL-3.0-or-later`; no implementation/template copied | Preserve existing grant; adapter combination reviewed at actual dependency pin |
| Filament declarations/kernel/generated files | Existing `AGPL-3.0-only`; interface shapes inspected, generated code and fixture contents not copied | No relicensing or vendoring; public schema/artifact release needs its owner's review |
| OCL/SysML/KerML/FRET tools/material | No parser, grammar, normative text or examples adopted | Separate version/license/mapping qualification before any later use |
| Validation environment | Installed Python 3, jsonschema 4.26.0, referencing 0.37.0 used only for local inspection where applicable | Not bundled or adopted as compiler runtime dependencies |

The proposed implementation identifier is the exact
[AGPL-3.0-only](https://spdx.org/licenses/AGPL-3.0-only.html) form; this differs
from `or-later`. The standard candidate is the
[W3C 2023 Software and Document License](https://www.w3.org/copyright/software-license-2023/),
which permits reuse and modifications subject to its notices. Neither candidate
has been applied automatically here. Exact copyright-holder notices, contributor
rights, patent/trademark policy and any combination questions remain owner/legal
review. This record makes no service-use restriction or commercial relicensing
claim. Root policy cannot withdraw existing per-file or previously released grants.

## E01 proposed publication checklist

Disposition now: **HOLD**. Prepare the candidate from an explicit included-file
inventory; never expose this repository's private history.

| Gate | Current evidence / required next evidence |
|---|---|
| Standalone semantics | Profile and source/identity/snapshot proposals supplied; normative review and full FS03/FS05 rules remain |
| Runtime independence | No private runtime dependency adopted; real compiler/package clean-consumer run still required |
| Public model boundary | Existing producer contract inspected; independently accessible licensed model schemas/artifacts still require clearance |
| Normative examples | Synthetic expected cases supplied; independent executable consumer and conformance qualification still required |
| Unknown/invalid/incomplete behavior | Explicit draft rules and adverse fixture expectations; actual reader/parser/evaluator runs still required |
| Artifact rights | File-class inventory and exact proposals above; owner selection and later included dependency/notice review still required |
| Disclosure | Candidate files checked separately from reviews; final compiled docs, source maps, archives and manifest/dependency scan still required |
| Promotion | Owner review of maintained publication-boundary and this concrete candidate; subsequent explicit destination/revision authorization still required |

Private extension example (excluded from candidate): a portfolio could prioritize
parent-graph invariants whose imported schema changed, then retain stale versus
new method evidence across a locked multi-repository configuration. This is a
private planning example with no executable or conformance claim. It is not a
hidden semantic rule for evaluating any released state invariant.

Failure controls: a clean consumer lacking a private schema must fail readiness;
a package manifest containing a private Git URL must fail promotion; an emitted
template requires its own notices; an unknown contributor's rights cannot be
inferred from repository ownership; an existing permissive file cannot acquire
new restrictions by being copied. Completing a local state-core inventory does
not clear Agent B's artifacts, TL dependencies or the ecosystem E04 review.
