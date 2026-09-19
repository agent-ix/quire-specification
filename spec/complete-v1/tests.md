---
id: TM-009
title: "Complete V1 atomic capability and requirement matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
---

# Complete V1 atomic capability and requirement matrix

This matrix is normative traceability for the complete 185-row inventory. A
planned row is required work, not a deferred capability or implementation claim.

## Specification action classification

Each row's specification action records what the complete-V1 authoring pass did
with the owning requirement.

- `Reuse`: an existing requirement already has the exact behavior and
  acceptance criteria.
- `Amend`: the right owner exists and its behavior is extended to cover the
  full atomic obligation.
- `Split`: the existing owner combined independently testable behavior, which
  is separated into its own criteria.
- `Resolve`: normative statements disagreed, or an accepted architecture
  excluded a capability the master inventory requires, and the disagreement is
  decided here.
- `New`: behavior and acceptance criteria are authored for the row.

Over the 185 rows the delivered totals are 97 Reuse, 40 Amend, 28 New, 19
Resolve and 1 Split.

V1-TYPE-024 is a Resolve row because extending reachability over inheritance
and conformance required deciding what "the same type" means for traversal.
FR-043 fixes it as conformance to the static edge type under
`quire.model.complete/v1`, with identity keeping each object's most-specific
type.

## Atomic Capability Coverage

| Capability | Primary requirement | Test cases | Specification action | Coverage Status |
| --- | --- | --- | --- | --- |
| V1-SRC-001 | FR-001 | TC-231 | Reuse | planned |
| V1-SRC-002 | FR-001 | TC-231 | Reuse | planned |
| V1-SRC-003 | FR-131 | TC-180, TC-231 | Amend | planned |
| V1-SRC-004 | FR-131 | TC-180, TC-231 | Amend | planned |
| V1-SRC-005 | FR-003 | TC-231 | Reuse | planned |
| V1-SRC-006 | FR-003 | TC-231 | Reuse | planned |
| V1-SRC-007 | FR-134 | TC-184, TC-231 | Resolve | planned |
| V1-SRC-008 | FR-002 | TC-231 | Reuse | planned |
| V1-SRC-009 | FR-002 | TC-231 | Reuse | planned |
| V1-SRC-010 | FR-132 | TC-182, TC-231 | Amend | planned |
| V1-SRC-011 | FR-132 | TC-182, TC-231 | Split | planned |
| V1-SRC-012 | FR-016 | TC-231 | Reuse | planned |
| V1-SRC-013 | FR-132 | TC-182, TC-231 | Amend | planned |
| V1-SRC-014 | FR-133 | TC-183, TC-231 | Amend | planned |
| V1-SRC-015 | FR-001 | TC-231 | Reuse | planned |
| V1-TYPE-001 | FR-002 | TC-231 | Reuse | planned |
| V1-TYPE-002 | FR-005 | TC-231 | Reuse | planned |
| V1-TYPE-003 | FR-007 | TC-231 | Reuse | planned |
| V1-TYPE-004 | FR-039 | TC-231 | Reuse | planned |
| V1-TYPE-005 | FR-140 | TC-185, TC-231 | New | planned |
| V1-TYPE-006 | FR-141 | TC-186, TC-231 | New | planned |
| V1-TYPE-007 | FR-141 | TC-186, TC-231 | New | planned |
| V1-TYPE-008 | FR-142 | TC-187, TC-231 | Amend | planned |
| V1-TYPE-009 | FR-142 | TC-187, TC-231 | New | planned |
| V1-TYPE-010 | FR-019 | TC-231 | Reuse | planned |
| V1-TYPE-011 | FR-019 | TC-231 | Reuse | planned |
| V1-TYPE-012 | FR-143 | TC-188, TC-231 | Amend | planned |
| V1-TYPE-013 | FR-149 | TC-194, TC-231 | Resolve | planned |
| V1-TYPE-014 | FR-008 | TC-231 | Reuse | planned |
| V1-TYPE-015 | FR-144 | TC-189, TC-231 | Amend | planned |
| V1-TYPE-016 | FR-144 | TC-189, TC-231 | Amend | planned |
| V1-TYPE-017 | FR-144 | TC-189, TC-231 | Amend | planned |
| V1-TYPE-018 | FR-145 | TC-190, TC-231 | Amend | planned |
| V1-TYPE-019 | FR-019 | TC-231 | Reuse | planned |
| V1-TYPE-020 | FR-143 | TC-188, TC-231 | Resolve | planned |
| V1-TYPE-021 | FR-011 | TC-231 | Reuse | planned |
| V1-TYPE-022 | FR-009 | TC-231 | Reuse | planned |
| V1-TYPE-023 | FR-153 | TC-198, TC-231 | Amend | planned |
| V1-TYPE-024 | FR-043 | TC-043, TC-231 | Resolve | planned |
| V1-TYPE-025 | FR-150 | TC-195, TC-231 | New | planned |
| V1-TYPE-026 | FR-151 | TC-196, TC-231 | Resolve | planned |
| V1-TYPE-027 | FR-151 | TC-196, TC-231 | New | planned |
| V1-TYPE-028 | FR-151 | TC-196, TC-231 | New | planned |
| V1-TYPE-029 | FR-152 | TC-197, TC-231 | Amend | planned |
| V1-TYPE-030 | FR-019 | TC-231 | Reuse | planned |
| V1-TYPE-031 | FR-147 | TC-192, TC-231 | Resolve | planned |
| V1-EXPR-001 | FR-006 | TC-231 | Reuse | planned |
| V1-EXPR-002 | FR-033 | TC-231 | Reuse | planned |
| V1-EXPR-003 | FR-146 | TC-191, TC-231 | Amend | planned |
| V1-EXPR-004 | FR-146 | TC-191, TC-231 | Resolve | planned |
| V1-EXPR-005 | FR-007 | TC-231 | Reuse | planned |
| V1-EXPR-006 | FR-147 | TC-192, TC-231 | Resolve | planned |
| V1-EXPR-007 | FR-039 | TC-231 | Reuse | planned |
| V1-EXPR-008 | FR-148 | TC-193, TC-231 | Resolve | planned |
| V1-EXPR-009 | FR-148 | TC-193, TC-231 | New | planned |
| V1-EXPR-010 | FR-041 | TC-231 | Reuse | planned |
| V1-EXPR-011 | FR-008 | TC-231 | Reuse | planned |
| V1-EXPR-012 | FR-145 | TC-190, TC-231 | Amend | planned |
| V1-EXPR-013 | FR-145 | TC-190, TC-231 | Resolve | planned |
| V1-EXPR-014 | FR-145 | TC-190, TC-231 | Resolve | planned |
| V1-EXPR-015 | FR-145 | TC-190, TC-231 | Amend | planned |
| V1-EXPR-016 | FR-149 | TC-194, TC-231 | Amend | planned |
| V1-EXPR-017 | FR-009 | TC-231 | Reuse | planned |
| V1-EXPR-018 | FR-012 | TC-231 | Reuse | planned |
| V1-EXPR-019 | FR-012 | TC-231 | Reuse | planned |
| V1-EXPR-020 | FR-012 | TC-231 | Reuse | planned |
| V1-EXPR-021 | FR-006 | TC-231 | Reuse | planned |
| V1-EXPR-022 | FR-013 | TC-231 | Reuse | planned |
| V1-EXPR-023 | FR-271 | TC-047, TC-231 | Amend | planned |
| V1-EXPR-024 | FR-019 | TC-231 | Reuse | planned |
| V1-EXPR-025 | FR-019 | TC-231 | Reuse | planned |
| V1-EXPR-026 | FR-007 | TC-231 | Reuse | planned |
| V1-TEMP-001 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-002 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-003 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-004 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-005 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-006 | FR-092 | TC-231 | Reuse | planned |
| V1-TEMP-007 | FR-090 | TC-231 | Reuse | planned |
| V1-TEMP-008 | FR-090 | TC-231 | Reuse | planned |
| V1-TEMP-009 | FR-090 | TC-231 | Reuse | planned |
| V1-TEMP-010 | FR-048 | TC-231 | Reuse | planned |
| V1-TEMP-011 | FR-094 | TC-231 | Reuse | planned |
| V1-TEMP-012 | FR-091 | TC-231 | Reuse | planned |
| V1-TEMP-013 | FR-093 | TC-231 | Reuse | planned |
| V1-TEMP-014 | FR-094 | TC-231 | Reuse | planned |
| V1-TEMP-015 | FR-112 | TC-231 | Reuse | planned |
| V1-TEMP-016 | FR-160 | TC-199, TC-231 | Amend | planned |
| V1-TEMP-017 | FR-160 | TC-199, TC-231 | Resolve | planned |
| V1-TEMP-018 | FR-160 | TC-199, TC-231 | Resolve | planned |
| V1-TEMP-019 | FR-095 | TC-231 | Reuse | planned |
| V1-TEMP-020 | FR-034 | TC-231 | Reuse | planned |
| V1-TEMP-021 | FR-091 | TC-231 | Reuse | planned |
| V1-TEMP-022 | FR-090 | TC-231 | Reuse | planned |
| V1-TEMP-023 | FR-161 | TC-200, TC-231 | New | planned |
| V1-PROT-001 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-002 | FR-035 | TC-231 | Reuse | planned |
| V1-PROT-003 | FR-035 | TC-231 | Reuse | planned |
| V1-PROT-004 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-005 | FR-170 | TC-201, TC-231 | Amend | planned |
| V1-PROT-006 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-007 | FR-035 | TC-231 | Reuse | planned |
| V1-PROT-008 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-009 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-010 | FR-171 | TC-202, TC-231 | Amend | planned |
| V1-PROT-011 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-012 | FR-171 | TC-202, TC-231 | Resolve | planned |
| V1-PROT-013 | FR-170 | TC-201, TC-231 | Amend | planned |
| V1-PROT-014 | FR-171 | TC-202, TC-231 | Resolve | planned |
| V1-PROT-015 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-016 | FR-172 | TC-203, TC-231 | Amend | planned |
| V1-PROT-017 | FR-054 | TC-231 | Reuse | planned |
| V1-PROT-018 | FR-172 | TC-203, TC-231 | Amend | planned |
| V1-PROT-019 | FR-172 | TC-203, TC-231 | Amend | planned |
| V1-PROT-020 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-021 | FR-049 | TC-231 | Reuse | planned |
| V1-PROT-022 | FR-057 | TC-231 | Reuse | planned |
| V1-PROT-023 | FR-173 | TC-204, TC-231 | Amend | planned |
| V1-PROT-024 | FR-051 | TC-231 | Reuse | planned |
| V1-PROT-025 | FR-059 | TC-231 | Reuse | planned |
| V1-PROT-026 | FR-059 | TC-231 | Reuse | planned |
| V1-PROT-027 | FR-174 | TC-205, TC-231 | Resolve | planned |
| V1-PROT-028 | FR-175 | TC-206, TC-231 | Amend | planned |
| V1-PROT-029 | FR-176 | TC-207, TC-231 | New | planned |
| V1-PROT-030 | FR-177 | TC-208, TC-231 | Amend | planned |
| V1-PROT-031 | FR-049 | TC-231 | Reuse | planned |
| V1-RUN-001 | FR-005 | TC-231 | Reuse | planned |
| V1-RUN-002 | FR-090 | TC-231 | Reuse | planned |
| V1-RUN-003 | FR-050 | TC-231 | Reuse | planned |
| V1-RUN-004 | FR-094 | TC-231 | Reuse | planned |
| V1-RUN-005 | FR-031 | TC-231 | Reuse | planned |
| V1-RUN-006 | FR-061 | TC-231 | Reuse | planned |
| V1-RUN-007 | FR-011 | TC-231 | Reuse | planned |
| V1-RUN-008 | FR-180 | TC-209, TC-231 | Amend | planned |
| V1-RUN-009 | FR-035 | TC-231 | Reuse | planned |
| V1-RUN-010 | FR-005 | TC-231 | Reuse | planned |
| V1-RUN-011 | FR-180 | TC-209, TC-231 | Amend | planned |
| V1-RUN-012 | FR-180 | TC-209, TC-231 | Amend | planned |
| V1-RUN-013 | FR-181 | TC-210, TC-231 | New | planned |
| V1-RUN-014 | FR-190 | TC-212, TC-231 | New | planned |
| V1-RUN-015 | FR-191 | TC-213, TC-231 | New | planned |
| V1-RUN-016 | FR-191 | TC-213, TC-231 | Resolve | planned |
| V1-RUN-017 | FR-192 | TC-214, TC-231 | New | planned |
| V1-RUN-018 | FR-193 | TC-215, TC-231 | New | planned |
| V1-RUN-019 | FR-194 | TC-216, TC-231 | Resolve | planned |
| V1-RUN-020 | FR-182 | TC-211, TC-231 | Amend | planned |
| V1-BACK-001 | FR-031 | TC-231 | Reuse | planned |
| V1-BACK-002 | FR-031 | TC-231 | Reuse | planned |
| V1-BACK-003 | FR-030 | TC-231 | Reuse | planned |
| V1-BACK-004 | FR-195 | TC-217, TC-231 | Amend | planned |
| V1-BACK-005 | FR-196 | TC-218, TC-231 | Amend | planned |
| V1-BACK-006 | FR-197 | TC-219, TC-231 | Amend | planned |
| V1-BACK-007 | FR-120 | TC-231 | Reuse | planned |
| V1-BACK-008 | FR-120 | TC-231 | Reuse | planned |
| V1-BACK-009 | FR-120 | TC-231 | Reuse | planned |
| V1-BACK-010 | FR-125 | TC-231 | Reuse | planned |
| V1-BACK-011 | FR-121 | TC-231 | Reuse | planned |
| V1-BACK-012 | FR-019 | TC-231 | Reuse | planned |
| V1-BACK-013 | FR-052 | TC-231 | Reuse | planned |
| V1-BACK-014 | FR-019 | TC-231 | Reuse | planned |
| V1-BACK-015 | FR-352 | TC-255, TC-231 | New | planned |
| V1-BACK-016 | FR-352 | TC-255, TC-231 | New | planned |
| V1-BACK-017 | FR-290 | TC-060, TC-231 | Amend | planned |
| V1-BACK-018 | FR-292 | TC-060, TC-231 | Reuse | planned |
| V1-EVID-001 | FR-310 | TC-230, TC-231 | Amend | planned |
| V1-EVID-002 | FR-004 | TC-231 | Reuse | planned |
| V1-EVID-003 | FR-311 | TC-231 | Amend | planned |
| V1-EVID-004 | FR-020 | TC-231 | Reuse | planned |
| V1-EVID-005 | FR-016 | TC-231 | Reuse | planned |
| V1-EVID-006 | FR-312 | TC-232, TC-231 | Amend | planned |
| V1-EVID-007 | FR-351 | TC-254, TC-231 | New | planned |
| V1-EVID-008 | FR-351 | TC-254, TC-231 | New | planned |
| V1-EVID-009 | FR-351 | TC-254, TC-231 | New | planned |
| V1-TOOL-001 | FR-300 | TC-220, TC-231 | New | planned |
| V1-TOOL-002 | FR-301 | TC-221, TC-231 | New | planned |
| V1-TOOL-003 | FR-302 | TC-222, TC-231 | New | planned |
| V1-TOOL-004 | FR-303 | TC-223, TC-231 | New | planned |
| V1-TOOL-005 | FR-304 | TC-224, TC-231 | New | planned |
| V1-TOOL-006 | FR-305 | TC-225, TC-231 | Resolve | planned |
| V1-TOOL-007 | FR-306 | TC-226, TC-231 | New | planned |
| V1-TOOL-008 | FR-307 | TC-227, TC-231 | Amend | planned |
| V1-TOOL-009 | FR-308 | TC-228, TC-231 | New | planned |
| V1-TOOL-010 | FR-309 | TC-229, TC-231 | New | planned |
| V1-TOOL-011 | FR-177 | TC-208, TC-231 | Reuse | planned |
| V1-TOOL-012 | FR-177 | TC-208, TC-231 | Reuse | planned |

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
| --- | --- | --- | --- |
| FR-131 | FR-131-AC-1 | TC-180 | planned |
| FR-131 | FR-131-AC-2 | TC-180 | planned |
| FR-131 | FR-131-AC-3 | TC-180 | planned |
| FR-132 | FR-132-AC-1 | TC-182 | planned |
| FR-132 | FR-132-AC-2 | TC-182 | planned |
| FR-132 | FR-132-AC-3 | TC-182 | planned |
| FR-133 | FR-133-AC-1 | TC-183 | planned |
| FR-133 | FR-133-AC-2 | TC-183 | planned |
| FR-133 | FR-133-AC-3 | TC-183 | planned |
| FR-134 | FR-134-AC-1 | TC-184 | planned |
| FR-134 | FR-134-AC-2 | TC-184 | planned |
| FR-134 | FR-134-AC-3 | TC-184 | planned |
| FR-134 | FR-134-AC-4 | TC-184 | planned |
| FR-134 | FR-134-AC-5 | TC-184 | planned |
| FR-140 | FR-140-AC-1 | TC-185 | planned |
| FR-140 | FR-140-AC-2 | TC-185 | planned |
| FR-140 | FR-140-AC-3 | TC-185 | planned |
| FR-141 | FR-141-AC-1 | TC-186 | planned |
| FR-141 | FR-141-AC-2 | TC-186 | planned |
| FR-141 | FR-141-AC-3 | TC-186 | planned |
| FR-142 | FR-142-AC-1 | TC-187 | planned |
| FR-142 | FR-142-AC-2 | TC-187 | planned |
| FR-142 | FR-142-AC-3 | TC-187 | planned |
| FR-143 | FR-143-AC-1 | TC-188 | planned |
| FR-143 | FR-143-AC-2 | TC-188 | planned |
| FR-143 | FR-143-AC-3 | TC-188 | planned |
| FR-144 | FR-144-AC-1 | TC-189 | planned |
| FR-144 | FR-144-AC-2 | TC-189 | planned |
| FR-144 | FR-144-AC-3 | TC-189 | planned |
| FR-145 | FR-145-AC-1 | TC-190 | planned |
| FR-145 | FR-145-AC-2 | TC-190 | planned |
| FR-145 | FR-145-AC-3 | TC-190 | planned |
| FR-146 | FR-146-AC-1 | TC-191 | planned |
| FR-146 | FR-146-AC-2 | TC-191 | planned |
| FR-146 | FR-146-AC-3 | TC-191 | planned |
| FR-147 | FR-147-AC-1 | TC-192 | planned |
| FR-147 | FR-147-AC-2 | TC-192 | planned |
| FR-147 | FR-147-AC-3 | TC-192 | planned |
| FR-148 | FR-148-AC-1 | TC-193 | planned |
| FR-148 | FR-148-AC-2 | TC-193 | planned |
| FR-148 | FR-148-AC-3 | TC-193 | planned |
| FR-149 | FR-149-AC-1 | TC-194 | planned |
| FR-149 | FR-149-AC-2 | TC-194 | planned |
| FR-149 | FR-149-AC-3 | TC-194 | planned |
| FR-150 | FR-150-AC-1 | TC-195 | planned |
| FR-150 | FR-150-AC-2 | TC-195 | planned |
| FR-150 | FR-150-AC-3 | TC-195 | planned |
| FR-151 | FR-151-AC-1 | TC-196 | planned |
| FR-151 | FR-151-AC-2 | TC-196 | planned |
| FR-151 | FR-151-AC-3 | TC-196 | planned |
| FR-152 | FR-152-AC-1 | TC-197 | planned |
| FR-152 | FR-152-AC-2 | TC-197 | planned |
| FR-152 | FR-152-AC-3 | TC-197 | planned |
| FR-153 | FR-153-AC-1 | TC-198 | planned |
| FR-153 | FR-153-AC-2 | TC-198 | planned |
| FR-153 | FR-153-AC-3 | TC-198 | planned |
| FR-160 | FR-160-AC-1 | TC-199 | planned |
| FR-160 | FR-160-AC-2 | TC-199 | planned |
| FR-160 | FR-160-AC-3 | TC-199 | planned |
| FR-161 | FR-161-AC-1 | TC-200 | planned |
| FR-161 | FR-161-AC-2 | TC-200 | planned |
| FR-161 | FR-161-AC-3 | TC-200 | planned |
| FR-170 | FR-170-AC-1 | TC-201 | planned |
| FR-170 | FR-170-AC-2 | TC-201 | planned |
| FR-170 | FR-170-AC-3 | TC-201 | planned |
| FR-171 | FR-171-AC-1 | TC-202 | planned |
| FR-171 | FR-171-AC-2 | TC-202 | planned |
| FR-171 | FR-171-AC-3 | TC-202 | planned |
| FR-172 | FR-172-AC-1 | TC-203 | planned |
| FR-172 | FR-172-AC-2 | TC-203 | planned |
| FR-172 | FR-172-AC-3 | TC-203 | planned |
| FR-173 | FR-173-AC-1 | TC-204 | planned |
| FR-173 | FR-173-AC-2 | TC-204 | planned |
| FR-173 | FR-173-AC-3 | TC-204 | planned |
| FR-174 | FR-174-AC-1 | TC-205 | planned |
| FR-174 | FR-174-AC-2 | TC-205 | planned |
| FR-174 | FR-174-AC-3 | TC-205 | planned |
| FR-175 | FR-175-AC-1 | TC-206 | planned |
| FR-175 | FR-175-AC-2 | TC-206 | planned |
| FR-175 | FR-175-AC-3 | TC-206 | planned |
| FR-176 | FR-176-AC-1 | TC-207 | planned |
| FR-176 | FR-176-AC-2 | TC-207 | planned |
| FR-176 | FR-176-AC-3 | TC-207 | planned |
| FR-177 | FR-177-AC-1 | TC-208 | planned |
| FR-177 | FR-177-AC-2 | TC-208 | planned |
| FR-177 | FR-177-AC-3 | TC-208 | planned |
| FR-180 | FR-180-AC-1 | TC-209 | planned |
| FR-180 | FR-180-AC-2 | TC-209 | planned |
| FR-180 | FR-180-AC-3 | TC-209 | planned |
| FR-181 | FR-181-AC-1 | TC-210 | planned |
| FR-181 | FR-181-AC-2 | TC-210 | planned |
| FR-181 | FR-181-AC-3 | TC-210 | planned |
| FR-182 | FR-182-AC-1 | TC-211 | planned |
| FR-182 | FR-182-AC-2 | TC-211 | planned |
| FR-182 | FR-182-AC-3 | TC-211 | planned |
| FR-190 | FR-190-AC-1 | TC-212 | planned |
| FR-190 | FR-190-AC-2 | TC-212 | planned |
| FR-190 | FR-190-AC-3 | TC-212 | planned |
| FR-191 | FR-191-AC-1 | TC-213 | planned |
| FR-191 | FR-191-AC-2 | TC-213 | planned |
| FR-191 | FR-191-AC-3 | TC-213 | planned |
| FR-192 | FR-192-AC-1 | TC-214 | planned |
| FR-192 | FR-192-AC-2 | TC-214 | planned |
| FR-192 | FR-192-AC-3 | TC-214 | planned |
| FR-193 | FR-193-AC-1 | TC-215 | planned |
| FR-193 | FR-193-AC-2 | TC-215 | planned |
| FR-193 | FR-193-AC-3 | TC-215 | planned |
| FR-194 | FR-194-AC-1 | TC-216 | planned |
| FR-194 | FR-194-AC-2 | TC-216 | planned |
| FR-194 | FR-194-AC-3 | TC-216 | planned |
| FR-195 | FR-195-AC-1 | TC-217 | planned |
| FR-195 | FR-195-AC-2 | TC-217 | planned |
| FR-195 | FR-195-AC-3 | TC-217 | planned |
| FR-196 | FR-196-AC-1 | TC-218 | planned |
| FR-196 | FR-196-AC-2 | TC-218 | planned |
| FR-196 | FR-196-AC-3 | TC-218 | planned |
| FR-197 | FR-197-AC-1 | TC-219 | planned |
| FR-197 | FR-197-AC-2 | TC-219 | planned |
| FR-197 | FR-197-AC-3 | TC-219 | planned |
| FR-300 | FR-300-AC-1 | TC-220 | planned |
| FR-300 | FR-300-AC-2 | TC-220 | planned |
| FR-300 | FR-300-AC-3 | TC-220 | planned |
| FR-301 | FR-301-AC-1 | TC-221 | planned |
| FR-301 | FR-301-AC-2 | TC-221 | planned |
| FR-301 | FR-301-AC-3 | TC-221 | planned |
| FR-302 | FR-302-AC-1 | TC-222 | planned |
| FR-302 | FR-302-AC-2 | TC-222 | planned |
| FR-302 | FR-302-AC-3 | TC-222 | planned |
| FR-303 | FR-303-AC-1 | TC-223 | planned |
| FR-303 | FR-303-AC-2 | TC-223 | planned |
| FR-303 | FR-303-AC-3 | TC-223 | planned |
| FR-303 | FR-303-AC-4 | TC-223 | planned |
| FR-303 | FR-303-AC-5 | TC-223 | planned |
| FR-304 | FR-304-AC-1 | TC-224 | planned |
| FR-304 | FR-304-AC-2 | TC-224 | planned |
| FR-304 | FR-304-AC-3 | TC-224 | planned |
| FR-305 | FR-305-AC-1 | TC-225 | planned |
| FR-305 | FR-305-AC-2 | TC-225 | planned |
| FR-305 | FR-305-AC-3 | TC-225 | planned |
| FR-306 | FR-306-AC-1 | TC-226 | planned |
| FR-306 | FR-306-AC-2 | TC-226 | planned |
| FR-306 | FR-306-AC-3 | TC-226 | planned |
| FR-306 | FR-306-AC-4 | TC-226 | planned |
| FR-307 | FR-307-AC-1 | TC-227 | planned |
| FR-307 | FR-307-AC-2 | TC-227 | planned |
| FR-307 | FR-307-AC-3 | TC-227 | planned |
| FR-307 | FR-307-AC-4 | TC-227 | planned |
| FR-307 | FR-307-AC-5 | TC-227 | planned |
| FR-308 | FR-308-AC-1 | TC-228 | planned |
| FR-308 | FR-308-AC-2 | TC-228 | planned |
| FR-308 | FR-308-AC-3 | TC-228 | planned |
| FR-309 | FR-309-AC-1 | TC-229 | planned |
| FR-309 | FR-309-AC-2 | TC-229 | planned |
| FR-309 | FR-309-AC-3 | TC-229 | planned |
| FR-310 | FR-310-AC-1 | TC-230 | planned |
| FR-310 | FR-310-AC-2 | TC-230 | planned |
| FR-310 | FR-310-AC-3 | TC-230 | planned |
| FR-311 | FR-311-AC-1 | TC-231 | planned |
| FR-311 | FR-311-AC-2 | TC-231 | planned |
| FR-311 | FR-311-AC-3 | TC-231 | planned |
| FR-312 | FR-312-AC-1 | TC-232 | planned |
| FR-312 | FR-312-AC-2 | TC-232 | planned |
| FR-312 | FR-312-AC-3 | TC-232 | planned |

## Reconciled Existing Requirement Coverage

The rows below close acceptance-criterion coverage omitted from the historical
family matrices. A directly linked test case is used where one exists;
otherwise TC-231 owns the criterion in the complete-V1 qualification corpus.

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
| --- | --- | --- | --- |
| FR-002 | FR-002-AC-1 | TC-231 | planned |
| FR-002 | FR-002-AC-2 | TC-231 | planned |
| FR-002 | FR-002-AC-3 | TC-231 | planned |
| FR-002 | FR-002-AC-4 | TC-231 | planned |
| FR-002 | FR-002-AC-5 | TC-231 | planned |
| FR-003 | FR-003-AC-1 | TC-231 | planned |
| FR-003 | FR-003-AC-2 | TC-231 | planned |
| FR-003 | FR-003-AC-3 | TC-231 | planned |
| FR-003 | FR-003-AC-4 | TC-231 | planned |
| FR-004 | FR-004-AC-1 | TC-231 | planned |
| FR-004 | FR-004-AC-2 | TC-231 | planned |
| FR-004 | FR-004-AC-3 | TC-231 | planned |
| FR-004 | FR-004-AC-4 | TC-231 | planned |
| FR-005 | FR-005-AC-1 | TC-231 | planned |
| FR-005 | FR-005-AC-2 | TC-231 | planned |
| FR-005 | FR-005-AC-3 | TC-231 | planned |
| FR-005 | FR-005-AC-4 | TC-231 | planned |
| FR-006 | FR-006-AC-1 | TC-231 | planned |
| FR-006 | FR-006-AC-2 | TC-231 | planned |
| FR-006 | FR-006-AC-3 | TC-231 | planned |
| FR-006 | FR-006-AC-4 | TC-231 | planned |
| FR-006 | FR-006-AC-5 | TC-231 | planned |
| FR-006 | FR-006-AC-6 | TC-231 | planned |
| FR-006 | FR-006-AC-7 | TC-231 | planned |
| FR-006 | FR-006-AC-8 | TC-231 | planned |
| FR-006 | FR-006-AC-9 | TC-231 | planned |
| FR-007 | FR-007-AC-1 | TC-231 | planned |
| FR-007 | FR-007-AC-2 | TC-231 | planned |
| FR-007 | FR-007-AC-3 | TC-231 | planned |
| FR-007 | FR-007-AC-4 | TC-231 | planned |
| FR-007 | FR-007-AC-5 | TC-231 | planned |
| FR-007 | FR-007-AC-6 | TC-231 | planned |
| FR-007 | FR-007-AC-7 | TC-192 | planned |
| FR-008 | FR-008-AC-1 | TC-231 | planned |
| FR-008 | FR-008-AC-2 | TC-231 | planned |
| FR-008 | FR-008-AC-3 | TC-231 | planned |
| FR-008 | FR-008-AC-4 | TC-231 | planned |
| FR-008 | FR-008-AC-5 | TC-231 | planned |
| FR-008 | FR-008-AC-6 | TC-189, TC-190, TC-231 | planned |
| FR-009 | FR-009-AC-1 | TC-231 | planned |
| FR-009 | FR-009-AC-2 | TC-231 | planned |
| FR-009 | FR-009-AC-3 | TC-231 | planned |
| FR-010 | FR-010-AC-1 | TC-231 | planned |
| FR-010 | FR-010-AC-2 | TC-231 | planned |
| FR-010 | FR-010-AC-3 | TC-231 | planned |
| FR-010 | FR-010-AC-4 | TC-231 | planned |
| FR-011 | FR-011-AC-1 | TC-231 | planned |
| FR-011 | FR-011-AC-2 | TC-231 | planned |
| FR-011 | FR-011-AC-3 | TC-231 | planned |
| FR-011 | FR-011-AC-4 | TC-231 | planned |
| FR-011 | FR-011-AC-5 | TC-231 | planned |
| FR-012 | FR-012-AC-1 | TC-231 | planned |
| FR-012 | FR-012-AC-2 | TC-231 | planned |
| FR-012 | FR-012-AC-3 | TC-231 | planned |
| FR-012 | FR-012-AC-4 | TC-231 | planned |
| FR-012 | FR-012-AC-5 | TC-231 | planned |
| FR-012 | FR-012-AC-6 | TC-231 | planned |
| FR-012 | FR-012-AC-7 | TC-231 | planned |
| FR-012 | FR-012-AC-8 | TC-231 | planned |
| FR-013 | FR-013-AC-1 | TC-231 | planned |
| FR-013 | FR-013-AC-2 | TC-231 | planned |
| FR-013 | FR-013-AC-3 | TC-231 | planned |
| FR-013 | FR-013-AC-4 | TC-231 | planned |
| FR-013 | FR-013-AC-5 | TC-236 | planned |
| FR-013 | FR-013-AC-6 | TC-236 | planned |
| FR-014 | FR-014-AC-1 | TC-231 | planned |
| FR-014 | FR-014-AC-2 | TC-231 | planned |
| FR-014 | FR-014-AC-3 | TC-231 | planned |
| FR-014 | FR-014-AC-4 | TC-231 | planned |
| FR-014 | FR-014-AC-5 | TC-231 | planned |
| FR-015 | FR-015-AC-1 | TC-231 | planned |
| FR-015 | FR-015-AC-2 | TC-231 | planned |
| FR-015 | FR-015-AC-3 | TC-231 | planned |
| FR-015 | FR-015-AC-4 | TC-231 | planned |
| FR-016 | FR-016-AC-1 | TC-231 | planned |
| FR-016 | FR-016-AC-2 | TC-231 | planned |
| FR-016 | FR-016-AC-3 | TC-231 | planned |
| FR-016 | FR-016-AC-4 | TC-231 | planned |
| FR-019 | FR-019-AC-7 | TC-006 | planned |
| FR-020 | FR-020-AC-11 | TC-020 | planned |
| FR-091 | FR-091-AC-7 | TC-111 | planned |
| FR-091 | FR-091-AC-8 | TC-200 | planned |
| FR-093 | FR-093-AC-6 | TC-114 | planned |
| FR-120 | FR-120-AC-2 | TC-150 | planned |
| FR-120 | FR-120-AC-3 | TC-150 | planned |
| FR-120 | FR-120-AC-4 | TC-150 | planned |
| FR-121 | FR-121-AC-2 | TC-150 | planned |
| FR-121 | FR-121-AC-3 | TC-150 | planned |
| FR-121 | FR-121-AC-4 | TC-150 | planned |
| FR-121 | FR-121-AC-5 | TC-150 | planned |
| FR-122 | FR-122-AC-2 | TC-151 | planned |
| FR-122 | FR-122-AC-3 | TC-151 | planned |
| FR-122 | FR-122-AC-4 | TC-151 | planned |
| FR-122 | FR-122-AC-5 | TC-151 | planned |
| FR-123 | FR-123-AC-2 | TC-152 | planned |
| FR-123 | FR-123-AC-3 | TC-152 | planned |
| FR-123 | FR-123-AC-4 | TC-152 | planned |
| FR-123 | FR-123-AC-5 | TC-152 | planned |
| FR-124 | FR-124-AC-2 | TC-153 | planned |
| FR-124 | FR-124-AC-3 | TC-153 | planned |
| FR-124 | FR-124-AC-4 | TC-153 | planned |
| FR-124 | FR-124-AC-5 | TC-153 | planned |
| FR-125 | FR-125-AC-2 | TC-154 | planned |
| FR-125 | FR-125-AC-3 | TC-154 | planned |
| FR-125 | FR-125-AC-4 | TC-154 | planned |
| FR-125 | FR-125-AC-5 | TC-154 | planned |
| FR-200 | FR-200-AC-1 | TC-231 | planned |
| FR-200 | FR-200-AC-2 | TC-231 | planned |
| FR-200 | FR-200-AC-3 | TC-231 | planned |
| FR-200 | FR-200-AC-4 | TC-231 | planned |
| FR-200 | FR-200-AC-5 | TC-231 | planned |
| FR-200 | FR-200-AC-6 | TC-231 | planned |
| FR-201 | FR-201-AC-1 | TC-231 | planned |
| FR-201 | FR-201-AC-2 | TC-231 | planned |
| FR-201 | FR-201-AC-3 | TC-231 | planned |
| FR-201 | FR-201-AC-4 | TC-231 | planned |
| FR-201 | FR-201-AC-6 | TC-233 | planned |
| FR-202 | FR-202-AC-1 | TC-231 | planned |
| FR-202 | FR-202-AC-2 | TC-231 | planned |
| FR-202 | FR-202-AC-3 | TC-231 | planned |
| FR-202 | FR-202-AC-4 | TC-231 | planned |
| FR-203 | FR-203-AC-1 | TC-231 | planned |
| FR-203 | FR-203-AC-2 | TC-231 | planned |
| FR-203 | FR-203-AC-3 | TC-231 | planned |
| FR-203 | FR-203-AC-4 | TC-231 | planned |
| FR-204 | FR-204-AC-1 | TC-231 | planned |
| FR-204 | FR-204-AC-2 | TC-231 | planned |
| FR-204 | FR-204-AC-3 | TC-231 | planned |
| FR-204 | FR-204-AC-4 | TC-231 | planned |
| FR-205 | FR-205-AC-1 | TC-231 | planned |
| FR-205 | FR-205-AC-2 | TC-231 | planned |
| FR-205 | FR-205-AC-3 | TC-231 | planned |
| FR-205 | FR-205-AC-4 | TC-231 | planned |
| FR-206 | FR-206-AC-1 | TC-231 | planned |
| FR-206 | FR-206-AC-2 | TC-231 | planned |
| FR-206 | FR-206-AC-3 | TC-231 | planned |
| FR-206 | FR-206-AC-4 | TC-231 | planned |
| FR-207 | FR-207-AC-1 | TC-231 | planned |
| FR-207 | FR-207-AC-2 | TC-231 | planned |
| FR-207 | FR-207-AC-3 | TC-231 | planned |
| FR-207 | FR-207-AC-4 | TC-231 | planned |
| FR-207 | FR-207-AC-5 | TC-231 | planned |
| FR-207 | FR-207-AC-6 | TC-231 | planned |
| FR-207 | FR-207-AC-7 | TC-231 | planned |
| FR-207 | FR-207-AC-8 | TC-235 | planned |
| FR-210 | FR-210-AC-1 | TC-050 | planned |
| FR-210 | FR-210-AC-2 | TC-050 | planned |
| FR-210 | FR-210-AC-3 | TC-050 | planned |
| FR-211 | FR-211-AC-1 | TC-050 | planned |
| FR-211 | FR-211-AC-2 | TC-050 | planned |
| FR-211 | FR-211-AC-4 | TC-050 | planned |
| FR-212 | FR-212-AC-1 | TC-050 | planned |
| FR-212 | FR-212-AC-2 | TC-050 | planned |
| FR-212 | FR-212-AC-3 | TC-050 | planned |
| FR-213 | FR-213-AC-1 | TC-050 | planned |
| FR-213 | FR-213-AC-2 | TC-050 | planned |
| FR-213 | FR-213-AC-3 | TC-050 | planned |
| FR-214 | FR-214-AC-1 | TC-050 | planned |
| FR-214 | FR-214-AC-2 | TC-050 | planned |
| FR-214 | FR-214-AC-3 | TC-050 | planned |
| FR-215 | FR-215-AC-1 | TC-050 | planned |
| FR-215 | FR-215-AC-2 | TC-050 | planned |
| FR-215 | FR-215-AC-3 | TC-050 | planned |
| FR-215 | FR-215-AC-4 | TC-050 | planned |
| FR-221 | FR-221-AC-1 | TC-171 | planned |
| FR-221 | FR-221-AC-2 | TC-171 | planned |
| FR-221 | FR-221-AC-3 | TC-171 | planned |
| FR-222 | FR-222-AC-1 | TC-231 | planned |
| FR-222 | FR-222-AC-2 | TC-231 | planned |
| FR-222 | FR-222-AC-3 | TC-231 | planned |
| FR-223 | FR-223-AC-1 | TC-231 | planned |
| FR-223 | FR-223-AC-2 | TC-231 | planned |
| FR-223 | FR-223-AC-3 | TC-231 | planned |
| FR-224 | FR-224-AC-1 | TC-051 | planned |
| FR-224 | FR-224-AC-2 | TC-051 | planned |
| FR-225 | FR-225-AC-1 | TC-051 | planned |
| FR-225 | FR-225-AC-2 | TC-051 | planned |
| FR-225 | FR-225-AC-3 | TC-051 | planned |
| FR-226 | FR-226-AC-1 | TC-051 | planned |
| FR-226 | FR-226-AC-2 | TC-051 | planned |
| FR-227 | FR-227-AC-1 | TC-231 | planned |
| FR-227 | FR-227-AC-2 | TC-231 | planned |
| FR-227 | FR-227-AC-3 | TC-231 | planned |
| FR-227 | FR-227-AC-4 | TC-231 | planned |
| FR-228 | FR-228-AC-1 | TC-231 | planned |
| FR-228 | FR-228-AC-2 | TC-231 | planned |
| FR-228 | FR-228-AC-3 | TC-231 | planned |
| FR-228 | FR-228-AC-4 | TC-231 | planned |
| FR-230 | FR-230-AC-1 | TC-231 | planned |
| FR-230 | FR-230-AC-2 | TC-231 | planned |
| FR-230 | FR-230-AC-3 | TC-231 | planned |
| FR-230 | FR-230-AC-4 | TC-231 | planned |
| FR-231 | FR-231-AC-1 | TC-231 | planned |
| FR-231 | FR-231-AC-2 | TC-231 | planned |
| FR-231 | FR-231-AC-3 | TC-231 | planned |
| FR-231 | FR-231-AC-4 | TC-231 | planned |
| FR-232 | FR-232-AC-1 | TC-231 | planned |
| FR-232 | FR-232-AC-2 | TC-231 | planned |
| FR-232 | FR-232-AC-3 | TC-231 | planned |
| FR-232 | FR-232-AC-4 | TC-231 | planned |
| FR-234 | FR-234-AC-1 | TC-231 | planned |
| FR-234 | FR-234-AC-2 | TC-231 | planned |
| FR-234 | FR-234-AC-3 | TC-231 | planned |
| FR-234 | FR-234-AC-4 | TC-231 | planned |
| FR-235 | FR-235-AC-1 | TC-058 | planned |
| FR-235 | FR-235-AC-2 | TC-058 | planned |
| FR-235 | FR-235-AC-3 | TC-058 | planned |
| FR-235 | FR-235-AC-4 | TC-058 | planned |
| FR-238 | FR-238-AC-1 | TC-057 | planned |
| FR-238 | FR-238-AC-2 | TC-057 | planned |
| FR-238 | FR-238-AC-3 | TC-057 | planned |
| FR-238 | FR-238-AC-4 | TC-057 | planned |
| FR-241 | FR-241-AC-1 | TC-231 | planned |
| FR-241 | FR-241-AC-2 | TC-231 | planned |
| FR-241 | FR-241-AC-3 | TC-231 | planned |
| FR-241 | FR-241-AC-4 | TC-231 | planned |
| FR-244 | FR-244-AC-1 | TC-231 | planned |
| FR-244 | FR-244-AC-2 | TC-231 | planned |
| FR-244 | FR-244-AC-3 | TC-231 | planned |
| FR-244 | FR-244-AC-4 | TC-231 | planned |
| FR-245 | FR-245-AC-1 | TC-231 | planned |
| FR-245 | FR-245-AC-2 | TC-231 | planned |
| FR-245 | FR-245-AC-3 | TC-231 | planned |
| FR-245 | FR-245-AC-4 | TC-231 | planned |
| FR-246 | FR-246-AC-1 | TC-231 | planned |
| FR-246 | FR-246-AC-2 | TC-231 | planned |
| FR-246 | FR-246-AC-3 | TC-231 | planned |
| FR-246 | FR-246-AC-4 | TC-231 | planned |
| FR-247 | FR-247-AC-1 | TC-231 | planned |
| FR-247 | FR-247-AC-2 | TC-231 | planned |
| FR-247 | FR-247-AC-3 | TC-231 | planned |
| FR-247 | FR-247-AC-4 | TC-231 | planned |
| FR-247 | FR-247-AC-5 | TC-231 | planned |
| FR-248 | FR-248-AC-1 | TC-231 | planned |
| FR-248 | FR-248-AC-2 | TC-231 | planned |
| FR-248 | FR-248-AC-3 | TC-231 | planned |
| FR-248 | FR-248-AC-4 | TC-231 | planned |
| FR-248 | FR-248-AC-5 | TC-231 | planned |
| FR-248 | FR-248-AC-6 | TC-231 | planned |
| FR-250 | FR-250-AC-1 | TC-231 | planned |
| FR-250 | FR-250-AC-2 | TC-231 | planned |
| FR-250 | FR-250-AC-3 | TC-231 | planned |
| FR-250 | FR-250-AC-4 | TC-231 | planned |
| FR-250 | FR-250-AC-5 | TC-231 | planned |
| FR-251 | FR-251-AC-1 | TC-231 | planned |
| FR-251 | FR-251-AC-2 | TC-231 | planned |
| FR-251 | FR-251-AC-3 | TC-231 | planned |
| FR-251 | FR-251-AC-4 | TC-231 | planned |
| FR-252 | FR-252-AC-1 | TC-231 | planned |
| FR-252 | FR-252-AC-2 | TC-231 | planned |
| FR-252 | FR-252-AC-3 | TC-231 | planned |
| FR-252 | FR-252-AC-4 | TC-231 | planned |
| FR-252 | FR-252-AC-5 | TC-231 | planned |
| FR-253 | FR-253-AC-1 | TC-231 | planned |
| FR-253 | FR-253-AC-2 | TC-231 | planned |
| FR-253 | FR-253-AC-3 | TC-231 | planned |
| FR-253 | FR-253-AC-4 | TC-231 | planned |
| FR-254 | FR-254-AC-1 | TC-231 | planned |
| FR-254 | FR-254-AC-2 | TC-231 | planned |
| FR-254 | FR-254-AC-3 | TC-231 | planned |
| FR-254 | FR-254-AC-4 | TC-231 | planned |
| FR-254 | FR-254-AC-5 | TC-231 | planned |
| FR-255 | FR-255-AC-1 | TC-231 | planned |
| FR-255 | FR-255-AC-2 | TC-231 | planned |
| FR-255 | FR-255-AC-3 | TC-231 | planned |
| FR-255 | FR-255-AC-4 | TC-231 | planned |
| FR-255 | FR-255-AC-5 | TC-231 | planned |
| FR-256 | FR-256-AC-1 | TC-231 | planned |
| FR-256 | FR-256-AC-2 | TC-231 | planned |
| FR-256 | FR-256-AC-3 | TC-231 | planned |
| FR-256 | FR-256-AC-4 | TC-231 | planned |
| FR-256 | FR-256-AC-5 | TC-231 | planned |
| FR-256 | FR-256-AC-6 | TC-231 | planned |
| FR-257 | FR-257-AC-1 | TC-231 | planned |
| FR-257 | FR-257-AC-2 | TC-231 | planned |
| FR-257 | FR-257-AC-3 | TC-231 | planned |
| FR-257 | FR-257-AC-4 | TC-231 | planned |
| FR-257 | FR-257-AC-5 | TC-231 | planned |
| FR-257 | FR-257-AC-6 | TC-231 | planned |
| FR-258 | FR-258-AC-1 | TC-231 | planned |
| FR-258 | FR-258-AC-2 | TC-231 | planned |
| FR-258 | FR-258-AC-3 | TC-231 | planned |
| FR-258 | FR-258-AC-4 | TC-231 | planned |
| FR-258 | FR-258-AC-5 | TC-231 | planned |
| FR-258 | FR-258-AC-6 | TC-231 | planned |
| FR-258 | FR-258-AC-7 | TC-231 | planned |
| FR-259 | FR-259-AC-1 | TC-231 | planned |
| FR-259 | FR-259-AC-2 | TC-231 | planned |
| FR-259 | FR-259-AC-3 | TC-231 | planned |
| FR-259 | FR-259-AC-4 | TC-231 | planned |
| FR-259 | FR-259-AC-5 | TC-231 | planned |
| FR-259 | FR-259-AC-6 | TC-231 | planned |
| FR-269 | FR-269-AC-1 | TC-231 | planned |
| FR-269 | FR-269-AC-2 | TC-231 | planned |
| FR-269 | FR-269-AC-3 | TC-231 | planned |
| FR-269 | FR-269-AC-4 | TC-231 | planned |
| FR-270 | FR-270-AC-1 | TC-231 | planned |
| FR-270 | FR-270-AC-2 | TC-231 | planned |
| FR-270 | FR-270-AC-3 | TC-231 | planned |
| FR-270 | FR-270-AC-4 | TC-231 | planned |
| FR-272 | FR-272-AC-1 | TC-231 | planned |
| FR-272 | FR-272-AC-2 | TC-231 | planned |
| FR-272 | FR-272-AC-3 | TC-231 | planned |
| FR-272 | FR-272-AC-4 | TC-231 | planned |
| FR-272 | FR-272-AC-5 | TC-231 | planned |
| FR-274 | FR-274-AC-1 | TC-231 | planned |
| FR-274 | FR-274-AC-2 | TC-231 | planned |
| FR-274 | FR-274-AC-3 | TC-231 | planned |
| FR-274 | FR-274-AC-4 | TC-231 | planned |
| FR-274 | FR-274-AC-5 | TC-231 | planned |
| FR-280 | FR-280-AC-1 | TC-231 | planned |
| FR-280 | FR-280-AC-2 | TC-231 | planned |
| FR-280 | FR-280-AC-3 | TC-231 | planned |
| FR-281 | FR-281-AC-1 | TC-061 | planned |
| FR-281 | FR-281-AC-2 | TC-061 | planned |
| FR-281 | FR-281-AC-3 | TC-061 | planned |
| FR-282 | FR-282-AC-1 | TC-231 | planned |
| FR-282 | FR-282-AC-2 | TC-231 | planned |
| FR-282 | FR-282-AC-3 | TC-231 | planned |
| FR-282 | FR-282-AC-4 | TC-231 | planned |
| FR-284 | FR-284-AC-1 | TC-231 | planned |
| FR-284 | FR-284-AC-2 | TC-231 | planned |
| FR-284 | FR-284-AC-3 | TC-231 | planned |
| FR-284 | FR-284-AC-4 | TC-231 | planned |
| FR-284 | FR-284-AC-5 | TC-231 | planned |
| FR-284 | FR-284-AC-6 | TC-231 | planned |
| FR-284 | FR-284-AC-7 | TC-231 | planned |
| FR-284 | FR-284-AC-8 | TC-231 | planned |
| FR-297 | FR-297-AC-2 | TC-150 | planned |
| FR-297 | FR-297-AC-3 | TC-150 | planned |
| FR-297 | FR-297-AC-4 | TC-150 | planned |
| FR-298 | FR-298-AC-2 | TC-150 | planned |
| FR-298 | FR-298-AC-3 | TC-150 | planned |
| FR-298 | FR-298-AC-4 | TC-150 | planned |
| FR-298 | FR-298-AC-5 | TC-150 | planned |
| FR-299 | FR-299-AC-2 | TC-231 | planned |
| FR-299 | FR-299-AC-3 | TC-231 | planned |
| FR-299 | FR-299-AC-4 | TC-231 | planned |
| FR-299 | FR-299-AC-5 | TC-231 | planned |
| FR-140 | FR-140-AC-4 | TC-185 | planned |
| FR-140 | FR-140-AC-5 | TC-185 | planned |
| FR-140 | FR-140-AC-6 | TC-185 | planned |
| FR-140 | FR-140-AC-7 | TC-185 | planned |
| FR-141 | FR-141-AC-4 | TC-186 | planned |
| FR-141 | FR-141-AC-5 | TC-186 | planned |
| FR-141 | FR-141-AC-6 | TC-186 | planned |
| FR-142 | FR-142-AC-4 | TC-187 | planned |
| FR-142 | FR-142-AC-5 | TC-187 | planned |
| FR-142 | FR-142-AC-6 | TC-187 | planned |
| FR-142 | FR-142-AC-7 | TC-187 | planned |
| FR-142 | FR-142-AC-8 | TC-233 | planned |
| FR-142 | FR-142-AC-9 | TC-187 | planned |
| FR-142 | FR-142-AC-10 | TC-187 | planned |
| FR-143 | FR-143-AC-4 | TC-188 | planned |
| FR-143 | FR-143-AC-5 | TC-188 | planned |
| FR-143 | FR-143-AC-6 | TC-188 | planned |
| FR-143 | FR-143-AC-7 | TC-188 | planned |
| FR-143 | FR-143-AC-8 | TC-188 | planned |
| FR-143 | FR-143-AC-9 | TC-188 | planned |
| FR-143 | FR-143-AC-10 | TC-188 | planned |
| FR-144 | FR-144-AC-4 | TC-189 | planned |
| FR-144 | FR-144-AC-5 | TC-189 | planned |
| FR-144 | FR-144-AC-6 | TC-189 | planned |
| FR-144 | FR-144-AC-7 | TC-189 | planned |
| FR-144 | FR-144-AC-8 | TC-189 | planned |
| FR-144 | FR-144-AC-9 | TC-189 | planned |
| FR-144 | FR-144-AC-10 | TC-189 | planned |
| FR-145 | FR-145-AC-4 | TC-190 | planned |
| FR-145 | FR-145-AC-5 | TC-190 | planned |
| FR-145 | FR-145-AC-6 | TC-190 | planned |
| FR-145 | FR-145-AC-7 | TC-190 | planned |
| FR-145 | FR-145-AC-8 | TC-190 | planned |
| FR-145 | FR-145-AC-9 | TC-190 | planned |
| FR-146 | FR-146-AC-4 | TC-191 | planned |
| FR-146 | FR-146-AC-5 | TC-191 | planned |
| FR-146 | FR-146-AC-6 | TC-191 | planned |
| FR-146 | FR-146-AC-7 | TC-191 | planned |
| FR-146 | FR-146-AC-8 | TC-191 | planned |
| FR-146 | FR-146-AC-9 | TC-191 | planned |
| FR-147 | FR-147-AC-4 | TC-192 | planned |
| FR-147 | FR-147-AC-5 | TC-192 | planned |
| FR-147 | FR-147-AC-6 | TC-192 | planned |
| FR-148 | FR-148-AC-4 | TC-193 | planned |
| FR-148 | FR-148-AC-5 | TC-193 | planned |
| FR-148 | FR-148-AC-6 | TC-193 | planned |
| FR-148 | FR-148-AC-7 | TC-193 | planned |
| FR-148 | FR-148-AC-8 | TC-193 | planned |
| FR-148 | FR-148-AC-9 | TC-193 | planned |
| FR-148 | FR-148-AC-10 | TC-193 | planned |
| FR-149 | FR-149-AC-4 | TC-194 | planned |
| FR-149 | FR-149-AC-5 | TC-194 | planned |
| FR-149 | FR-149-AC-6 | TC-194 | planned |
| FR-149 | FR-149-AC-7 | TC-194 | planned |
| FR-149 | FR-149-AC-8 | TC-194 | planned |
| FR-149 | FR-149-AC-9 | TC-194 | planned |
| FR-149 | FR-149-AC-10 | TC-194 | planned |
| FR-149 | FR-149-AC-11 | TC-194 | planned |
| FR-149 | FR-149-AC-13 | TC-194 | planned |
| FR-150 | FR-150-AC-4 | TC-195 | planned |
| FR-150 | FR-150-AC-5 | TC-195 | planned |
| FR-151 | FR-151-AC-4 | TC-196 | planned |
| FR-151 | FR-151-AC-5 | TC-196 | planned |
| FR-152 | FR-152-AC-4 | TC-197 | planned |
| FR-152 | FR-152-AC-5 | TC-197 | planned |
| FR-153 | FR-153-AC-4 | TC-198 | planned |
| FR-153 | FR-153-AC-5 | TC-198 | planned |
| FR-143 | FR-143-AC-11 | TC-198 | planned |
| FR-144 | FR-144-AC-11 | TC-198 | planned |
| FR-149 | FR-149-AC-12 | TC-198 | planned |
| FR-150 | FR-150-AC-6 | TC-195 | planned |
| FR-150 | FR-150-AC-7 | TC-195 | planned |
| FR-150 | FR-150-AC-8 | TC-195 | planned |
| FR-151 | FR-151-AC-6 | TC-196 | planned |
| FR-151 | FR-151-AC-7 | TC-196 | planned |
| FR-151 | FR-151-AC-8 | TC-196 | planned |
| FR-151 | FR-151-AC-9 | TC-196 | planned |
| FR-151 | FR-151-AC-10 | TC-196 | planned |
| FR-152 | FR-152-AC-6 | TC-197 | planned |
| FR-152 | FR-152-AC-7 | TC-197 | planned |
| FR-152 | FR-152-AC-8 | TC-197 | planned |
| FR-153 | FR-153-AC-6 | TC-198 | planned |
| FR-153 | FR-153-AC-7 | TC-198 | planned |
| FR-153 | FR-153-AC-8 | TC-198 | planned |
| FR-154 | FR-154-AC-1 | TC-234 | planned |
| FR-154 | FR-154-AC-2 | TC-234 | planned |
| FR-154 | FR-154-AC-3 | TC-234 | planned |
| FR-154 | FR-154-AC-4 | TC-234 | planned |
| FR-154 | FR-154-AC-5 | TC-234 | planned |
| FR-154 | FR-154-AC-6 | TC-234 | planned |
| FR-154 | FR-154-AC-7 | TC-234 | planned |
| FR-154 | FR-154-AC-8 | TC-234 | planned |
| FR-204 | FR-204-AC-5 | TC-198 | planned |
| FR-208 | FR-208-AC-1 | TC-231 | planned |
| FR-208 | FR-208-AC-2 | TC-197 | planned |
| FR-208 | FR-208-AC-3 | TC-234 | planned |
| FR-208 | FR-208-AC-4 | TC-235 | planned |
| FR-208 | FR-208-AC-5 | TC-235 | planned |
| FR-208 | FR-208-AC-6 | TC-235 | planned |
| FR-208 | FR-208-AC-7 | TC-235 | planned |
| FR-208 | FR-208-AC-8 | TC-235 | planned |
| FR-208 | FR-208-AC-9 | TC-235 | planned |
| FR-208 | FR-208-AC-10 | TC-235 | planned |
| FR-208 | FR-208-AC-11 | TC-235 | planned |
| FR-208 | FR-208-AC-12 | TC-235 | planned |
| FR-272 | FR-272-AC-6 | TC-196 | planned |
| FR-272 | FR-272-AC-7 | TC-233 | planned |
| FR-321 | FR-321-AC-4 | TC-195 | planned |
| FR-160 | FR-160-AC-4 | TC-199 | planned |
| FR-160 | FR-160-AC-5 | TC-199 | planned |
| FR-161 | FR-161-AC-4 | TC-200 | planned |
| FR-161 | FR-161-AC-5 | TC-200 | planned |
| FR-170 | FR-170-AC-4 | TC-201 | planned |
| FR-170 | FR-170-AC-5 | TC-201 | planned |
| FR-171 | FR-171-AC-4 | TC-202 | planned |
| FR-171 | FR-171-AC-5 | TC-202 | planned |
| FR-172 | FR-172-AC-4 | TC-203 | planned |
| FR-172 | FR-172-AC-5 | TC-203 | planned |
| FR-173 | FR-173-AC-4 | TC-204 | planned |
| FR-173 | FR-173-AC-5 | TC-204 | planned |
| FR-174 | FR-174-AC-4 | TC-205 | planned |
| FR-174 | FR-174-AC-5 | TC-205 | planned |
| FR-175 | FR-175-AC-4 | TC-206 | planned |
| FR-175 | FR-175-AC-5 | TC-206 | planned |
| FR-176 | FR-176-AC-4 | TC-207 | planned |
| FR-176 | FR-176-AC-5 | TC-207 | planned |
| FR-177 | FR-177-AC-4 | TC-208 | planned |
| FR-177 | FR-177-AC-5 | TC-208 | planned |
| FR-180 | FR-180-AC-4 | TC-209 | planned |
| FR-180 | FR-180-AC-5 | TC-209 | planned |
| FR-181 | FR-181-AC-4 | TC-210 | planned |
| FR-181 | FR-181-AC-5 | TC-210 | planned |
| FR-181 | FR-181-AC-6 | TC-210 | planned |
| FR-182 | FR-182-AC-4 | TC-211 | planned |
| FR-182 | FR-182-AC-5 | TC-211 | planned |
| FR-190 | FR-190-AC-4 | TC-212 | planned |
| FR-190 | FR-190-AC-5 | TC-212 | planned |
| FR-191 | FR-191-AC-4 | TC-213 | planned |
| FR-191 | FR-191-AC-5 | TC-213 | planned |
| FR-192 | FR-192-AC-4 | TC-214 | planned |
| FR-192 | FR-192-AC-5 | TC-214 | planned |
| FR-192 | FR-192-AC-6 | TC-214 | planned |
| FR-192 | FR-192-AC-7 | TC-214 | planned |
| FR-193 | FR-193-AC-4 | TC-215 | planned |
| FR-193 | FR-193-AC-5 | TC-215 | planned |
| FR-194 | FR-194-AC-4 | TC-216 | planned |
| FR-194 | FR-194-AC-5 | TC-216 | planned |
| FR-195 | FR-195-AC-4 | TC-217 | planned |
| FR-195 | FR-195-AC-5 | TC-217 | planned |
| FR-196 | FR-196-AC-4 | TC-218 | planned |
| FR-196 | FR-196-AC-5 | TC-218 | planned |
| FR-197 | FR-197-AC-4 | TC-219 | planned |
| FR-197 | FR-197-AC-5 | TC-219 | planned |
| FR-320 | FR-320-AC-1 | TC-231 | planned |
| FR-320 | FR-320-AC-2 | TC-231 | planned |
| FR-320 | FR-320-AC-3 | TC-231 | planned |
| FR-321 | FR-321-AC-1 | TC-231 | planned |
| FR-321 | FR-321-AC-2 | TC-231 | planned |
| FR-321 | FR-321-AC-3 | TC-231 | planned |
| FR-322 | FR-322-AC-1 | TC-231 | planned |
| FR-322 | FR-322-AC-2 | TC-231 | planned |
| FR-322 | FR-322-AC-3 | TC-231 | planned |
| FR-322 | FR-322-AC-4 | TC-217 | planned |
| FR-322 | FR-322-AC-5 | TC-217 | planned |
| FR-322 | FR-322-AC-6 | TC-217 | planned |
| FR-322 | FR-322-AC-7 | TC-217 | planned |
| FR-322 | FR-322-AC-8 | TC-217 | planned |
| FR-322 | FR-322-AC-9 | TC-217 | planned |
| FR-322 | FR-322-AC-10 | TC-217 | planned |
| FR-322 | FR-322-AC-13 | TC-233 | planned |
| FR-322 | FR-322-AC-14 | TC-233 | planned |
| FR-322 | FR-322-AC-15 | TC-233 | planned |
| FR-322 | FR-322-AC-17 | TC-233 | planned |
| FR-322 | FR-322-AC-18 | TC-233 | planned |
| FR-322 | FR-322-AC-19 | TC-233 | planned |
| FR-322 | FR-322-AC-21 | TC-233 | planned |
| FR-322 | FR-322-AC-22 | TC-233 | planned |
| FR-322 | FR-322-AC-23 | TC-233 | planned |
| FR-322 | FR-322-AC-24 | TC-233 | planned |
| interface_004 | interface_004-AC-1 | TC-233 | planned |
| interface_004 | interface_004-AC-2 | TC-233 | planned |
| FR-323 | FR-323-AC-1 | TC-231 | planned |
| FR-323 | FR-323-AC-2 | TC-231 | planned |
| FR-323 | FR-323-AC-3 | TC-231 | planned |
| FR-324 | FR-324-AC-1 | TC-231 | planned |
| FR-324 | FR-324-AC-2 | TC-231 | planned |
| FR-324 | FR-324-AC-3 | TC-231 | planned |
| FR-325 | FR-325-AC-1 | TC-231 | planned |
| FR-325 | FR-325-AC-2 | TC-231 | planned |
| FR-325 | FR-325-AC-3 | TC-231 | planned |
| FR-326 | FR-326-AC-1 | TC-231 | planned |
| FR-326 | FR-326-AC-2 | TC-231 | planned |
| FR-326 | FR-326-AC-3 | TC-231 | planned |
| FR-327 | FR-327-AC-1 | TC-231 | planned |
| FR-327 | FR-327-AC-2 | TC-231 | planned |
| FR-327 | FR-327-AC-3 | TC-231 | planned |
| FR-328 | FR-328-AC-1 | TC-231 | planned |
| FR-328 | FR-328-AC-2 | TC-231 | planned |
| FR-328 | FR-328-AC-3 | TC-231 | planned |
| FR-329 | FR-329-AC-1 | TC-231 | planned |
| FR-329 | FR-329-AC-2 | TC-231 | planned |
| FR-329 | FR-329-AC-3 | TC-231 | planned |
| FR-330 | FR-330-AC-1 | TC-231 | planned |
| FR-330 | FR-330-AC-2 | TC-231 | planned |
| FR-330 | FR-330-AC-3 | TC-231 | planned |
| FR-331 | FR-331-AC-1 | TC-231 | planned |
| FR-331 | FR-331-AC-2 | TC-231 | planned |
| FR-331 | FR-331-AC-3 | TC-231 | planned |
| FR-332 | FR-332-AC-1 | TC-231 | planned |
| FR-332 | FR-332-AC-2 | TC-231 | planned |
| FR-332 | FR-332-AC-3 | TC-231 | planned |
| FR-333 | FR-333-AC-1 | TC-231 | planned |
| FR-333 | FR-333-AC-2 | TC-231 | planned |
| FR-333 | FR-333-AC-3 | TC-231 | planned |
| FR-334 | FR-334-AC-1 | TC-231 | planned |
| FR-334 | FR-334-AC-2 | TC-231 | planned |
| FR-334 | FR-334-AC-3 | TC-231 | planned |
| FR-335 | FR-335-AC-1 | TC-231 | planned |
| FR-335 | FR-335-AC-2 | TC-231 | planned |
| FR-335 | FR-335-AC-3 | TC-231 | planned |
| FR-336 | FR-336-AC-1 | TC-231 | planned |
| FR-336 | FR-336-AC-2 | TC-231 | planned |
| FR-336 | FR-336-AC-3 | TC-231 | planned |
| FR-337 | FR-337-AC-1 | TC-231 | planned |
| FR-337 | FR-337-AC-2 | TC-231 | planned |
| FR-337 | FR-337-AC-3 | TC-231 | planned |
| FR-338 | FR-338-AC-1 | TC-231 | planned |
| FR-338 | FR-338-AC-2 | TC-231 | planned |
| FR-338 | FR-338-AC-3 | TC-231 | planned |
| FR-339 | FR-339-AC-1 | TC-180 | planned |
| FR-339 | FR-339-AC-2 | TC-180 | planned |
| FR-339 | FR-339-AC-3 | TC-180 | planned |
| FR-339 | FR-339-AC-4 | TC-180 | planned |
| FR-351 | FR-351-AC-1 | TC-254 | planned |
| FR-351 | FR-351-AC-2 | TC-254 | planned |
| FR-351 | FR-351-AC-3 | TC-254 | planned |
| FR-351 | FR-351-AC-4 | TC-254 | planned |
| FR-351 | FR-351-AC-5 | TC-254 | planned |
| FR-351 | FR-351-AC-6 | TC-254 | planned |
| FR-352 | FR-352-AC-1 | TC-255 | planned |
| FR-352 | FR-352-AC-2 | TC-255 | planned |
| FR-352 | FR-352-AC-3 | TC-255 | planned |
| FR-352 | FR-352-AC-4 | TC-255 | planned |
| FR-352 | FR-352-AC-5 | TC-255 | planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-180 | Complete V1 profile bundle accounting | Property | P0 | FR-131 | 🚧 Planned |
| TC-181 | Recursive function termination profiles | Property | P0 | FR-146 | 🚧 Planned |
| TC-182 | Canonical complete package manifest | Property | P0 | FR-132 | 🚧 Planned |
| TC-183 | Uniform extension compatibility | Property | P0 | FR-133 | 🚧 Planned |
| TC-184 | Original source locus preservation | Property | P0 | FR-134 | 🚧 Planned |
| TC-185 | Exact decimal semantics | Property | P0 | FR-140 | 🚧 Planned |
| TC-186 | Text and enum identity semantics | Property | P0 | FR-141 | 🚧 Planned |
| TC-187 | Quantity and unit conversion | Property | P0 | FR-142 | 🚧 Planned |
| TC-188 | Record tuple and recursive values | Property | P0 | FR-143 | 🚧 Planned |
| TC-189 | Collection kind algebra | Property | P0 | FR-144 | 🚧 Planned |
| TC-190 | Collection query and conversion algebra | Property | P0 | FR-145 | 🚧 Planned |
| TC-191 | Total pure function checking | Property | P0 | FR-146 | 🚧 Planned |
| TC-192 | Integer division profiles | Property | P0 | FR-147 | 🚧 Planned |
| TC-193 | IEEE exceptional and rounding profiles | Property | P0 | FR-148 | 🚧 Planned |
| TC-194 | Complete equality matrix | Property | P0 | FR-149 | 🚧 Planned |
| TC-195 | Model normalization provenance | Property | P0 | FR-150 | 🚧 Planned |
| TC-196 | Conformance redefinition and dispatch | Property | P0 | FR-151 | 🚧 Planned |
| TC-197 | Systems-model typed binding | Property | P0 | FR-152, FR-208 | 🚧 Planned |
| TC-198 | Closed model lookup | Property | P0 | FR-153 | 🚧 Planned |
| TC-199 | Uncertain revisioned temporal evaluation | Property | P0 | FR-160 | 🚧 Planned |
| TC-200 | Infinite-trace provider boundary | Property | P0 | FR-161 | 🚧 Planned |
| TC-201 | Channel and queue premises | Property | P0 | FR-170 | 🚧 Planned |
| TC-202 | Join loop and dynamic-role control | Property | P0 | FR-171 | 🚧 Planned |
| TC-203 | Choice obligation and capture binding | Property | P0 | FR-172 | 🚧 Planned |
| TC-204 | Complete compensation lifecycle | Property | P0 | FR-173 | 🚧 Planned |
| TC-205 | Sound global projection | Property | P0 | FR-174 | 🚧 Planned |
| TC-206 | Protocol realizability witnesses | Property | P0 | FR-175 | 🚧 Planned |
| TC-207 | Assume-guarantee composition | Property | P0 | FR-176 | 🚧 Planned |
| TC-208 | Protocol refinement relation | Property | P0 | FR-177 | 🚧 Planned |
| TC-209 | Complete reference workflow corpus | Property | P0 | FR-180 | 🚧 Planned |
| TC-210 | Finite simulation reproducibility | Property | P0 | FR-181 | 🚧 Planned |
| TC-211 | Authorized idempotent workflow effects | Property | P0 | FR-182 | 🚧 Planned |
| TC-212 | Model-based generation and shrinking | Property | P0 | FR-190 | 🚧 Planned |
| TC-213 | Relational and hyperproperty analysis | Property | P0 | FR-191 | 🚧 Planned |
| TC-214 | Verification plan dispatch | Property | P0 | FR-192 | 🚧 Planned |
| TC-215 | Continuous hybrid analysis | Property | P0 | FR-193 | 🚧 Planned |
| TC-216 | Bounded synthesis and checking | Property | P0 | FR-194 | 🚧 Planned |
| TC-217 | Complete contract IR lowering | Property | P0 | FR-195 | 🚧 Planned |
| TC-218 | Kani model-domain harnesses | Property | P0 | FR-196 | 🚧 Planned |
| TC-219 | Backend counterexample native replay | Property | P0 | FR-197 | 🚧 Planned |
| TC-220 | Rust lifecycle API contract | Property | P0 | FR-300 | 🚧 Planned |
| TC-221 | CLI lifecycle and exits | Property | P0 | FR-301 | 🚧 Planned |
| TC-222 | Lossless CST round trip | Property | P0 | FR-302 | 🚧 Planned |
| TC-223 | Formatter LSP incremental parity | Property | P0 | FR-303 | 🚧 Planned |
| TC-224 | Model query and rendering determinism | Property | P0 | FR-304 | 🚧 Planned |
| TC-225 | Rust plugin validation and containment | Property | P0 | FR-305 | 🚧 Planned |
| TC-226 | AOT JIT native and cache parity | Property | P0 | FR-306 | 🚧 Planned |
| TC-227 | Reusable semantic library resolution | Property | P0 | FR-307 | 🚧 Planned |
| TC-228 | WASM native parity and bounds | Property | P0 | FR-308 | 🚧 Planned |
| TC-229 | Analysis codegen accounting | Property | P0 | FR-309 | 🚧 Planned |
| TC-230 | Semantic assurance traceability | Property | P0 | FR-310 | 🚧 Planned |
| TC-231 | Complete V1 qualification coverage | Property | P0 | FR-311 | 🚧 Planned |
| TC-232 | Counterexample lineage integrity | Property | P0 | FR-312 | 🚧 Planned |
| TC-233 | Published checked-package and scalar contract vectors | Property | P0 | FR-322 | 🚧 Planned |
| TC-234 | Domain package model intake | Property | P0 | FR-154, FR-208 | 🚧 Planned |
| TC-235 | Business construct meaning binding | Property | P0 | FR-208, FR-154, FR-207, FR-047 | 🚧 Planned |

## Integration Test Matrix

| Integration ID | Purpose | Target Project | Type | Test Cases | Coverage Status |
| --- | --- | --- | --- | --- | --- |
| IT-070 | Artifact bundle to checked package | quire-rs extraction + FCD extraction-frontend + quire-spec-language | library | TC-195, TC-196, TC-197, TC-198, TC-234, TC-235 | planned |
| IT-071 | Composed domain execution | quire-integration + domain runtimes | library | TC-209, TC-210 | planned |
| IT-072 | Revisioned observation repair | quire-observation + temporal/protocol | library | TC-199, TC-204 | planned |
| IT-073 | Protocol analysis suite | quire-spec-language + quire-protocol | library | TC-205–TC-208 | planned |
| IT-074 | Verification method dispatch | quire-verification + method providers | library/process | TC-212–TC-216 | planned |
| IT-075 | Contract backend native replay | Contract IR/codegen/runtime + QSL | library/tool | TC-217–TC-219 | planned |
| IT-076 | API/CLI/WASM/editor parity | QSL tool surfaces | library/process | TC-220–TC-224, TC-228 | planned |
| IT-077 | Plugin/JIT containment | Rust provider host | library/process | TC-225, TC-226 | planned |
| IT-078 | Workflow to evidence | quire-integration + observation + verification | library/process | TC-211, TC-230–TC-232 | planned |

## Model-domain capability coverage note

V1-TYPE-021 (FR-011), V1-TYPE-022 (FR-009) and V1-EXPR-017 through V1-EXPR-022
(FR-009, FR-012, FR-006, FR-013) keep their primary requirements. Under the
selected `quire.model.complete/v1` definition their inheritance-dependent
meaning is covered by TC-043 and TC-046 together with the TC-196 vectors R02
(frame with a subtype create), R08 and D06 (redefined operation pre- and
postconditions) and the TC-198 vectors L01, L05 and L07 (identity, population
and anchors under inheritance); V1-TYPE-024 is resolved to FR-043, which
supersedes FR-010's parent reachability.

## Coverage gate

Complete-V1 qualification requires all 185 capability rows and every selected
acceptance-criterion row to have executed evidence under one exact ecosystem
lock. Until then the status remains planned or partial and cannot be promoted by
family-level samples.
