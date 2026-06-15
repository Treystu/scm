# Scratch Notes

## Migration Notes (2026-06-13)
- Migrated from Claude Code → MiMo Code
- Claude Code config preserved at ~/.claude/settings.json
- MiMo Code global config at ~/.config/mimocode/mimocode.json
- OpenRouter API key shared between both (env var OPENROUTER_API_KEY)
- mimo binary at /Users/scmessenger/.hermes/node/bin/mimo (added to .zshrc PATH)

## Workspace Architecture
- SCMessenger_Clean = production workspace (gatekeeper approved only)
- SCMessenger = working workspace (consolidation/pruning)
- MiMo Code workspace tied to SCMessenger_Clean — deleting it removes everything
