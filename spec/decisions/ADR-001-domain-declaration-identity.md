---
id: ADR-001
title: "Declaration identity in a lifted domain package"
type: ADR
status: proposed
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: depends_on
  - target: ix://agent-ix/filament-core-data/FR-095
    type: depends_on
  - target: ix://agent-ix/filament-core-data/FR-093
    type: depends_on
---
# ADR-001: Declaration identity in a lifted domain package

## Status

**Open question for the architect.** Two normative specifications fix the
identity of the same IR node differently, and an implementation can satisfy
only one of them.

## Context

Quire admits a domain model from a lifted domain package and names each
declaration by that identity in clause text, counterexamples and result
records. Identity therefore decides whether editing a document changes a
declaration key, and whether a Quire clause can name a declaration at all.

[FR-154](../functional/type-model/FR-154-admit-domain-package-model.md) states
that a declaration's IR node identity is its artifact id
(`ix://<package identity>/<artifact id>`, member identity being the owner's
identity, `/` and the member name), that the artifact title is presentation
only, and that object ids matching FR-154's object-id rule are admitted
verbatim. Source-language qualified-name spelling is governed by the language
grammar, separately from identity intake.

`ix://agent-ix/filament-core-data/FR-095` mints a type node as
`ix://<org>/<name>/type/<slug(displayName)>` and a member as
`field/<Name>-<field>`, closing the grammar to a fixed slot-segment set, and
`ix://agent-ix/filament-core-data/FR-093` sources `displayName` from the
artifact title when the frontmatter name is not an identifier. Slugging maps
`Config_Version` to `Config-Version`.

Three consequences follow from the difference. A retitled artifact keeps every
declaration key under FR-154 and re-mints under FR-095. A member key is
`<owner>/<member>` under FR-154 and `field/<Name>-<field>` under FR-095. An
artifact id carrying an underscore or hyphen is admitted verbatim under
FR-154; the producer must preserve that identity instead of slugging it.

## Question

Which specification fixes declaration identity for a lifted domain package,
and what identity grammar do both sides then state?

Answering it fixes, in one place: whether the artifact id or the title is the
identity source; whether the identity carries a slot segment; the member
identity form; and the admitted character set for an identity tail.

## Consequences

Until the question is answered, `FR-154` and `filament-core-data/FR-095` bind
different keys for the same declaration, and a clause naming a declaration by
one grammar does not resolve under the other.
