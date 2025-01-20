use anyhow::Result;
use core::str;
use git2::Object;
use log::debug;
use rusqlite::{named_params, OptionalExtension, Transaction};

use crate::database::person;

const SQL_INIT_CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS "tag" (
    "name"      TEXT PRIMARY KEY,
    "target_id" TEXT NOT NULL,
    "is_lw"     INTEGER DEFAULT 0,
    "message"   TEXT,
    "tagger"    INTEGER
) WITHOUT ROWID;

CREATE TEMPORARY TABLE "temp_tag" AS
SELECT
    "name"
FROM
    "tag";
"#;

pub fn init(tx: &mut Transaction) -> Result<()> {
    tx.execute_batch(SQL_INIT_CREATE)?;
    Ok(())
}

const SQL_TARGET_SELECT: &str = r#"
SELECT "target_id" FROM "tag" WHERE "name" = :name LIMIT 1;
"#;

const SQL_TARGET_DELETE: &str = r#"
DELETE FROM "temp_tag" WHERE "name" = :name;
"#;

pub fn target(tx: &mut Transaction, name: &str) -> Result<Option<String>> {
    let sp = tx.savepoint()?;

    let mut stmt_delete = sp.prepare_cached(SQL_TARGET_DELETE)?;
    stmt_delete.execute(named_params! {":name": name})?;
    drop(stmt_delete);

    let mut stmt_select = sp.prepare_cached(SQL_TARGET_SELECT)?;
    let commit = stmt_select
        .query_row(named_params! {":name": name}, |r| r.get::<_, String>(0))
        .optional()?;
    drop(stmt_select);

    sp.commit()?;
    Ok(commit)
}

const SQL_UPSERT: &str = r#"
INSERT INTO
    "tag" (
        "name",
        "target_id",
        "is_lw",
        "message",
        "tagger"
    )
VALUES
    (:name, :target_id, :is_lw, :message, :tagger) ON CONFLICT("name") DO
UPDATE
SET
    "target_id" = :target_id,
    "is_lw" = :is_lw,
    "message" = :message,
    "tagger" = :tagger;
"#;

pub fn upsert(tx: &mut Transaction, name: &str, target_obj: &Object) -> Result<()> {
    debug!(target: "tag", "insert {}", name);

    let mut sp = tx.savepoint()?;
    let mut is_lw = true;
    let mut message = "";
    let mut tagger = 0;
    if let Some(tag) = target_obj.as_tag() {
        is_lw = false;
        message = tag.message().unwrap_or_default();
        if let Some(s) = tag.tagger() {
            tagger = person::sync(&mut sp, &s)?
        }
    } else if let Some(commit) = target_obj.as_commit() {
        message = commit.message().unwrap_or_default();
        tagger = person::sync(&mut sp, &commit.committer())?;
    } else {
        // tag on tree?
    }

    let mut stmt = sp.prepare_cached(SQL_UPSERT)?;
    stmt.execute(named_params! {
        ":name": name,
        ":target_id": target_obj.id().to_string(),
        ":is_lw": is_lw,
        ":message": message,
        ":tagger": tagger,
    })?;

    drop(stmt);

    sp.commit()?;
    Ok(())
}

const SQL_CLEAR: &str = r#"
DELETE FROM
    "tag"
WHERE
    "name" IN (
        SELECT
            "name"
        FROM
            "temp_tag"
    );
"#;

pub fn clear(tx: &mut Transaction) -> Result<()> {
    let sp = tx.savepoint()?;
    let mut stmt = sp.prepare_cached(SQL_CLEAR)?;
    let count = stmt.execute(())?;

    drop(stmt);
    sp.commit()?;

    debug!(target: "tag", "remove {}", count);
    Ok(())
}
