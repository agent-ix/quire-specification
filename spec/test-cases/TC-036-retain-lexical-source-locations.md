---
id: TC-036
title: "Preserve original source locations through lexical recognition"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-036
    type: verifies
---
## Description

Planned controls for [FR-036](../functional/foundation/FR-036-retain-lexical-source-locations.md).
These expected cases have not been implemented or executed.

## Test Procedure

Tokenize equivalent LF/CRLF units containing a multibyte JSON string, an escaped quote, a comment marker inside a string, and an error after that text. Exercise a longer identifier beginning with a keyword. Independently supply invalid UTF-8, BOM, raw NUL, bare CR, unpaired surrogate, leading zero, and unterminated string. Test exact source/token limits and one over. Compare old/new edition classification of a newly reserved word.

## Expected Results

Reported spans select the intended original byte slices and identities remain byte-exact. Lexical defects refuse, not repair; resource controls produce no accepted partial unit. Complete-word matching and edition selection prevent accidental keyword or identifier reinterpretation.
