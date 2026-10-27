---
paths:
  - "packages/ai-coach/**"
  - "apps/api/src/modules/coach/**"
---

# AI coach rules (apply when touching these files)

- The coach explains engine output; it never computes. Its input is a structured summary of
  the spot (cards, stacks, action, engine numbers with their exact/estimate flag); its output
  is prose plus the ids of the numbers it cites.
- Every number in an explanation must appear in the input. A post-check compares numbers in
  the output with the input and drops or regenerates an explanation that adds one; the check
  has a unit test with a planted invented number.
- Estimates stay labelled in the prose ("about", "estimated") exactly as the engine flagged
  them.
- Default model `claude-haiku-5-5` with prompt caching on the system prompt and spot schema;
  a stronger model only for Coach-tier session reviews. The model id lives in one config
  place.
- Per-user monthly cap (soft cap 500–1,000 explanations on Pro), counted server-side; over the
  cap the app shows engine numbers without prose. AI off or provider down → same fallback.
- No hand data, prompts or completions in logs; usage is logged as counts and token totals
  only.
- Prompts live in versioned files under `packages/ai-coach/prompts/`; a prompt change runs the
  coach golden set (spots with expected cited numbers) before merge.
