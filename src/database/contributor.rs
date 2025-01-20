use anyhow::Result;
use git2::Signature;
use indoc::indoc;
use rusqlite::{named_params, OptionalExtension, Transaction};

pub fn get_id(tx: &Transaction, sig: &Signature) -> Result<i64> {
    let name = sig.name();
    let mail = sig.email();

    let mut stmt = tx.prepare_cached(indoc! { r#"
        SELECT "id" FROM "contributors"
        WHERE "name" = :name AND "mail" = :mail
        LIMIT 1
        "# })?;
    let id = stmt
        .query_row(named_params! {":name": name, ":mail": mail}, |r| {
            r.get::<_, i64>(0)
        })
        .optional()?;
    if let Some(id) = id {
        Ok(id)
    } else {
        let mut stmt = tx.prepare_cached(indoc! { r#"
            INSERT INTO "contributors" (name, mail)
            VALUES (:name, :mail)
            "# })?;
        let id = stmt.insert(named_params! {":name": name, ":mail": mail})?;
        Ok(id)
    }
}
