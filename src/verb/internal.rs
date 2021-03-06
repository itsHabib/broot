//! declare the internal functions which may be used in verbs.
//! They don't take any user argument other than the selection
//! (this may change if the needs arise).
//! They can be called as ":some_name" from builtin verbs and
//! from configured verbs.

use crate::errors::ConfError;

macro_rules! Internals {
    (
        $($name:ident: $description:expr,)*
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum Internal {
            $($name,)*
        }
        impl Internal {
            pub fn try_from(verb: &str) -> Result<Internal, ConfError> {
                use Internal::*;
                match verb {
                    $(stringify!($name) => Ok($name),)*
                    _ => Err(ConfError::UnknownInternal{ verb: verb.to_string() }),
                }
            }
        }
        impl Internal {
            pub fn name(&self) -> &'static str {
                use Internal::*;
                match self {
                    $($name => stringify!($name),)*
                }
            }
            pub fn description(&self) -> &'static str {
                use Internal::*;
                match self {
                    $($name => $description,)*
                }
            }
        }
    }
}

Internals! {
    back: "revert to the previous state (mapped to *esc*)",
    close_panel_ok: "close the current panel, validating the selected path",
    close_panel_cancel: "close the current panel, not using the selected path",
    complete: "complete the current argument",
    focus: "display the directory (mapped to *enter*)",
    help: "display broot's help",
    line_down: "move one line down",
    line_up: "move one line up",
    open_stay: "open file or directory according to OS settings (stay in broot)",
    open_leave: "open file or directory according to OS settings (quit broot)",
    page_down: "scroll one page down",
    page_up: "scroll one page up",
    parent: "move to the parent directory",
    print_path: "print path and leaves broot",
    print_relative_path: "print relative path and leaves broot",
    print_tree: "print tree and leaves broot",
    start_end_panel: "ok, I need to explain this in a few words",
    quit: "quit Broot",
    refresh: "refresh tree and clear size cache",
    select_first: "select the first file",
    select_last: "select the last file",
    toggle_dates: "toggle showing last modified dates",
    toggle_files: "toggle showing files (or just folders)",
    toggle_git_ignore: "toggle use of .gitignore",
    toggle_git_file_info: "toggle display of git file information",
    toggle_git_status: "toggle showing only files relevant for git status",
    toggle_hidden: "toggle showing hidden files",
    toggle_perm: "toggle showing file permissions",
    toggle_sizes: "toggle showing sizes",
    toggle_trim_root: "toggle removing nodes at first level too",
    total_search: "search again but on all children",
    up_tree: "focus the parent of the current root",
}

impl Internal {
    /// whether this internal accept a path as (optional) argument
    pub fn accept_path(self) -> bool {
        match self {
            Internal::focus => true,
            _ => false,
        }
    }
}
