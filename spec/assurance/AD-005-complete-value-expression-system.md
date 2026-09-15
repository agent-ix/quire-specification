---
id: AD-005
title: "Complete value, type, expression and collection system"
type: ArchitectureDescription
status: accepted
owner: quire-agent-a
system: native Quire V1 semantic kernel
relationships:
  - target: ix://agent-ix/quire-specification/AD-001
    type: references
  - target: ix://agent-ix/quire-specification/AD-003
    type: references
---

# Complete value, type, expression and collection system

## System Boundary

S2 owns the versioned value/type algebra, pure expressions, functions,
definedness and collection semantics shared by state clauses, temporal atoms,
protocol guards, analyzers and mappings. QSL is the reference checker and
evaluator. The cycle-free contract model may represent these meanings, but no
Rust type, IR encoding or backend limit defines the language.

Complete V1 contains Boolean, mathematical and bounded integers, normalized
rationals, exact decimals, explicit IEEE binary-float profiles, text,
declaration-qualified enumerations, quantities, presence/null, records, tuples,
finite recursive values, object references, sequences, sets, bags and ordered
sets. All conversion is explicit and reports loss or refusal.

## Views

```mermaid
flowchart LR
  D[Versioned type declarations] --> C[Type and termination checker]
  E[Located typed expressions] --> C
  C --> K[Checked pure expression graph]
  K --> R[Reference evaluator]
  K --> I[Target-neutral IR]
  R --> O[Value or typed non-result]
  I --> B[Exact backend capability check]
```

Values are mathematical and profile-selected. Consumers needing finite domains
must obtain explicit bounds; they cannot invent maxima, round exact values or
encode absence as a scalar. Records and tuples use declared structural
equality, while object references use qualified identity. Recursive type
definitions require finite runtime values and bounded traversal.

Pure functions declare parameter and result types, effects=`none`, a
termination obligation and resource limits. Recursion is admitted only after a
well-founded decrease is established. Runtime fuel remains independent:
exhaustion returns incomplete even for a statically terminating function.

Collections share bounded traversal operations while preserving kind-specific
order, uniqueness and multiplicity. `map` and `collect` are one operation;
filter, flatten, fold/reduce, quantifiers and conversions have explicit result
kinds, empty cases, intermediate bounds and deterministic exposure rules.

## Decisions

| Choice | Rejected alternative | Consequence |
| --- | --- | --- |
| One shared typed expression graph | Separate expression languages per family | State, temporal and protocol consumers share exact typing and source correspondence. |
| Mathematical semantics plus explicit finite admission | Machine-width or backend-driven semantics | Finite consumers refuse missing bounds without narrowing the language. |
| Exact decimal and explicit IEEE profiles are distinct | A generic numeric/float type | Decimal equality is coefficient/scale semantic; IEEE exceptional values and rounding remain profile-visible. |
| Checked recursion plus independent runtime limits | Refuse all recursion or trust fuel as termination | V1 admits total recursive functions without claiming that exhaustion is a logical result. |
| Collection kinds retain their algebra | Convert everything to sequences | Order, uniqueness and multiplicity cannot be silently fabricated or lost. |

## Risks

| Risk | Control |
| --- | --- |
| A backend silently approximates unsupported values | Per-node capability negotiation and explicit refusal before artifact emission. |
| Structural and object identity collapse | Distinct types and equality matrices; no cross-domain equality. |
| Float nondeterminism escapes provenance | Width, rounding mode, exceptional-value policy and tool pins enter the package/run identity. |
| Recursive or aggregate work becomes unbounded | Static termination/domain obligations plus separately charged runtime limits. |
