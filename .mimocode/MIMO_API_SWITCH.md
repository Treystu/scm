# Switch to Xiaomi MiMo Direct API

## Problem
Current mimocode session uses OpenRouter (`https://openrouter.ai/api`) as the API backend.
All agents spawned by this session inherit OpenRouter — they do NOT use the MiMo direct endpoint.

## Solution
Restart mimocode with these environment variables:

```bash
export ANTHROPIC_BASE_URL="https://token-plan-sgp.xiaomimimo.com/v1"
export ANTHROPIC_AUTH_TOKEN="tp-scvxitmsxobro7uaiw2u6k5zlfwup90xamhb4nh29dwwxro7"
export OPENROUTER_API_KEY="tp-scvxitmsxobro7uaiw2u6k5zlfwup90xamhb4nh29dwwxro7"
unset ANTHROPIC_API_KEY
```

Then relaunch mimocode. All agents will route through `token-plan-sgp.xiaomimimo.com` instead of OpenRouter.

## Available Models
- `mimo-v2.5-pro` (primary — 4.1B credits)
- `mimo-v2.5`
- `mimo-v2-pro`
- `mimo-v2-omni`

## Verification
After restart, confirm with:
```bash
env | grep ANTHROPIC_BASE_URL
# Should show: https://token-plan-sgp.xiaomimimo.com/v1
```
