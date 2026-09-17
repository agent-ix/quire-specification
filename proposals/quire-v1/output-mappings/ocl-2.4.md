# OCL 2.4 output profile

Selected identity `quire.output.ocl24/v1`, revision `1-draft.1`. Target standard:
OMG OCL 2.4, normative document `formal/14-02-03`.

## Admitted fragment

The profile generates named invariant and operation pre/postcondition output
over an exact model correspondence from spec artifacts. Its initial expression
fragment contains total Boolean literals and `not`/`and`/`or`/`implies`, equality
and ordered comparison of equal types, bounded integer literals and checked
`+`/`-`/`*`, exact field reads, immutable local bindings, and bounded
`Sequence` `size`/`includes`/`count`/`forAll`/`exists` operations.

A native operation pre-state read maps to OCL `@pre` only when the exact
operation, receiver, field and pre/post correspondence is selected. The current
value is never substituted. Native ordered duplicate-preserving collections map
only to `Sequence`; no Set/Bag/OrderedSet coercion is permitted.

## Conditional and unsupported meaning

- Native bounded integer arithmetic is preserved only over the admitted native
  domain after all intermediate bounds are proved. OCL's wider integer domain
  does not enlarge the native claim.
- Optional, absent, invalid, nullable or unavailable values are conditional only
  under an explicit total definedness correspondence; otherwise they are
  unrepresented. OCL null/invalid behavior is never treated as native absence.
- Exact rationals with denominator other than one, integer division/remainder,
  one-based indexed access, collection transformation/sum, object/relationship
  navigation without an exact model mapping, temporal operators and protocol
  control are unrepresented by this revision.
- A ConfigVersion scalar bound/invariant or exact operation pre/post comparison
  is eligible. ParentPrecedes navigation is preserved only with an exact
  relationship/endpoint mapping; otherwise that obligation is unrepresented or
  refused as a whole rather than emitted without the relationship.

## Output and qualification boundary

Generated OCL is output-only. Parser/typechecker acceptance, including by a
tool with a different standard-library revision, is structural evidence. It
does not establish OCL 2.4 evaluation equivalence, native preservation or a new
input language. An implementation may add a qualified observer only through a
separate exact tool/dependency/license selection.
