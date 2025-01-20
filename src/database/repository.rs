use anyhow::Result;
use indoc::indoc;
use rusqlite::{named_params, Connection};

pub fn get_id(conn: &Connection, name: &str) -> Result<i64> {
    let id = conn.query_row(
        indoc! { r#"
        SELECT "id" FROM "repositories"
        WHERE "name" = :name
        LIMIT 1
        "# },
        named_params! {":name": name},
        |r| r.get::<_, i64>(0),
    )?;
    Ok(id)
}

pub fn insert(conn: &Connection, name: &str, show: &str, head: &str) -> Result<()> {
    conn.execute(
        indoc! { r#"
        INSERT INTO "repositories" (name, show, head, fake)
        VALUES (:name, :show, :head, 0)
        ON CONFLICT(name)
        DO UPDATE SET
            show = EXCLUDED.show,
            head = EXCLUDED.head,
            fake = 0;
        "# },
        named_params! {":name": name, ":show": show, ":head": head},
    )?;
    Ok(())
}
