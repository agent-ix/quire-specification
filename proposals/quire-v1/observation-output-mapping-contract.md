# Observation result consumer and output-mapping contract

Draft F-owned consumer boundary for OB03 and the composed `1-draft` baseline.
It reconciles structured observation results with executable and export consumers.
It does not define an IR type, target-language mapping profile, model schema,
canonicalizer, CLI, backend, parser, or evidence store. Native Quire remains the
only editable clause authority.

## Input identity and per-obligation disposition

A consumer receives a result for one requested assessment subject. For every
obligation in that subject, including an obligation the consumer cannot represent,
the input retains:

| Value | Required meaning |
| --- | --- |
| Subject and request | Exact linked-package/static subject, runtime assessment-input identity, requested capability, and selected observation binding/progress/closure identities. |
| Source correspondence | Native clause, source revision/digest/span, declaration/anchor/capture identities, selected model and profile identities. |
| Result facts | Independent B-owned execution, truth, activation, participation, completeness, adequacy, assumptions, and provenance facts; no summary may erase another fact. |
| Consumer selection | Consumer identity/version, selected target/output profile, configuration, and declared supported capability. |
| Mapping outcome | One of `preserved`, `conditional`, `unrepresented`, or `refused`, with an exact source-bound reason and any required premise or loss. |

The consumer preserves all four mapping outcomes independently from the observation
availability state. A result that is pending, incomplete, refused, untriggered,
unsupported, or not executed remains that state at the boundary; it cannot be
converted to a Boolean pass, a mapped success, or a qualified run.

## Mapping outcome rules

- **preserved** means the selected consumer contract represents the named
  obligation and every required selected premise. It is not a claim that a foreign
  parser accepted text, a target executed, or an external tool agreed.
- **conditional** means the output retains an identified premise outside the
  selected consumer contract. The premise, source identity, and missing
  correspondence remain visible.
- **unrepresented** means the selected target profile has no representation for
  the obligation or an essential selected fact. The original obligation remains
  listed; it is not silently omitted.
- **refused** means input identity, capability, profile, mapping selection, or
  resource admission prevents deterministic output. Retain the typed reason and
  affected identity; do not emit a plausible substitute.

A target syntax parser, serializer, or foreign-tool acceptance check is only a
structural observation. It does not promote any outcome to preserved or establish
semantic equivalence.

## Consumer restrictions

Codegen/executable consumers may consume this envelope only under their declared
capability and must not become a clause authority, recreate B's result/evidence
family, infer missing observations, or replace D/E-owned mapping semantics.
Exporters may emit OCL, SysML v2/KerML, or FRETish text as outputs, but no
foreign runtime is required for generation or qualification. Every first-party
executable adapter, reader, loss reporter, fixture audit, and test path is Rust
1.98.1.

## Planned controls

For one common O1/O2 assessment, map a preserved obligation and independently
exercise conditional capture/clock correspondence, an unrepresented history or
collection feature, a refused stale profile, pending/incomplete/refused result
states, and a parser-accepted generated-text control. The controls retain their
source/binding/model/profile identity and prove no target parse or serialization
result becomes a semantic or execution claim. They are planned in
[TC-146](../../spec/test-cases/TC-146-map-observation-results-to-consumers.md)
and [IT-051](../../spec/integration/IT-051-observation-output-consumer-contract.md).

