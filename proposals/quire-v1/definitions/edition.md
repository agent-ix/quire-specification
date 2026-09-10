# Native Quire composed edition definition

Definition identity: `ix:native`; language edition: `1-draft`;
definition revision: `1-draft.1`. **Proposed, not adopted or implemented.**

## Selection and scope

The compiler inventory binds this exact definition artifact; source headers
select `language "ix:native" edition "1-draft";`. The artifact byte digest,
definition revision and language/edition remain distinct. This is the first
concrete definition candidate for the composed edition, not a replacement
identity for historical `0-draft` sources.

This definition owns the complete lexical and syntactic grammar, closed source
inventory, common name resolution, source correspondence, static package
components, profile dependency resolution and typed diagnostic stages. Parsing
a temporal/protocol form does not admit its semantics without that family's
exact selected definition.

A profile definition's admitted-form catalog determines which recognized
constructs it permits. Its selected rule files supply their semantics and all
applicable type, definedness, source and binding constraints. Rules describing
another facet do not grant that facet's forms. A child profile may add explicitly
listed forms; it cannot weaken inherited numeric, presence, identity, purity,
anchor or resource rules. Imported predicates retain their own defining profile.

Required definitions use exact identity, revision and byte digest. Unknown or
conflicting definitions, cycles and incompatible inherited rules refuse dependent
admission. No definition is a runtime script, plugin or implicit network fetch.
A compiler reports support for the selected contract separately from knowing its
bytes. Markdown is the normative document encoding, not another clause parser.

Only the rule files explicitly selected below and transitive required definitions
are normative dependencies. Their external/background links do not silently add
unversioned rules. Source-model and observation contracts remain explicit
parameters of the linked package; neither this definition nor a profile supplies
missing producer authority.

## Selected rule files

All rule files below are selected at repository revision `033fcda3cc9f964d018b466faa12cd14c8a9878a`.
The digest is SHA-256 of the entire file's exact bytes. A link is a locator;
changed working-tree bytes do not replace the selected snapshot.

| Rule file | Exact byte digest |
| --- | --- |
| [shared-grammar.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/proposals/quire-v1/shared-grammar.md) | `sha256:d6f74466c1afaf1f4f235668edaaf445c3b5042e3a75d690083d5713cdc4d38f` |
| [package-contract.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/proposals/quire-v1/package-contract.md) | `sha256:f7faa516da17b64d4eeab582a2943a9aad7382e09f4c39e953c90f02e62dee0b` |
| [FR-030-bind-composed-definitions.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-030-bind-composed-definitions.md) | `sha256:f460982bb2de2cce8beb82a5f0453dac891e49780ab2571d7cf56eb8ed4e9c9a` |
| [FR-031-report-requested-capabilities.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-031-report-requested-capabilities.md) | `sha256:b10661586f0396ac9f204f17f11042ed662f57de2f8f682cda3f30cfe9f666dd` |
| [FR-032-preserve-language-evolution.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-032-preserve-language-evolution.md) | `sha256:49b5de82e2ee9c0e97995dce6fba7ca22013a1343382c69c4a9be01f761474e4` |
| [FR-035-bind-ecosystem-subjects.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-035-bind-ecosystem-subjects.md) | `sha256:e9105e44dc93451f74be69f510f5a77483e2f1e145fc95ab626a1eaaa10381ad` |
| [FR-036-retain-lexical-source-locations.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-036-retain-lexical-source-locations.md) | `sha256:227cbe0215a391fcb91535a9cdbeadb979ee00038c37d544b711cb4b0ad853ef` |
| [FR-037-parse-shared-native-expressions.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-037-parse-shared-native-expressions.md) | `sha256:0dcf6e73097d8fe0ee45d864e70731b2fcda6379eb058af331b975302b2547b2` |
| [FR-038-resolve-predicate-scopes.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-038-resolve-predicate-scopes.md) | `sha256:fec72cfaa7fc5ea767dfc7ecb966dc6ecf6011a0794f455b6d6767a624d1695a` |
| [FR-040-admit-explicit-state-extensions.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-040-admit-explicit-state-extensions.md) | `sha256:857f11a85f3c210b2dfc71ead4005e5215ce38c4dd2be4dc081a51849ad7d15f` |
| [FR-047-emit-typed-located-causes.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-047-emit-typed-located-causes.md) | `sha256:794d0a76cec53bd113dddfb750b1a8db4c8bc5677be73999582d6861dadb9805` |
| [NFR-010-bound-composed-processing.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/non-functional/NFR-010-bound-composed-processing.md) | `sha256:d9419cc39ac28a1562dea3ef74bdf7b092b5b4b32cf172f80d95976ae8baaf16` |
