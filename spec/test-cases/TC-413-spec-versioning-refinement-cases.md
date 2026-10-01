---
id: TC-413
title: "Spec-versioning refinement cases"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-452
    type: verifies
---

# TC-413: Spec-versioning refinement cases

## Description

Run the spec-versioning refinement gate over revision pairs built to give
each case result, and confirm the case results, the verdict and the exit
code.

## Test Procedure

Build a test-only corpus over the ConfigVersion specification. Run the
gate through the real local Rust boundary over each vector's corpus and read
every case result, the verdict and the exit code. Each expected result is a
literal in the test; no test computes its expectation with the gate's own
classification or comparison.

## Vectors

| Vector | Pair | Expected result |
| --- | --- | --- |
| SV-01 | The superseding revision tightens one ConfigVersion clause; one case the prior admits and the superseding refuses; two cases both admit | The tightened case is a regression; the two others hold; verdict violation, exit 10 |
| SV-02 | The superseding revision drops the selected clause | The case holds; verdict success, exit 0 |
| SV-03 | The superseding revision drops a selected Boolean function | Regression; exit 10 |
| SV-04 | The superseding compile refuses `unsupported_construct`/`not-yet-implemented` for a case the prior admits | Unresolved, not holds; verdict unsupported, exit 21 |
| SV-05 | The superseding run of an admitted case has a one-unit work limit | Unresolved, not holds; verdict incomplete, exit 22 |
| SV-06 | SV-01 with its two revision labels swapped and the same `prior`/`superseding` naming | The same case results as SV-01 |
| SV-07 | A case the prior revision refuses | Not applicable |

## Expected Results

Each vector gives exactly its listed case results, verdict and exit code.
An unsupported or incomplete case is never reported as holding, and every
regression is listed whatever the verdict.
