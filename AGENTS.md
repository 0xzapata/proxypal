# ProxyPal Development Guidelines

## Commands

- **Dev**: `pnpm dev` (Vite) or `pnpm tauri dev` (Tauri app)
- **Build**: `pnpm build` or `pnpm tauri build`
- **Type check**: `pnpm tsc --noEmit`
- **Rust check**: `cd src-tauri && cargo check`

## Code Style

### TypeScript (Frontend - SolidJS)

- Use functional components with arrow functions for handlers
- Props interfaces: `interface ComponentProps { ... }` directly above component
- Imports: external libs → internal aliases (`../lib`, `../stores`) → relative (`./ui`)
- Use `type` imports for type-only: `import type { Provider } from "../lib/tauri"`
- Signals: `const [value, setValue] = createSignal(initial)`
- Tailwind for styling; use `class` not `className`

### Rust (Backend - Tauri)

- Structs: derive `Serialize, Deserialize` for IPC types
- Use `#[tauri::command]` for exposed functions
- State: wrap in `Mutex<T>`, access via `State<AppState>`
- camelCase for JSON fields via `#[serde(rename = "camelCase")]`
- Error handling: return `Result<T, String>` from commands

<!-- bv-agent-instructions-v1 -->

---

## Beads Workflow Integration

This project uses [beads_viewer](https://github.com/Dicklesworthstone/beads_viewer) for issue tracking. Issues are stored in `.beads/` and tracked in git.

### Essential Commands

```bash
# View issues (launches TUI - avoid in automated sessions)
bv

# CLI commands for agents (use these instead)
bd ready              # Show issues ready to work (no blockers)
bd list --status=open # All open issues
bd show <id>          # Full issue details with dependencies
bd create --title="..." --type=task --priority=2
bd update <id> --status=in_progress
bd close <id> --reason="Completed"
bd close <id1> <id2>  # Close multiple issues at once
bd sync               # Commit and push changes
```

### Workflow Pattern

1. **Start**: Run `bd ready` to find actionable work
2. **Claim**: Use `bd update <id> --status=in_progress`
3. **Work**: Implement the task
4. **Complete**: Use `bd close <id>`
5. **Sync**: Always run `bd sync` at session end

### Key Concepts

- **Dependencies**: Issues can block other issues. `bd ready` shows only unblocked work.
- **Priority**: P0=critical, P1=high, P2=medium, P3=low, P4=backlog (use numbers, not words)
- **Types**: task, bug, feature, epic, question, docs
- **Blocking**: `bd dep add <issue> <depends-on>` to add dependencies

### Session Protocol

**Before ending any session, run this checklist:**

```bash
git status              # Check what changed
git add <files>         # Stage code changes
bd sync                 # Commit beads changes
git commit -m "..."     # Commit code
bd sync                 # Commit any new beads changes
git push                # Push to remote
```

### Best Practices

- Check `bd ready` at session start to find available work
- Update status as you work (in_progress → closed)
- Create new issues with `bd create` when you discover tasks
- Use descriptive titles and set appropriate priority/type
- Always `bd sync` before ending session

<!-- end-bv-agent-instructions -->
