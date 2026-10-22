---
id: FR-469
title: "Checked-package extension form nodes"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-467
    type: references
  - target: ix://agent-ix/quire-specification/FR-133
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-354
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-092
    type: references
---

# FR-469: Checked-package extension form nodes

## Description

When a `quire.checked-semantic-graph/v2` graph carries a declaration form of a
selected extension ([FR-467](../../functional/source-package/FR-467-define-extension-definition-members.md)),
the producer SHALL encode it as the `extension`/`extension_form` node below.
When a reader reads an `extension` node, the reader SHALL refuse a node whose
joins below do not hold with the refusal this requirement names, in
[FR-322](FR-322-checked-package-artifact.md)'s first-refusal order.

## Properties

- **Tag and form** — `node_tag: "extension"`, `semantic_form:
  "extension_form"`. The closed tag and form are the same for every
  extension. The extension's node kind is data in the body, never a tag or a
  form, so selecting a new extension changes neither the closed `node_tag`
  set nor any closed form list.
- **Body** — the Body production `extension_form`:
  `{term: "extension_form", definition, node_kind, clauses}`.
  - `definition` is the extension definition's `DefinitionRef`
    `{authority, identity}`.
  - `node_kind` is the `node_kind` of the typed-node schema entry
    ([FR-467](../../functional/source-package/FR-467-define-extension-definition-members.md)
    "Members") whose production the form is written in.
  - `clauses` is an array of `binding` terms, one per clause of the
    production, in the production's clause order. A binding's `name` is the
    clause name, and its `value` is a `reference` term to the clause's
    subnode: the node of the clause's checked value. Every clause value is
    its own node, so a clause value is never an inline `literal`,
    `aggregate` or `application`.
- **Declaration and owner** — the form's declared name is the node's
  `declaration` (`{qualified_name}`), with a `declaration` source occurrence,
  as a `record` node carries one. The node is a structural node carrying a
  `declaration`, so it carries the `SourceOwner` of the source unit that
  declares it (FR-322 "Owner member schema"). Its name is in FR-322's types
  namespace, so a form and a record of one name in one package refuse
  `ambiguous_declaration`/`ambiguous-name`.
- **Occurrences and source span** — the node has exactly one occurrence, role
  `declaration`, ordinal 0. Its `source_map` entry's `regions` are exactly
  the one region of the whole form, from the leading keyword through the
  closing brace. Each clause subnode carries its own occurrence and
  `source_map` entry for the clause value's region, by the rules for that
  subnode's own tag.
- **Type and dependencies** — the node is its own `semantic_type`. Its
  `dependencies` are exactly the unique digest-ascending targets of its
  clause references (FR-322's dependency rule). It carries no
  `recursion_group` and no `nominal_identity_preimage`. The node has no
  operation family, so no catalogued operand position admits a reference to
  it.
- **Node key** — the node is keyed by the closed `quire.structural-node/v1`
  preimage `{version: "quire.structural-node/v1", node_tag: "extension",
  semantic_form: "extension_form", semantic_type: null, declaration,
  recursion: null, owner, body}`, with `declaration` and `owner` equal to the
  node's members and `body` its `extension_form` body with each clause
  reference's target as its node key. The definition identity, the node
  kind, the declared name, the owner and every clause subnode's key
  therefore enter the node id and, through the `identity_projection`,
  `package_id`. The source span does not. FR-322's derived-shape key stage
  checks the key as it checks every other structural node.
- **Clause subnode types** — a clause entry's `type` names either a core
  `type-ref` or a node kind. For a core `type-ref`, the subnode's
  `semantic_type` is the type node that `type-ref` denotes, keyed by the same
  rules as every other node of that type, or, for a record type, a record
  type node that conforms to it (FR-322 "Record conformance"). For a node
  kind, the subnode is an `extension`/`extension_form` node whose
  `node_kind` is that node kind.
- **Carriage** — a selected extension enters a package only through its
  `lock.definition_selections` entry and its `extension` nodes.
  `lock.required_features` names feature identifiers, so the producer SHALL
  NOT list an extension definition's identity there. The refusal for an
  extension the reader does not hold is therefore the node-level
  `unknown_required_capability` of reader join 2, and the package-level
  `unknown_required_feature` never applies to an extension.
- **Published schema** — the published V2 package schema enumerates
  `extension` among the `node_tag` values with the one form
  `extension_form`, binds the `ExtensionFormBody` above to that tag and
  form, restricts its occurrences to one `declaration` occurrence, and lists
  `extension` among the tags that carry `declaration`. The node-identity
  preimage schema defines the closed `ExtensionFormNode` preimage above. The
  published fixtures `positive-extension-nodes.json` and
  `positive-all-families.json` each carry the
  `retention Audit { subject: "audit", days: 30 }` form of FR-467-AC-1.
- **Published definitions** — the proposal directory that holds the
  published fixtures (`proposals/checked-package-v2/`, the parent of
  `fixtures/`) publishes the extension definitions those fixtures select in
  `extension-definitions.json`, closed by
  `extension-definitions.schema.json`: `{definitions: [{reference,
  definition}]}`, one entry per definition, ascending by `reference`
  (`authority`, then `identity`). `reference` is the definition's
  `DefinitionRef` `{authority, identity}` as a
  `lock.definition_selections` entry spells it. `definition` is exactly the
  definition's two FR-467 members `{grammar_schema, typed_node_schema}` in
  FR-467's "Members" shape, with no other member. The file publishes no
  dependency edges: the reader holds caller-validated definitions, and FR-467
  validation, including its `definition-cycle` rule over the dependency
  edges, happens before a definition is held, outside the reader. A
  definition's clause typed by another extension's node kind, as `policy`'s
  `retain` clause is typed by the `retention` node kind, is therefore checked
  by join 5 against the subnode in the graph, never against an edge. The
  file publishes `retention` (FR-467-AC-1) and the `quota` and `policy`
  definitions of TC-951. A conformance reader of a published fixture holds,
  for each `lock.definition_selections` entry of the fixture, the published
  definition whose `reference` equals it, and holds no definition the file
  does not publish; a selection with no published definition is not held.
  The definitions are reader input under join 2 below, never fixture data,
  so loading them changes no fixture byte, node id or `package_id`.
  `extension-definition-vectors.json` names the definitions file and lists
  withheld-definition controls: each vector names a fixture, the references
  the reader withholds, and the expected result, either `admitted` or the
  `unknown_required_capability` refusal with `cause` `null`, the extension
  node's `node_id` as `locus` and
  `/semantic_graph/nodes/{i}/body/definition` as `document_pointer`. Its
  relative paths, the `definitions` file and each vector's `fixture`,
  resolve against `proposals/checked-package-v2/`.

## Reader joins

`read` is supplied the extension definitions the reader holds, beside its
limits and domain package documents. They are not artifact data and never
participate in package identity. The reader checks these joins at FR-322's
declaration step, after every protocol-reference carrier and before any
frame check, over the `extension` nodes in ascending node-id digest order,
and reports the first failing join of the first failing node:

1. **Definition selection.** `definition` equals a
   `lock.definition_selections` entry, else
   `missing_declaration`/`missing-selection` at
   `/semantic_graph/nodes/{i}/body/definition`.
2. **Held definition.** The reader holds that definition, else
   `unknown_required_capability`, which carries no cause (FR-272), with the
   node's `node_id` as `locus` and
   `/semantic_graph/nodes/{i}/body/definition` as `document_pointer`. The
   capability the package requires is the extension definition the node
   names. The refusal names the node by
   its exact identity and leaves the package's lock, source map and node
   unchanged, which is how a consumer lacking an extension retains its
   source and exact identity
   ([FR-133-AC-3](../../functional/source-package/FR-133-extend-language-uniformly.md)).
3. **Node kind.** `node_kind` equals the `node_kind` of a node entry of the
   held definition's `typed_node_schema`, else
   `missing_declaration`/`missing-name` at
   `/semantic_graph/nodes/{i}/body/node_kind`.
4. **Clause set.** The `clauses` names equal that node entry's clause names
   in order. At the first position where they differ, a clause name the
   entry lists later or not at all, or a repeated name, refuses
   `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{i}/body/clauses/{c}`.
   When every present clause matches but the entry lists more, the node
   refuses `invalid_package`/`missing-member` naming the first absent clause,
   at `/semantic_graph/nodes/{i}/body/clauses`.
5. **Clause types.** In clause order, a clause whose subnode does not meet
   its entry's `type` ("Clause subnode types") refuses
   `ill_typed`/`type-mismatch` naming the clause, the expected type or node
   kind and the subnode's `semantic_type`, at
   `/semantic_graph/nodes/{i}/body/clauses/{c}/value`. A clause target absent
   from the graph refuses `missing_declaration`/`missing-name` at the same
   pointer.

A body outside the `extension_form` production, an `extension` node with
another form, a second occurrence or an occurrence of another role, and a
clause value that is not a `reference` fail the V2 schema and refuse
`malformed_wire`. A `dependencies` list that is not the clause targets is
FR-322's `invalid_semantic_graph` at `/dependencies`, at the graph-shape step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-469-AC-1 | With FR-467-AC-1's `retention` extension selected and held, `retention Audit { subject: "audit", days: 30 }` is one `extension`/`extension_form` node with `declaration` `["Audit"]`, a `SourceOwner`, the `retention` definition's `DefinitionRef`, `node_kind` `retention`, the clause bindings `subject` and `days` in that order, each a `reference` to a subnode typed `Text` and `Int[1, 3650]`, `dependencies` exactly those two subnodes in digest order, and one `declaration` occurrence whose `source_map` region spans the whole form; the full reader admits the package. | Test (TC-951) |
| FR-469-AC-2 | The node's key recomputes from its `quire.structural-node/v1` `ExtensionFormNode` preimage; changing the definition identity, the node kind, the declared name, the owner or a clause subnode, with identities recomputed, changes the node id and `package_id`, and changing only the form's source region changes neither. A retained key after such a change refuses `invalid_package`/`stale-node-key` at the node. | Test (TC-951) |
| FR-469-AC-3 | A reader that does not hold the `retention` definition refuses the package `unknown_required_capability` with no cause, the node's `node_id` as `locus` and `/semantic_graph/nodes/{i}/body/definition` as `document_pointer`, and the package's lock, source map and node are unchanged; a `definition` absent from `lock.definition_selections` refuses `missing_declaration`/`missing-selection` at the same pointer before that check. | Test (TC-951) |
| FR-469-AC-4 | A `node_kind` the held definition does not declare refuses `missing_declaration`/`missing-name` at `/body/node_kind`; the clauses in the order `days`, `subject`, an unknown clause name and a repeated clause each refuse `invalid_package`/`invalid-value` at the offending clause; a body without `days` refuses `invalid_package`/`missing-member` naming `days` at `/body/clauses`. | Test (TC-951) |
| FR-469-AC-5 | A `days` subnode typed `Text` refuses `ill_typed`/`type-mismatch` naming `days`, `Int[1, 3650]` and `Text` at `/body/clauses/1/value`; a clause target absent from the graph refuses `missing_declaration`/`missing-name` at that pointer; a clause typed by a node kind whose subnode is an `extension` node of another node kind refuses `ill_typed`/`type-mismatch`. | Test (TC-951) |
| FR-469-AC-6 | An inline `literal` clause value, an `extension` node with a second occurrence or an `expression` occurrence, and an `extension` node with a form other than `extension_form` each fail the V2 schema and refuse `malformed_wire`; a `dependencies` list missing a clause target refuses `invalid_semantic_graph` at `/dependencies`; a record `Audit` beside the form `Audit` refuses `ambiguous_declaration`/`ambiguous-name` at the later node in node-id order. | Test (TC-951) |
| FR-469-AC-7 | The node kind is body data: a second extension with node kind `quota` uses the same `extension`/`extension_form` tag and form, and both V2 schemas admit it without change. | Test (TC-951) |
| FR-469-AC-8 | The published packages that carry the `retention` form list the `retention` definition in `lock.definition_selections` and list no definition identity in `lock.required_features`; a reader that does not hold the definition refuses `unknown_required_capability`, never `unknown_required_feature`. | Test (TC-951) |
| FR-469-AC-9 | `extension-definitions.json` meets `extension-definitions.schema.json`, lists one entry per `reference` in reference order, and gives each production of each definition a node entry of the same `keyword` listing the production's clause names in order. | Test (TC-951) |
| FR-469-AC-10 | Reading `positive-extension-nodes.json` and `positive-all-families.json` with the published definitions their locks select held admits both, and withholding `retention` refuses each `unknown_required_capability` with no cause, the `Audit` node's `node_id` as `locus` and that node's `/semantic_graph/nodes/{i}/body/definition` as `document_pointer`, exactly as `extension-definition-vectors.json` states. | Test (TC-951) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`, its
  closed tag set, the body grammar, the derived-shape key stage and the
  reader's refusal order.
- [FR-467](../../functional/source-package/FR-467-define-extension-definition-members.md)
  owns the grammar schema and typed-node schema members this node is checked
  against.
- [FR-133](../../functional/source-package/FR-133-extend-language-uniformly.md)
  owns what an extension may change and the refusal of a consumer that lacks
  it.
- [FR-272](../foundation/FR-272-typed-diagnostic-cause.md) owns the
  `unknown_required_capability` row and its `locus` and `document_pointer`.
- QSL FR-354 admits extension forms and emits this node; QSL FR-092 owns the
  `quire.structural-node/v1` key function.
