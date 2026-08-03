## Claude Code Skill

This repository includes a [Claude Code](https://claude.ai/claude-code) skill that enables AI-assisted research of Oxide RFDs. The skill provides read-only access to search, view, and analyze RFDs using `rfd-cli`.

### Features

- First-time setup walkthrough (install, configure, authenticate)
- All read-only CLI commands with correct syntax
- 53 curated foundational RFDs organized by category
- Guidance on following links and cross-references
- Research strategies for simple lookups vs comprehensive research

### Installation

The skill file is located at `.claude/skills/rfd-api/SKILL.md`. To make it available globally across all projects:

```sh
mkdir -p ~/.claude/skills/rfd-api
cp .claude/skills/rfd-api/SKILL.md ~/.claude/skills/rfd-api/SKILL.md
```

### Optional: Auto-approve read-only commands

Add these to your Claude Code settings (`~/.claude/settings.json`) to allow read-only `rfd-cli` commands without individual approval prompts:

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
