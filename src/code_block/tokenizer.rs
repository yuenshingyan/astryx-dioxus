#[derive(Clone, Copy)]
pub(super) enum DiffLineKind {
    Context,
    Added,
    Removed,
    Header,
}

impl DiffLineKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Header => "header",
        }
    }
}

pub(super) fn classify_diff_line(lines: &[&str], index: usize) -> DiffLineKind {
    let line = lines[index];
    let is_file_header = |i| {
        matches!(lines.get(i..i + 3), Some([old, new, hunk])
            if old.starts_with("--- ") && new.starts_with("+++ ") && hunk.starts_with("@@"))
    };
    if line.starts_with("@@") || is_file_header(index) || (index > 0 && is_file_header(index - 1)) {
        DiffLineKind::Header
    } else if line.starts_with('+') {
        DiffLineKind::Added
    } else if line.starts_with('-') {
        DiffLineKind::Removed
    } else {
        DiffLineKind::Context
    }
}
