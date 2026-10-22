<!-- reviewer-dispositions repo=agent-ix/quire-specification visibility=private quoin=0.28.3 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-7887 round=3 reviewed=d2ec448dcdf6829020e30c0594d5f7dec728f803 model=claude-opus-5-5 run=edec2bd5-9066-43cb-8070-f86f8f86b671 -->

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | d2ec448 |
| FND-009 | fixed | d2ec448 |
| FND-010 | fixed | d2ec448 |

New findings (disposition pass 3):

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-011 | low | other | FR-322-AC-69 states the `case` admission twice: 'A `case` whose arm bodies are `Money` and `TaxedMoney` with result type `Money` is admitted (FR-440-AC-4)', then again inside the next sentence ('.. |

+++ [reviewer data]

```yaml
dispositions:
- fnd: FND-008
  outcome: fixed
  after_excerpt: 'The parameters of a `function.call` come from the callee `function` node''s body: its first binding, named `parameters`, is an `aggregate` of `reference` terms to the `value`/`parameter` nodes in parameter order, each typed by its `semantic_type`, including a parameter the body never reads (QSL FR-092 "Function nodes"). A callee whose body has no `parameters` binding takes no parameter, and a `dependency_reference` callee uses the same binding of the named dependency node. That binding is the one source of a function''s declared parameter types; no other member carries them.'
- fnd: FND-009
  outcome: fixed
  after_excerpt: 'of `{version: "quire.application-node/v1", node_tag, semantic_form, semantic_type, declaration, recursion, body}`.  (the `parameters` wire member is removed; schema.json and node-identity-preimage.schema.json differ from main only by the `event_type` enum entries)'
- fnd: FND-010
  outcome: fixed
  after_excerpt: 'A supplied composite value whose declaration key is neither a key of that package''s graph nor, for a model record value type or event type, a declaration key of the evaluating package''s selected document is `refused { code: invalid_runtime_input, cause: wrong-value-kind }` during input validation, before any evaluation or charge.'
new_findings:
- fnd: FND-011
  severity: low
  confidence: high
  check: other
  artifact_id: FR-322-AC-69
  related_ids:
  - FR-440-AC-4
  path: spec/objects/interfaces/FR-322-checked-package-artifact.md
  lines: '1436'
  excerpt: '[...] A `case` whose arm bodies are `Money` and `TaxedMoney` with result type `Money` is admitted (FR-440-AC-4). A `function` node typed `Money` whose body root is an application typed `TaxedMoney`, and a `case` whose arm bodies are typed `Money` and `TaxedMoney` with result type `Money`, are admitted; a `Money` body at a `TaxedMoney` function is refused. [...]'
  finding: 'FR-322-AC-69 states the `case` admission twice: ''A `case` whose arm bodies are `Money` and `TaxedMoney` with result type `Money` is admitted (FR-440-AC-4)'', then again inside the next sentence (''... and a `case` whose arm bodies are typed `Money` and `TaxedMoney` with result type `Money`, are admitted''). Delete one.'
  method: spec-review/integrity
```

+++
