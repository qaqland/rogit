use super::{contributor, datetime, message};
use anyhow::Result;
use core::str;
use git2::Commit;
use indoc::indoc;
use rusqlite::{named_params, OptionalExtension, Transaction};

pub fn get_id(tx: &Transaction, hash: &str, repo_id: i64) -> Result<Option<i64>> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
        SELECT "id" FROM "commits"
        WHERE "hash" = :hash AND "repo" = :repo
        LIMIT 1
        "# })?;
    let id = stmt
        .query_row(named_params! {":hash": hash, ":repo": repo_id}, |r| {
            r.get::<_, i64>(0)
        })
        .optional()?;
    Ok(id)
}

pub fn get_tree_by_id(tx: &Transaction, id: i64, repo_id: i64) -> Result<Option<String>> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
        SELECT "tree" FROM "commits"
        WHERE "id" = :id AND "repo" = :repo
        LIMIT 1
        "# })?;
    let tree_hash = stmt
        .query_row(named_params! {":id": id, ":repo": repo_id}, |r| {
            r.get::<_, String>(0)
        })
        .optional()?;
    Ok(tree_hash)
}

pub fn get_tree_by_hash(tx: &Transaction, hash: &str, repo_id: i64) -> Result<Option<String>> {
    let mut stmt = tx.prepare_cached(indoc! { r#"
        SELECT "tree" FROM "commits"
        WHERE "hash" = :hash AND "repo" = :repo
        LIMIT 1
        "# })?;
    let tree_hash = stmt
        .query_row(named_params! {":hash": hash, ":repo": repo_id}, |r| {
            r.get::<_, String>(0)
        })
        .optional()?;
    Ok(tree_hash)
}

pub fn insert(tx: &Transaction, obj: &Commit, repo_id: i64) -> Result<i64> {
    let hash = obj.id().to_string();

    // let id = get_id(tx, &hash, repo_id)?;

    // if let Some(_) = id {
    //     return Ok(None);
    // }

    let tree_id = obj.tree_id().to_string();
    let committer = contributor::get_id(tx, &obj.committer())?;
    let date = datetime::get_time(obj.time())?;
    let msg = obj.message().unwrap_or_default();
    let msg_id = message::insert(tx, msg, &obj.author(), repo_id)?;

    let mut stmt = tx.prepare_cached(indoc! { r#"
        INSERT INTO "commits" (hash, repo, tree, date, c7r_id, msg_id)
        VALUES (:hash, :repo, :tree, :date, :c7r_id, :msg_id)
        "# })?;
    let id = stmt.insert(named_params! {
        ":hash": hash,
        ":repo": repo_id,
        ":tree": tree_id,
        ":date": date,
        ":c7r_id": committer,
        ":msg_id": msg_id,
    })?;

    Ok(id)
}
