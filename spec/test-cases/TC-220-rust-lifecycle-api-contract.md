---
id: TC-220
title: "Rust lifecycle API contract"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-220: Rust lifecycle API contract

## Description

Calls every lifecycle operation through the Rust library with valid,
wrong-stage, cancelled and foreign-format inputs, and `prove` over a checked
package that `package` refuses to emit. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| LC-01 | One `1-draft` source through `parse`, `select`, `check`, `package`, `execute`, `monitor`, `analyze`, `lower`, `generate`, `prove`, `replay`, `inspect` and `render`, each with explicit limits and a cancellation handle | Each returns its typed outcome, and each artifact keeps the source, definition and capability identity of its input | FR-300-AC-1 |
| LC-02 | A `compile_fail` test passing `check`'s output where `execute` takes a checked package, and `parse`'s output to `package`; a request naming an unknown profile | The wrong-stage calls fail to compile; the unknown profile returns a typed failure and no partial artifact | FR-300-AC-2 |
| LC-03 | The LC-01 `check`, `run` and `prove` requests through the library and through the CLI in machine mode | The two JSON outcome documents are byte-equal | FR-300-AC-3 |
| LC-04 | Cancel a running `check`, `execute`, `analyze` and `prove` after its first charge | Each stops within one charge; `check` and `execute` return `cancelled` with cause `requested` and no partial artifact; each open `analyze` and `prove` item settles `incomplete` with the cancellation cause | FR-300-AC-4 |
| LC-05 | A serialized outcome; the same document with its `format` changed to another identity | The first carries its `format` identity; the reader refuses the second | FR-300-AC-5 |
| LC-06 | A checked package declaring a `statistical` probabilistic claim, which `package` refuses; `prove` with the claim's item and a subject binding routed to the in-process statistical engine, then with a process provider named as its backend; `analyze` with the same item and binding | The in-process record is `measured` and equals `analyze`'s; the process-provider item settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming `probabilistic-satisfaction`, the backend and that the package is not emitted, with no request frame written | FR-300-AC-6 |
| LC-07 | The LC-06 in-process request issued twice in one run, counting the engine's runs; the LC-06 process-provider item, counting the frames written; a request frame of the same items with `package` removed | The in-process request holds the manifest, `deadline_ms`, items, checked package and binding as typed values; its results carry the same members as a process provider's; the engine runs twice and the second result comes from the second run; the process-provider item settles `unsupported` with `unsupported_projection`/`unsupported-requested-capability` naming `probabilistic-satisfaction`, the backend and that the package is not emitted, and no frame is written for it; the frame refuses before consumption | FR-331-AC-33 |

## Expected Results

Every operation is typed end to end, wrong-stage input cannot be written,
cancellation stops within one charge with no partial artifact, and the library
and CLI agree byte for byte.
