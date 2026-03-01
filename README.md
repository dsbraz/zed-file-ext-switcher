# File Extension Switcher

A [Zed](https://zed.dev) extension that switches between companion files sharing the same base name but a different extension — similar to the VS Code [file-ext-switcher](https://marketplace.visualstudio.com/items?itemName=JohannesRudolph.file-ext-switcher) extension.

## Examples

| Active file | Companions found |
|---|---|
| `Counter.razor` | `Counter.razor.cs`, `Counter.razor.css` |
| `app.component.ts` | `app.component.html`, `app.component.scss` |
| `main.c` | `main.h` |
| `user_test.go` | `user.go` |

## Usage

Open the **command palette** and run **Switch to Companion File**.

For a keybinding, add to your `keymap.json`:

```json
{
  "context": "Workspace",
  "bindings": {
    "ctrl-alt-o": [
      "extensions::RunExtensionWorkspaceCommand",
      { "extension_id": "file-ext-switcher", "command_id": "switch-companion-file" }
    ]
  }
}
```

**Single match** — opens the companion file directly.
**Multiple matches** — opens a picker listing all candidates; type to filter, Enter to open, Escape to dismiss.
**No match** — displays an error notification.

## Supported extension groups

| Group | Extensions |
|---|---|
| Blazor / Razor | `.razor`, `.razor.cs`, `.razor.css` |
| Angular | `.component.ts`, `.component.html`, `.component.scss`, `.component.css`, `.component.spec.ts` |
| TypeScript / HTML / CSS | `.ts`, `.html`, `.css`, `.scss` |
| C / C++ | `.h`, `.hpp`, `.c`, `.cpp`, `.cc` |
| Swift | `.swift`, `.xib`, `.storyboard` |
| Test ↔ impl (TypeScript) | `.test.ts` / `.ts`, `.spec.ts` / `.ts` |
| Test ↔ impl (Go) | `_test.go` / `.go` |
| Test ↔ impl (Rust) | `_test.rs` / `.rs` |

Matching uses longest-suffix-first priority, so `.razor.cs` is recognised before `.cs`.

Only files that exist on disk are returned as candidates.

## Requirements

Requires Zed with the `workspace_command` extension API (v0.8.0), introduced in [zed-industries/zed#50449](https://github.com/zed-industries/zed/pull/50449).

## Installation

Once the extension is available in the Zed extension registry:

1. Open the **Extensions** panel (`zed: extensions`)
2. Search for **File Extension Switcher**
3. Click **Install**

## License

MIT
