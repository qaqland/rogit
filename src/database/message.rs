use super::{contributor, datetime};
use anyhow::Result;
use git2::Signature;
use indoc::indoc;
use rusqlite::{named_params, Transaction};

pub fn insert(tx: &Transaction, msg: &str, sig: &Signature, repo_id: i64) -> Result<i64> {
    let date = datetime::get_time(sig.when())?;
    let author_id = contributor::get_id(tx, sig)?;
    let mut stmt = tx.prepare_cached(indoc! { r#"
        INSERT INTO "messages" (repo, author, msg, date)
        VALUES (:repo, :author, :msg, :date)
    "# })?;
    let id = stmt.insert(named_params! {
        ":repo": repo_id,
        ":author": author_id,
        ":msg": msg,
        ":date": date,
    })?;
    Ok(id)
}
