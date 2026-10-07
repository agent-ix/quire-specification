---
id: TC-428
title: "The plugin wire frames, identifies and settles every plugin run"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-462
    type: verifies
---

# TC-428: The plugin wire frames, identifies and settles every plugin run

## Description

Runs scripted plugin executables against the host over `quire.plugin-wire/v1`
and checks framing, the envelope, backend identity, cancellation, deadlines
and every failure settlement. Planned.

## Test Procedure

| Row | Plugin behaviour | Expected | AC |
| --- | --- | --- | --- |
| PW-01 | Sends `hello` with a valid manifest, answers each `request` with one `result`, exits after `shutdown` | One terminal record per routed item; each record names the backend identity the manifest names | FR-462-AC-1 |
| PW-02 | Sends a frame whose JSON is not in RFC 8785 form; a frame one byte over the frame-length limit; frames whose total exceeds the run's byte limit | Each affected item settles `failed` with a tool-failure cause | FR-462-AC-2 |
| PW-03 | Sends envelopes with `protocol` `quire.plugin-wire/v2`, an unknown `kind`, a missing `id`, an extra member, and a `result` naming an id the host did not send | Each is refused before its body is read, and each affected item settles `failed` | FR-462-AC-3 |
| PW-04 | Crashes mid-run; exits non-zero; returns a result for an item routed to another provider; returns a result for an unadvertised kind; omits one terminal record before `shutdown` | Each affected item settles `failed` with a tool-failure cause; items routed to other providers keep their own records | FR-462-AC-4 |
| PW-05 | The caller cancels: one plugin answers `cancel`, one ignores it past the grace period; a third plugin overruns the deadline | The first settles each open item `incomplete`, cause cancelled; the second is ended with the same settlement; the third is ended and each open item settles `incomplete`, cause `timed-out` | FR-462-AC-5 |
| PW-06 | A plugin at the named path that writes arbitrary text, including JSON, to standard error; a same-named executable placed on the search path | Settlements equal PW-01's; only the named path is started | FR-462-AC-6 |
| PW-07 | The host sends a `request` for `positive-all-families.json`; a recording plugin decodes `package` and checks it; the plugin reader is then given request bodies whose `package` is padded, uses `+` and `/`, has nonzero trailing bits, or decodes to a package whose `package_id` differs from the body's | The recorded `package` decodes to exactly the package's RFC 8785 bytes and the body's `package_id` is that package's; for each altered body no item runs and every routed item settles `failed`, with diagnostic `invalid_runtime_input`/`invalid-value` for the three spellings and `stale_dependency`/`content-mismatch` for the differing `package_id` | FR-462-AC-7 |
| PW-08 | Each PW-02 to PW-04 failure, and a `result` body its stage schema refuses | Each affected item settles `failed` with `tool-failure` and the `defect` of FR-462's failure table for that failure (`malformed-body` for the refused body) | FR-462-AC-8 |
| PW-09 | A plugin whose `hello` advertises the kind `proof`, one advertising the mode `partial`, one advertising `probabilistic-satisfaction` with no evidence kinds, and one advertising (`value-validity`, `bounded`) with no `domains`; then a request naming the first plugin's identity | Each registration is refused `invalid_capability` with `unknown-kind`, `unknown-mode`, `unknown-evidence-kind` or `invalid-domains`, keyed by its backend identity; no item settles `failed`; the request's item receives the unknown-backend mark | FR-462-AC-9 |

## Expected Results

Every plugin run is framed, identified by its manifest's backend identity and
settled per item, and every wire defect settles `failed` or `incomplete`
without touching items routed elsewhere.
