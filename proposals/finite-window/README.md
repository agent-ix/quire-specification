# Timestamped-event finite-window vectors

`finite-window-vectors.json` publishes the vectors `FWV-01` to `FWV-75` for the
timestamped-event finite-window profile (FR-091-AC-10 to FR-091-AC-26,
FR-094-AC-11, TC-980). They are the QSL FR-501 rows 1 to 22, 27, 28, 31 to 33, 35 to 37, 39 and 40 (TC-1295) as inputs and
expected outcomes: positions with stamps and valuations, the progress watermark
`W`, the completeness assertion, the lower-boundary kind and the expected
`true`, `false`, `pending` or `indeterminate` outcome with its position, deadline
and reason. The counter-tick rows reuse `../checked-package-v2/tick-cover-vectors.json`.
The evaluator is QSL's; QSpec's test applies the stated rule to the vectors.
