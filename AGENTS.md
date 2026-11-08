# Contributor and session guidance

Read README.md, LICENSE (`AGPL-3.0-or-later`) and the owning issue before editing. This repository holds the draft Quire language standard; it has no released version yet.

- Preserve authored/source identity and explicit unsupported/incomplete results.
- Use the first specification issue to establish requirements, acceptance fixtures and scoped implementation/CI work before scaffolding speculative crates.
- Inspect existing shared model, contract and Quoin interfaces before duplicating them.
- Do not infer licensing from process or repository boundaries.
- Keep private research, traces and issue-tracker links out of this repository. Source relicensing needs explicit approval.
- Use isolated worktrees for concurrent code sessions; preserve unrelated edits.
- Owner directive: build, test, and qualification checks run locally until stable. Hosted workflows for those checks must be manual-dispatch only (`workflow_dispatch`); do not add automatic triggers or dispatch a run without explicit direction. The existing CLA workflow may continue its automatic administration.
- Prototype workflow: maintain one current draft and use Git for working history.
  Update existing artifacts, record material decisions, and review at PR
  readiness; avoid repeated status logs, intermediate reviews and speculative
  assurance work.
- This repository has a Rust qualification test crate whose only library API is the dependency-free `quire_specification` accessors for authored definition bytes and the typed `QUALIFICATION_VECTORS` table generated from the TC-185/186/187/192/193 rows. Run the actual local
  commands in README.md. The native compiler remains in quire-spec-language.
  Preserve the distinction between schema/JCS producer checks and B's complete
  strict-reader/report qualification; the former does not discharge the latter.

<!-- required-language-policy:start -->
## Required implementation language and containment

Owner directive (2026-09-07): first-party production and possible qualification paths must be **Rust**, including libraries, CLI/tools, generators, validators, canonicalization, result adapters, test assertions and fixture/CI audits. "Glue", prototypes, scripts and tests are not exemptions; a Rust wrapper around non-Rust semantic logic is not remediation.

- Every new TypeScript choice needs explicit owner approval for its exact scope. Any other executable language must be brought to the owner's attention; do not silently introduce or expand it.
- Quoin's existing implementation is explicitly retained for now: **contain its spread, do not schedule a wholesale Quoin rewrite**. Keep new reusable semantics in Rust behind versioned, structured interfaces. Existing Node/TypeScript is not blanket permission for new components or expanded language scope.
- Reuse shared assurance contracts and Quoin evidence ownership; do not recreate local evidence frameworks. Classify each helper as an existing shared capability, a missing reusable capability, or domain-specific logic before porting.
- Complete /specify and /spec-review for the changed requirements/interfaces, resolve findings, then implement and verify. Preserve ongoing work. Do not claim a language port or qualification complete from a policy update.
- Filament repositories/tickets are excluded from this remediation. Interface consumption does not authorize producer changes.

Existing Quoin retention is the only accommodation recorded here; this block grants no new TypeScript approval.
<!-- required-language-policy:end -->

/specify and /spec-review are mandatory before implementation. Preserve already-written work; pause further unreviewed implementation/merge.
