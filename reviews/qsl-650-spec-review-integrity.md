---
id: SR-1305
title: "Spec integrity review of PR #194 (QSL-650): FR-331, FR-462, TC-428, TC-431 structure"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@045b37b3bd08d2f5a55b65b424274e7ac435d922; PR #194 diff against origin/main 65e816f: FR-331, FR-462, TC-428, TC-431, proposals/backend-provider-v1/schema.json"
review_set: subset
---
# Spec integrity review of PR #194 (QSL-650)

## Summary

Ticket: QSL-650. This is a structural pass over the edited artifacts. It
checks table integrity, AC atomicity, Verification cells, trace links, and
whether the schema is closed where FR-331 says it is.

Examined:
- FR-462 frame-body table and the paragraph added after it (examined)
- FR-462-AC-7 and TC-428 PW-07 (examined)
- FR-331-AC-18, FR-331-AC-19, FR-331-AC-20 Verification cells (examined)
- TC-431 frontmatter relationships and BP rows (examined)
- FR-331 Result entries `evidence` sentence and schema `Result.evidence` (examined)
- FR-331 frontmatter relationships added: FR-462, FR-322, FR-196, FR-272 (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new paragraph "Each body is one stage of the FR-331 wire schema …" sits inside the frame-body table, between the `result` row (line 70) and the `cancel` row (line 79). The paragraph ends the table. The `cancel`, `shutdown` and `fault` rows then render as a headerless fragment, so three of the six frame kinds drop out of the table that defines them. Move the paragraph below line 81. | spec/functional/tooling/FR-462-speak-the-plugin-wire.md:66-81 |
| FND-002 | low | The Verification cells of FR-331-AC-18, AC-19 and AC-20 read "Test", and FR-462-AC-7's reads "Test", with no TC id. Every other AC in both files names its TC. TC-431 BP-01 to BP-06 verify the FR-331 ACs and TC-428 PW-07 verifies FR-462-AC-7, so write "Test (TC-431)" and "Test (TC-428)". | spec/objects/interfaces/FR-331-backend-provider-envelope.md:309-311, spec/functional/tooling/FR-462-speak-the-plugin-wire.md:140 |
| FND-003 | low | FR-462-AC-7 is compound. It covers the encoding, four refusals and the whole 14-row defect table in one criterion. PW-07's expected column reaches back into PW-02 to PW-04 for the defect mapping, so one failing defect fails an AC that is mostly about package bytes. Split it: AC-7 for `package`/`package_id` and their refusals, and a new AC for "each wire failure settles `failed` with `tool-failure` and the table's `defect`". Give the new AC its own PW row. | spec/functional/tooling/FR-462-speak-the-plugin-wire.md:140, spec/test-cases/TC-428-plugin-wire.md:28 |
| FND-004 | low | `Result.evidence` is the unconstrained schema `{}`. FR-331 says `evidence` is FR-419 closed-form evidence or, for `well-definedness-unchecked`, "the FR-393 witness arm". The schema defines that arm (`GraphEvidence`, `arm: witness`) but does not reference it. The README's list of owner-shaped leaves names FR-419 only. As written, any JSON value passes as `evidence` on an `inconclusive` result, with any cause, and on `refuted` it is forbidden only by name. Constrain it: on `inconclusive` with `well-definedness-unchecked`, `$ref` the GraphEvidence witness arm. Forbid it on other inconclusive causes. Leave it open only under `closed-form`, as FR-419 owns that shape. | proposals/backend-provider-v1/schema.json:810, proposals/backend-provider-v1/schema.json:823, spec/objects/interfaces/FR-331-backend-provider-envelope.md:185-186 |

## Verdict

The trace links are coherent. TC-431 `verifies` FR-331, and its BP rows map
to AC-18, AC-19, AC-20 and AC-4. PW-07 maps to FR-462-AC-7. FR-331's new
`references` edges point to FRs the text actually cites. FND-001 is a real
document defect, a broken normative table, and must be fixed before merge.
FND-002 to FND-004 are small and belong in this PR.
