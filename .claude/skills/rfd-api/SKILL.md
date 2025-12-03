---
name: rfd-api
description: Read-only access to Oxide Computer Company RFDs (Requests for Discussion) via rfd-cli. Use for researching Oxide design decisions, architecture, technical specs, and engineering history. Triggers include "what does RFD say", "search RFDs", "find RFDs about", "look up RFD", "check the RFD for", "how does Oxide handle", "what's the design for", or any reference to a specific RFD number.
allowed-tools: [Bash, WebFetch]
---

# RFD API Research Skill

This skill enables **read-only** access to Oxide Computer Company's RFD (Request for Discussion) documents via the `rfd-cli` tool.

## When to Use This Skill

**USE** this skill when:
- The user asks about Oxide design decisions or architecture
- The user references a specific RFD by number (e.g., "RFD 123", "RFD-45")
- You need to research how Oxide handles a technical topic
- The user asks to search or find RFDs on a topic
- You need context from Oxide engineering documents to answer a question

**DO NOT USE** this skill for:
- Creating, editing, or modifying RFDs (this skill is read-only)
- Administrative operations (user management, permissions, etc.)
- Anything unrelated to Oxide RFD documents

## First-Time Setup

On first use, walk the user through this setup process. Explain what you're doing at each step.

### Step 1: Check if rfd-cli is installed

```bash
which rfd-cli
```

If not found, inform the user that you need to install `rfd-cli` to access RFD documents, then run:

```bash
cargo install --git https://github.com/oxidecomputer/rfd-api.git rfd-cli
```

This requires Rust/Cargo. If cargo is not available, direct the user to https://rustup.rs/ first.

### Step 2: Configure the API host

```bash
rfd-cli config set host https://rfd-api.shared.oxide.computer
```

### Step 3: Authenticate

Ask the user which authentication method they prefer:

**Short-lived session** (expires, requires re-auth periodically):
```bash
rfd-cli auth login google
```

**Long-lived API token** (recommended for repeated use):
```bash
rfd-cli auth login -m token google
```

Note: The `-m token` flag must come before the provider name (`google`).

Either command will open a browser for Google OAuth authentication and store credentials locally.

### Step 4: Verify setup

```bash
rfd-cli list --format json | head -c 500
```

If this returns RFD data, setup is complete.

## Available Commands (Read-Only)

Always use `--format json` for machine-readable output that's easier to parse.

**Important**: Most commands use named flags (e.g., `--number`, `--q`) rather than positional arguments.

### List all RFDs

```bash
rfd-cli list --format json
```

Returns an array of RFD metadata objects (without full content).

### Search RFDs

```bash
rfd-cli search --q "<QUERY>" --format json
```

Full-text search across all accessible RFDs. Returns matching results with hierarchy, content snippets, and RFD numbers.

Example: `rfd-cli search --q "authentication" --format json`

**Optional search parameters:**
- `--limit <N>` - Limit number of results (default: 20)
- `--offset <N>` - Skip first N results (for pagination)
- `--highlight-pre-tag <TAG>` - Custom tag before highlighted matches (default: `<em>`)
- `--highlight-post-tag <TAG>` - Custom tag after highlighted matches (default: `</em>`)
- `--attributes-to-crop <ATTRS>` - Comma-separated list of attributes to crop

Example with pagination: `rfd-cli search --q "network" --limit 10 --offset 20 --format json`

**Search results include:**
- `rfd_number` - The RFD number
- `hierarchy` - Document section hierarchy (title, headings)
- `content` - Matching text snippet
- `anchor` - Link anchor to the specific section

### View RFD metadata only

```bash
rfd-cli meta --number <NUMBER> --format json
```

Returns metadata without the document body. Faster for quick lookups.

Example: `rfd-cli meta --number 238 --format json`

### View full RFD content

```bash
rfd-cli view --number <NUMBER> --format json
```

Returns the full RFD including raw document content (can be large).

Example: `rfd-cli view --number 238 --format json`

For very long RFDs, consider piping through `head`:
```bash
rfd-cli view --number 238 --format json 2>/dev/null | head -c 50000
```

### View specific RFD attribute

```bash
rfd-cli attr --number <NUMBER> --attr <ATTRIBUTE> --format json
```

Attributes: `discussion`, `labels`, `state`

### View RFD discussion URL

```bash
rfd-cli attr --number <NUMBER> --attr discussion --format json
```

Returns the discussion PR URL for an RFD.

### Get PR comments for an RFD

```bash
rfd-cli discussion --number <NUMBER> --format json
```

Returns comments from the GitHub discussion PR (may be empty for some RFDs).

### List RFD revisions

```bash
rfd-cli revision list --number <NUMBER> --format json
```

Returns all revisions for a given RFD number. Each revision includes:
- `id` - UUID of the revision (use this for other revision commands)
- `commit_sha` - Git commit SHA
- `committed_at` - Timestamp

### View specific revision

```bash
rfd-cli revision view --number <NUMBER> --revision <REVISION_UUID> --format json
```

View a specific historical revision of an RFD. The `--revision` flag takes the revision UUID from `revision list`, not the commit SHA.

### View revision metadata

```bash
rfd-cli revision meta --number <NUMBER> --revision <REVISION_UUID> --format json
```

Get metadata for a specific revision. The `--revision` flag takes the revision UUID.

## RFD States

RFDs progress through these states (see RFD 1 for full details):

- `ideation` - Only a topic description exists; a "scratchpad" for related ideas. Anyone can pick this up and develop it further.
- `prediscussion` - Work in progress, being rapidly iterated on in a branch, not yet ready for broader discussion.
- `discussion` - Active discussion happening in a Pull Request. The RFD is open for feedback.
- `published` - The PR has been merged. The RFD represents a direction/decision, but can still be updated and discussed.
- `committed` - The idea has been fully implemented. This is an explanation of how a system *works*, not just how it *might* work.
- `abandoned` - The idea was found to be non-viable or should be ignored.

## Key RFD Concepts

**Determination**: A key concept from RFD 5 (Phases of Engineering). A "determination" is a decision about technical direction - choosing a path after exploring alternatives. Many RFDs include a "Determination" section documenting what was decided and why. When researching a topic, look for the determination to understand what was ultimately chosen.

**Labels**: RFDs use labels for categorization. Common labels include: `control-plane`, `hardware`, `software`, `network`, `storage`, `security`, `virtualization`, `boot`, `root-of-trust`, `host`, `process`, `corp`, `console`, `fault-management`.

**Short URLs**: RFDs can be accessed via short URLs:
- `{num}.rfd.oxide.computer` - e.g., `238.rfd.oxide.computer`
- `rfd.oxide.computer/{num}` - e.g., `rfd.oxide.computer/238`
- `{num}.rfd.oxide.computer/discussion` - links to the discussion PR

**Internal references**: RFDs use AsciiDoc format with references like `<<rfd301>>` or `[RFD 301]` to cite other RFDs. When you see these, consider fetching the referenced RFD for context.

## Foundational RFDs (Public)

These publicly-visible RFDs provide essential context for understanding Oxide's approach, culture, and architecture. This is a curated list of starting points - many more valuable RFDs exist but may require authentication to access.

### Culture & Process
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 1 | Requests for Discussion | The RFD process itself |
| 2 | Mission, Principles and Values | Oxide's philosophical foundation |
| 3 | Oxide Hiring Process | How Oxide evaluates and hires |
| 5 | Phases of Engineering | Engineering methodology (scoping → determination → production) |
| 38 | Journal Club | How Oxide stays current with research |
| 83 | Preserving Time for Focused Work | Focus Day and meeting culture |
| 113 | Engineering Determination | How to make technical decisions |
| 537 | Record Every Meeting | Transparency in practice |

### Architecture & System Design
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 4 | User Facing API | API design principles |
| 26 | Host Operating System & Hypervisor | Why illumos (the OS choice) |
| 48 | Control Plane Requirements | Core control plane architecture |
| 53 | Control Plane Data Storage | CockroachDB and storage philosophy |
| 60 | Storage Architecture Considerations | Storage design philosophy |
| 63 | Network Architecture | Networking approach (IPv6, OPTE, VPCs) |
| 177 | Crucible | Distributed block storage architecture |
| 241 | Holistic Boot | Boot architecture strategy |

### Operator & Product Experience
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 82 | Operator Facilities Design | Operator experience principles |

### Software Patterns & Practices
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 79 | Rust Concurrency | Async vs sync patterns |
| 192 | Omicron Database Design | CockroachDB patterns in Nexus |
| 289 | Steno Upgrade | Distributed saga patterns |
| 373 | Reliable Persistent Workflows | State reconciliation patterns |
| 397 | Async/await Challenges | Cancel safety problems |
| 400 | Cancel Safety | Solutions for async Rust |
| 419 | Unwinding Sagas | Writing correct saga nodes |
| 479 | Dropshot API traits | API framework patterns |
| 532 | Internal HTTP API Versioning | Patterns for internal APIs |

### Hardware & Standards
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 552 | Transparency in HW/SW Interfaces | Philosophy on hardware partnerships |

### Terminology & Reference
| RFD | Title | Why It's Foundational |
|-----|-------|----------------------|
| 203 | Standard Units for Bits | Data measurement standards |

## Output Format

When using `--format json`, responses include structured data. Key fields in RFD objects:

- `rfd_number` - The RFD number (integer)
- `title` - RFD title
- `state` - Current state (see above)
- `authors` - Author information
- `labels` - Comma-separated labels
- `discussion` - URL to discussion PR
- `visibility` - `Public` or `Private`
- `content` - Full document content (only in `view` output)
- `commit` - Git commit SHA
- `committed_at` - Commit timestamp

## Usage Guidelines

1. **Start with search** when looking for RFDs on a topic: `rfd-cli search --q "topic" --format json`
2. **Use meta for quick lookups** when you just need basic info: `rfd-cli meta --number <N> --format json`
3. **Use view for full content** when you need to read the actual document: `rfd-cli view --number <N> --format json`
4. **Check discussion URLs** to find related GitHub PRs and conversations
5. **Use quotes to support narrative, not replace it**:
   - Build a coherent explanation first, then use quotes to illuminate key points
   - Quote passages that reveal reasoning, philosophy, or memorable articulations
   - Use block quotes (>) sparingly for particularly important passages
   - Weave shorter quotes inline to support points you're making
   - Always cite the RFD number when quoting
   - Avoid "quote vignettes" - disconnected quotations with minimal explanation
   - Your goal is to educate: explain the why, how, and what next
6. **Write conversationally** - maintain a fluid, educational tone that builds understanding progressively

## Research Strategies

### Simple Lookups vs Deep Research

**For simple questions** ("what does RFD 238 say about X", "what's the state of RFD 123"):
- Fetch the specific RFD directly
- Search for keywords if you don't know the RFD number
- Quick and interactive

**For broad research** ("how does Oxide approach fault management", "what's the history of the update system", "find all RFDs about security"):
- Consider using sub-agents to parallelize the work
- Each agent can fetch and summarize a batch of RFDs
- Then synthesize the results into a coherent answer
- This is more thorough but takes longer and uses more resources

**When to use sub-agents:**
- The topic likely spans 5+ RFDs
- You need to understand how a design evolved over time
- The user is asking for comprehensive background, not a quick answer
- You're searching for something without knowing which RFDs are relevant

**Sub-agent pattern for RFD research:**
1. Search or list RFDs to identify candidates
2. Dispatch agents in parallel, each analyzing a batch of RFDs
3. Have each agent summarize findings and return structured results
4. Synthesize the agent results into a final answer

Ask the user if they want a quick answer or thorough research when the scope is ambiguous.

**Getting a complete RFD index**: If you need to know what RFDs exist beyond the foundational list, fetch a complete index on-demand:
```bash
rfd-cli list --format json | jq -r '.[] | "\(.rfd_number): \(.title)"'
```
This returns all accessible RFD numbers and titles (~600+ RFDs). Only fetch this when needed, as it adds context overhead.

## Following Links and References

When reading RFDs, behave like a researcher browsing documentation - follow links to build a complete picture:

**Internal RFD references**: When an RFD mentions another RFD (e.g., "see RFD 301", "as described in <<rfd355>>"), fetch that referenced RFD to understand the full context. RFDs often build on each other, and following these references helps provide comprehensive answers.

**GitHub links**: When you encounter GitHub URLs (issues, PRs, discussions, code links), use WebFetch to read them. These often contain important context like:
- Discussion PR comments with design rationale
- Related issues explaining motivation or constraints
- Code implementations referenced by the RFD

**External documentation**: For links to external resources (Wikipedia, specs, other documentation), fetch them if they're relevant to understanding the topic.

**When to follow links**:
- Always follow internal RFD references when they're directly relevant to the user's question
- Fetch GitHub discussion PRs when the user wants to understand the "why" behind a design
- Follow 2-3 levels deep if needed to answer the question thoroughly
- Use judgment - don't fetch every link, focus on those that add meaningful context

This approach mirrors how an engineer would research a topic: start with the primary RFD, then open related documents in new tabs to build understanding.

## Writing Style: Learning from Oxide RFDs

When presenting RFD content or synthesizing information from multiple RFDs, adopt the distinctive writing style evident in Oxide's engineering documents:

### Overall Approach: Narrative, Not Vignettes

**Your goal is to educate, not to compile quotes.**

Write in a fluid, coherent, conversational style that seeks to help the reader understand:
- **The WHY**: What problem is being solved? What are the constraints and motivations?
- **The HOW**: What approach was chosen? What reasoning led to this choice?
- **The WHAT NEXT**: What are the implications? What remains to be done?

**Use quotes to illuminate, not to replace narrative.** Quotes should:
- Provide evidence for points you're making
- Capture particularly clear or memorable articulations
- Show the reasoning process in the engineers' own words
- Reveal philosophy or principles

**Avoid vignetted quotes** - don't present a series of disconnected quotations with minimal connective tissue. Instead, weave quotes into a flowing explanation that builds understanding progressively.

### Core Principles

**Honest About Tradeoffs**
- RFDs explicitly acknowledge what was NOT chosen and why
- They're transparent about limitations and compromises
- Quote passages that show this honesty (e.g., "This is not perfect, however...")
- Example from RFD 216: "Failing into a closed state is generally a good design goal, however failing in such a way that remediation requires physical intervention in all but the most extreme cases is not acceptable."

**Pragmatic Over Perfect**
- Oxide consistently chooses operational viability over theoretical purity
- Security decisions balance protection with recovery capability
- Look for phrases like "we prefer", "for these reasons", "while X would be ideal, Y is more practical"
- Capture the reasoning behind pragmatic choices, not just the choices themselves

**Philosophical Grounding**
- Many RFDs start with Goals and Non-Goals sections
- They articulate *why* before diving into *how*
- Quote philosophical statements that reveal engineering values
- Example: Customer sovereignty, operational pragmatism, defense in depth without brittleness

**Determinations Matter**
- Look for "Determinations" sections - these are the actual decisions
- These capture what was ultimately chosen after exploring alternatives
- Always include determinations when summarizing RFD outcomes
- They represent the "answer" after the exploration phase

**Security Realism, Not Theater**
- RFDs are honest about which attacks are prevented and which remain possible
- They acknowledge that perfect security is impossible
- Quote passages that show threat model realism
- Example from RFD 36 on replay attacks: "This approach is by no means perfect, however it raises the bar on attackers significantly"

**Acknowledge Uncertainty**
- RFDs often have "Open Questions" or "Future Work" sections
- They're honest about what isn't fully figured out yet
- Include these acknowledgments - they show intellectual honesty
- Don't present uncertain things as certain

**Show Evolution of Thinking**
- When RFDs reference earlier approaches that didn't work, include that context
- Explain what was tried, what was learned, what changed
- This historical context enriches understanding
- Example: RFD 303's discussion of why DeviceId alone wasn't sufficient

**User Impact Focus**
- RFDs constantly reference how decisions affect customers, operators, and users
- Quote passages about customer sovereignty and operational concerns
- Highlight when design choices prioritize user freedom or operational reality
- Example: RFD 216 on why verified boot is minimal - to preserve customer control

### Stylistic Elements

**Conversational Yet Precise**
- RFDs ask rhetorical questions ("But how?", "Why would you believe that?")
- They use analogies and examples to explain complex concepts
- Maintain technical accuracy while being readable
- Don't be afraid of informal phrasing when it aids clarity

**Memorable Phrases**
- RFDs contain quotable passages that capture philosophy succinctly
- These are the passages worth including in block quotes
- Look for sentences that encapsulate key insights or principles
- They often contain words like "fundamentally", "critically", "importantly"

**Explicit Structure**
- Many RFDs use numbered lists for requirements or constraints
- They separate concerns clearly (Goals vs Non-Goals, Security vs Resilience)
- Use this structure when presenting information
- Make tradeoffs explicit with clear enumeration

**Technical Depth with Context**
- RFDs don't just state decisions - they explain the reasoning
- Include the "why" behind technical choices
- Quote passages that show the reasoning process
- Example: Why HKDF with SHA3-256? Why not just SHA-256?

### What to Avoid

- Don't sanitize or oversimplify the tradeoffs RFDs explicitly acknowledge
- Don't present determinations without the reasoning that led to them
- Don't skip over the philosophical/principled statements to jump to technical details
- Don't omit acknowledgments of limitations or open questions
- Don't make RFDs sound more certain than they actually are
- Avoid corporate/marketing speak - RFDs are engineer-to-engineer communication

### When Synthesizing Multiple RFDs

- Show how thinking evolved across RFDs (chronologically when relevant)
- Highlight when later RFDs reference or build on earlier ones
- Capture the architectural vision that emerges across documents
- Quote from multiple RFDs to show consistency in philosophy
- Note when approaches changed and why (learning, new constraints, etc.)

This approach mirrors how an engineer would research a topic: start with the primary RFD, then open related documents in new tabs to build understanding.

## Troubleshooting

### "No token configured" or authentication errors
Run `rfd-cli auth login google` to re-authenticate.

### Command not found
Install rfd-cli with:
```bash
cargo install --git https://github.com/oxidecomputer/rfd-api.git rfd-cli
```

### Permission denied on specific RFDs
Some RFDs are private. You can only access RFDs your account has permission to view.

### Unexpected argument errors
Most commands require named flags. Use `rfd-cli <command> --help` to check syntax.
Common pattern: `--number <N>` for RFD number, `--q "<query>"` for search.

## Permission Configuration

For smoother operation, users can add these to their Claude Code settings (`~/.claude/settings.json`):

```json
{
  "permissions": {
    "allow": [
      "Bash(rfd-cli search:*)",
      "Bash(rfd-cli meta:*)",
      "Bash(rfd-cli view:*)",
      "Bash(rfd-cli list:*)",
      "Bash(rfd-cli attr:*)",
      "Bash(rfd-cli discussion:*)",
      "Bash(rfd-cli revision:*)"
    ]
  }
}
```

This allows read-only rfd-cli commands to run without individual approval prompts.
