use zed_extension_api::{self as zed, WorkspaceCommand, WorkspaceCommandResult, Worktree};

/// Known companion file groups.
///
/// When the user invokes "Switch to Companion File", the extension:
/// 1. Detects which group the active file belongs to (by matching its suffix).
/// 2. Finds the next file in that group that actually exists on disk.
/// 3. Returns `WorkspaceCommandResult::OpenFile` for a single match or
///    `WorkspaceCommandResult::PickAndOpen` when multiple candidates exist.
const COMPANION_GROUPS: &[&[&str]] = &[
    // Blazor / Razor
    &[".razor", ".razor.cs", ".razor.css"],
    // Angular
    &[
        ".component.ts",
        ".component.html",
        ".component.scss",
        ".component.css",
        ".component.spec.ts",
    ],
    // Generic TypeScript + HTML + style
    &[".ts", ".html", ".css", ".scss"],
    // C / C++
    &[".h", ".hpp", ".c", ".cpp", ".cc"],
    // Swift
    &[".swift", ".xib", ".storyboard"],
    // Test ↔ impl (generic)
    &[".test.ts", ".ts"],
    &[".spec.ts", ".ts"],
    &["_test.go", ".go"],
    &["_test.rs", ".rs"],
];

struct FileExtSwitcher;

impl zed::Extension for FileExtSwitcher {
    fn new() -> Self {
        FileExtSwitcher
    }

    fn workspace_commands(&self) -> Vec<WorkspaceCommand> {
        vec![WorkspaceCommand {
            id: "switch-companion-file".into(),
            name: "Switch to Companion File".into(),
            description: Some(
                "Open the companion file with the same base name (cycles through known extension groups)".into(),
            ),
        }]
    }

    fn run_workspace_command(
        &self,
        command_id: String,
        active_file: Option<String>,
        worktree: Option<&Worktree>,
    ) -> zed::Result<WorkspaceCommandResult> {
        if command_id != "switch-companion-file" {
            return Err(format!("Unknown command: {command_id}"));
        }

        let active_path = active_file
            .as_deref()
            .ok_or("No active file to switch from")?;

        let candidates = find_companion_candidates(active_path, worktree);

        match candidates.len() {
            0 => Err(format!(
                "No companion files found for '{}'",
                file_name(active_path)
            )),
            1 => Ok(WorkspaceCommandResult::OpenFile(
                candidates.into_iter().next().unwrap(),
            )),
            _ => Ok(WorkspaceCommandResult::PickAndOpen(candidates)),
        }
    }
}

/// Returns the list of companion file paths that exist on disk.
fn find_companion_candidates(active_path: &str, worktree: Option<&Worktree>) -> Vec<String> {
    let Some(group) = find_group(active_path) else {
        return Vec::new();
    };

    let current_suffix = group
        .iter()
        .find(|&&s| active_path.ends_with(s))
        .copied()
        .unwrap_or("");

    let base = &active_path[..active_path.len() - current_suffix.len()];
    let dir = parent_dir(active_path);

    let mut candidates = Vec::new();

    for &suffix in group.iter() {
        if suffix == current_suffix {
            continue;
        }

        let candidate_path = format!("{base}{suffix}");

        if path_exists(&candidate_path, dir, worktree) {
            candidates.push(candidate_path);
        }
    }

    candidates
}

/// Find which companion group the given path belongs to.
fn find_group(path: &str) -> Option<&'static [&'static str]> {
    // Try longest suffix first to handle ".razor.cs" before ".cs"
    let mut best: Option<(&'static [&'static str], usize)> = None;

    for &group in COMPANION_GROUPS {
        for &suffix in group {
            if path.ends_with(suffix) {
                let len = suffix.len();
                if best.map_or(true, |(_, best_len)| len > best_len) {
                    best = Some((group, len));
                }
            }
        }
    }

    best.map(|(group, _)| group)
}

/// Check if a file path exists, using the worktree if available.
fn path_exists(abs_path: &str, dir: &str, worktree: Option<&Worktree>) -> bool {
    if let Some(wt) = worktree {
        let root = wt.root_path();
        if abs_path.starts_with(&root) {
            let rel = abs_path.trim_start_matches(&root).trim_start_matches('/');
            return wt.read_text_file(rel).is_ok();
        }
    }

    // Fallback: attempt to read the file via the extension process API.
    // Use std::fs since the extension runs in a WASI sandbox with filesystem access.
    std::fs::metadata(abs_path).is_ok()
}

fn parent_dir(path: &str) -> &str {
    path.rfind('/').map(|i| &path[..i]).unwrap_or("")
}

fn file_name(path: &str) -> &str {
    path.rfind('/').map(|i| &path[i + 1..]).unwrap_or(path)
}

zed_extension_api::register_extension!(FileExtSwitcher);
