use anyhow::{Context, Result};
use core::str;
use git2::{Delta, Diff, DiffDelta, DiffFile};
use indoc::indoc;
use rusqlite::{named_params, Transaction};

pub fn insert(tx: &Transaction, relation_id: i64, diff: &Diff) -> Result<()> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
    INSERT INTO "changes" (diff, mode, file)
    VALUES (:diff, :mode, :file)
    "# })?;

    for delta in diff.deltas() {
        let path = get_path(&delta.new_file()).unwrap_or_default();
        let file_id =
            get_file(tx, &path).with_context(|| format!("Failed to query id: {}", path))?;
        let mode = get_mode(&delta);

        stmt.insert(named_params! {
            ":diff": relation_id,
            ":mode": mode,
            ":file": file_id,
        })?;
    }
    Ok(())
}

fn get_file(tx: &Transaction, path: &str) -> Result<i64> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
    INSERT OR IGNORE INTO "editfiles" (path)
    VALUES (:path)
    "# })?;
    stmt.execute(named_params! {":path": path})?;

    let mut stmt = tx.prepare_cached(indoc! { r#"
    SELECT "id" FROM "editfiles"
    WHERE path = :path
    LIMIT 1
    "# })?;
    let id = stmt.query_row(named_params! {":path": path}, |r| r.get::<_, i64>(0))?;
    Ok(id)
}

fn get_path(file: &DiffFile) -> Option<String> {
    let Some(path) = file.path() else {
        return None;
    };
    let Some(path_str) = path.to_str() else {
        return None;
    };
    Some(path_str.to_owned())
}

fn get_mode(delta: &DiffDelta) -> i64 {
    match delta.status() {
        Delta::Unmodified => 0,
        Delta::Added => 1,
        Delta::Deleted => 2,
        Delta::Modified => 3,
        Delta::Renamed => 4,
        Delta::Copied => 5,
        Delta::Ignored => 6,
        Delta::Untracked => 7,
        Delta::Typechange => 8,
        Delta::Unreadable => 9,
        Delta::Conflicted => 10,
    }
}
