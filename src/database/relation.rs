use anyhow::Result;
use core::str;
use indoc::indoc;
use rusqlite::{named_params, OptionalExtension, Transaction};

pub fn get_id(
    tx: &Transaction,
    parent_id: i64,
    child_id: i64,
    repo_id: i64,
) -> Result<Option<i64>> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
    SELECT "id" FROM "relations"
    WHERE "repo" = :repo AND "child" = :child AND "parent" = :parent
    LIMIT 1
    "# })?;
    let id = stmt
        .query_row(
            named_params! {
                ":repo": repo_id,
                ":child": child_id,
                ":parent": parent_id,
            },
            |r| r.get::<_, i64>(0),
        )
        .optional()?;
    Ok(id)
}

pub fn insert(
    tx: &Transaction,
    parent_id: i64,
    child_id: i64,
    repo_id: i64,
) -> Result<i64> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
    INSERT INTO "relations" (repo, child, parent)
    VALUES (:repo, :child, :parent)
    "# })?;
    let id = stmt.insert(named_params! {
        ":repo": repo_id,
        ":child": child_id,
        ":parent": parent_id,
    })?;

    Ok(id)
}

