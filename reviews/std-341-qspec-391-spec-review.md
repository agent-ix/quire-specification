---
id: SR-16207
title: "spec-review of the backend descriptor tool member, ManifestDigest and cache-key preimage (QSpec PR 391)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@d6b07993; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/functional/tooling/FR-306-compile-and-cache-exactly.md; spec/objects/foundation/FR-201-canonical-identity-domain.md; spec/test-cases/TC-226-aot-jit-native-and-cache-parity.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; proposals/backend-provider-v1/{README.md,schema.json,vectors.json}; tests/backend_provider_v1.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-306
    type: references
  - target: ix://agent-ix/quire-specification/FR-201
    type: references
  - target: ix://agent-ix/quire-specification/TC-226
    type: references
  - target: ix://agent-ix/quire-specification/TC-431
    type: references
---

# Spec review of the backend descriptor tool member and cache-key preimage (QSpec PR 391)

## Summary

Ticket: STD-341. One spec-review pass over `git diff origin/main...d6b07993`
(paired with QSL PR 1302). Oracles: QSpec FR-290 (registry byte-equality,
`duplicate-backend`, tool-absence probe), FR-305 (plugins, never stored),
FR-322 (`PackageId` digest-record shape), QSL ADR-029 CA-2, QSL FR-288 and
ADR-013 (the `ManifestDigest` lives on the QSL `BackendDescriptor`).

Gates, run at the head: `quire validate --scope . 'spec/**/*.md'` gives 86
failure lines, the same as main; `cargo test` passes in full (129 + 24 + the
small suites, 0 failed), including
`tc_431_tool_version_is_manifest_content_outside_the_backend_id` and the
generic `tc_431_adverse_vectors_are_refused`, which refuses the five new
adverse vectors at their patched members. QSL conformance
`cargo test -p qsl-package --lib checked_v2` on QSL main 71a080048 with
`QSPEC_DIR` at this head: 46 passed, 0 failed. origin/main is an ancestor of
the head.

Examined, clean:

- Key preimage. The closed object `{operation, package_id, request, backend,
  manifest, target}` with "No other member enters the key preimage" is
  sound: `tool` is inside the descriptor, the descriptor is the whole
  `ManifestDigest` preimage, and the digest record is a key member, so a
  descriptor change that touches only `tool.version` changes the RFC 8785
  key bytes while `backend` stays equal. `target` absent (not null) for
  `prove`/`analyze` is well defined under JCS.
- Canonical bytes. `ManifestDigest` is SHA-256 over the RFC 8785 bytes of
  the one descriptor (not the enclosing array, stage, snapshot or request),
  labelled `quire.tool-manifest.jcs/v1`; the `{domain, algorithm, digest}`
  record matches FR-322's `PackageId` shape and the corpus's `sha256`
  algorithm token. FR-201's description (domain compared before bytes) makes
  the domain a label, so FR-201-AC-8's "unequal in any other domain" holds.
- FR-306-AC-2, AC-6 and FR-201-AC-8 can each fail; TC-226 EB-02, EB-06 and
  EB-07 carry them. FR-331-AC-37's schema clauses are exercised by the test
  and the five adverse vectors.
- Backward effect. `kani-hello` gained `tool`; its pointer-owner map gained
  `/manifest/0/tool`. Every adverse vector based on `kani-hello` still
  refuses at its own patch, and the QSL conformance run is green.
- Ids. FR-331-AC-37, FR-306-AC-6, TC-431 BP-33 and TC-226 EB-07 are not on
  main or in any other open QSpec PR (392, 390, 389, 267, 207, 59). Open PR
  390 also edits the schema and vectors and merges with this head without
  conflict. Draft PR 207 conflicts textually (FR-201-AC-8, FR-306, TC-226);
  the PR body states the supersession and that 207 can be closed, but 207
  is still open with no note on it.
- Lead defaults are flagged as "not owner answers", including the
  extension past the ticket (`tool` on a linked provider's descriptor whose
  backend runs an external tool).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | The tool-only-upgrade miss rests on a rule nothing verifies. Process providers are plugins (FR-305) and FR-306 never stores a plugin result, so the schema-enforced `hello` requirement keys no cache entry; the only cached results with an external tool are a non-plugin linked provider's. For that case FR-331 says `tool` is required "on a linked provider's descriptor whose backend runs an external tool", but no AC refuses or detects such a descriptor without `tool` (AC-37 admits every snapshot descriptor without it), "runs an external tool" is not defined, and FR-305 also calls a compile-time linked provider a plugin, so the text does not say which linked descriptors are cacheable. A linked Kani descriptor that omits `tool` passes every AC and its results survive a Kani upgrade. Fix: name the cacheable provider class (a first-party linked provider that is not an FR-305 plugin), make `tool` required on it, and add an AC whose failure is the omission (for example, a store refusing a cacheable result from such a descriptor without `tool`). | spec/objects/interfaces/FR-331-backend-provider-envelope.md:89; spec/functional/tooling/FR-306-compile-and-cache-exactly.md:133-135; spec/functional/tooling/FR-305-load-explicit-rust-plugins.md:38-41 | missing-requirement |
| FND-002 | medium | `tool.version` is "that tool's version as the tool reports it", but nothing says when it is obtained or binds it to the tool that runs. A linked provider can build its descriptor once (at compile time or start-up) while the tool on the host is upgraded; the descriptor, the `ManifestDigest` and the key stay the same and the old entry hits, which is the stale hit STD-341 exists to refuse. FR-306-AC-6 and TC-226 EB-07 change only the descriptor, never the tool, so they cannot catch it, and the FR-290 probe that does query the tool is not tied to the descriptor. Lead default 1 (no separate probe key member) is fine, but the version still needs a source rule. Fix: state that the provider obtains `tool.version` from the tool it will run, at registration of each run, and that a probe reporting a different version than the descriptor's is a registration refusal (or `tool-unavailable`); add an AC/TC row that upgrades the tool without editing the descriptor source. | spec/functional/tooling/FR-306-compile-and-cache-exactly.md:146; spec/objects/interfaces/FR-331-backend-provider-envelope.md:89; spec/objects/protocol/FR-290-protocol-claim-kind.md:364-378 | missing-requirement |
| FND-003 | medium | The corpus contradicts the new snapshot rule. FR-331 says "a registry snapshot carries each descriptor as its provider gave it, so a snapshot descriptor carries `tool` exactly when its provider's does", and the README repeats it, but `kani-request` and `request-item-level-defects` carry a `kani` snapshot descriptor without `tool` while `kani-hello`, the kani provider's descriptor, now carries `tool`. Under FR-290's byte-equality rule a registry built from `kani-hello` produces a different snapshot from the one these requests check candidates against, so a conformer that builds the snapshot from the fixtures' own hello disagrees with the request vectors. Fix: add the same `tool` to the `kani` descriptor in the request-stage positives, or make the request fixtures name a linked provider without a tool. | proposals/backend-provider-v1/vectors.json (positive kani-request, request-item-level-defects); proposals/backend-provider-v1/README.md:49-51 | wrong-requirement |
| FND-004 | low | Every `hello` descriptor must carry `tool`, the "external tool its backend runs", but a process plugin whose engine is its own code runs no external tool; the spec gives it no value to send, so two authors would put the plugin's own name and version or invent one. Harmless for caching (plugin results are never stored) but ambiguous. Fix: say that a process provider with no external tool names its own executable and version in `tool`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:89; proposals/backend-provider-v1/schema.json:22 (manifest stage) | wrong-requirement |
| FND-005 | low | TC-431 BP-33 says "the descriptor without `tool` is admitted as a registry-snapshot descriptor", but the test validates it against `$defs.BackendDescriptor` alone, not as a member of a request-stage `manifest`, and the digest check hashes plain `jcs_sha256` of the descriptor rather than the `quire.tool-manifest.jcs/v1` record FR-331-AC-37 names. Both pass today, but a request-stage rule that required `tool` would not fail this test. Fix: validate the toolless descriptor inside a `RequestStage` snapshot (for example `kani-request` with `tool` removed). | tests/backend_provider_v1.rs:1393-1401; spec/test-cases/TC-431-backend-provider-wire-vectors.md:61 | correct-requirement-no-evidence |

## Verdict

Not mergeable as is. The preimage, its canonical bytes and the identity
split are sound, all gates pass, and the ids are unique. FND-001 is high:
the one class of cached result that runs an external tool is covered only by
an unverified, undefined rule, so the PR does not yet guarantee the
tool-only-upgrade miss the ticket asks for. FND-002 and FND-003 should be
fixed in the same round; FND-004 and FND-005 are small. Before merge, close
QSpec PR 207 with a pointer to this PR (it conflicts textually).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-006 | low | The probe now fails on a version other than the descriptor's `tool.version` (FR-331 "Backend tool", FR-290 Tool absence), but two restatements of the probe outcome were not widened: FR-331's `results` bullet still records `unsupported` for "a supported, routed item whose tool probe finds the tool missing, errors or exceeds its limit", and FR-290-AC-8 still reads "a routed item whose tool is missing, or whose probe errors or exceeds its limit". A reader of either misses the version-mismatch case. FR-331-AC-38 carries the behaviour, so this is wording. Fix: add "or reports another version than the descriptor's `tool.version`" to both. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:97; spec/objects/protocol/FR-290-protocol-claim-kind.md:396 | wrong-requirement |
| FND-007 | low | FR-331-AC-38 is verified by "Test (TC-226)" and TC-226 EB-08 names FR-331-AC-38, but TC-226's frontmatter `relationships` verify only FR-306 and FR-201, so the graph has no TC-226 -> FR-331 edge. Fix: add `ix://agent-ix/quire-specification/FR-331` `verifies` to TC-226. | spec/test-cases/TC-226-aot-jit-native-and-cache-parity.md:5-9 | correct-requirement-no-evidence |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-008 | medium | A pure in-process engine's `tool.version` is "that crate's package version in the producer's build", so its key changes only when the crate's `Cargo.toml` version changes. Two producer builds whose engine code differs but whose crate version is equal (every QSL main build between version bumps: `qsl-analyze` is `0.1.0` on main while its code moves) give equal descriptors, equal `ManifestDigest`s and equal keys, so the second build returns the first build's `analyze` result, which is the stale hit this ticket refuses for external tools. "So a release of the engine's code changes the descriptor" holds only if every behaviour change ships with a version bump, and nothing states or checks that. FR-306-AC-8 and EB-09 change the crate version, so they cannot catch it. Fix: bind the engine's code identity rather than its declared version (for example the producer's build identity or a digest of the engine crate's source), or state that the producer's build sets the version to a value that changes with every change to the engine's code and add a row where code changes with the version held. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:328-330; spec/functional/tooling/FR-306-compile-and-cache-exactly.md:116-128 | missing-requirement |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 09371106 every descriptor must carry `tool` (schema `required: ["backend", "tool", "advertises"]`, `oneOf` BackendTool or `producer`); FR-331 "Backend tool" defines "fronts an external tool" and "pure in-process engine"; FR-305 makes a plugin a third-party provider and a first-party provider (QSL engine, the Kani backend) not a plugin; FR-306 caches first-party providers' results only; FR-331-AC-37 refuses a snapshot descriptor without `tool` (`snapshot-kani-without-tool`). |
| FND-002 | fixed | FR-331 "Backend tool": the provider queries the executable for `tool.version` each time it builds its descriptor, the registry is built afresh at each invocation, and the FR-290 probe compares the executable's version with the descriptor's and fails `tool-unavailable` naming both. FR-306 reads the cache only after the probe passes and never stores `tool-unavailable`. FR-306-AC-7, FR-331-AC-38 and TC-226 EB-08 replace the executable without editing any descriptor source. |
| FND-003 | fixed | `kani-request` (and the other request positives) now carry the kani descriptor's `tool`; the test asserts `request["manifest"][0] == entry` from `kani-hello`, and `temporal-request`'s engine carries `producer`. |
| FND-004 | fixed | FR-331 "Backend tool": "A process provider whose engine is its own code and that runs no other executable names its own executable and its version." |
| FND-005 | fixed | The test validates the toolless and malformed descriptors inside the `kani-request` request stage with the whole-envelope validator and compares `manifest_digest` records `{domain: quire.tool-manifest.jcs/v1, algorithm: sha256, digest}`. |
| FND-006 | fixed | At 4a27b742 FR-331 `results`: "whose tool probe finds the tool missing, finds a version other than the descriptor's `tool.version` ("Backend tool"), errors or exceeds its limit"; FR-290-AC-8: "or whose probe finds a version other than the descriptor's `tool.version`, errors or exceeds its limit". |
| FND-007 | fixed | TC-226 frontmatter now has `target: ix://agent-ix/quire-specification/FR-331`, `type: verifies`. |
