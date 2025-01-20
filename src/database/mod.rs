use crate::config::{Config, Mode};
use anyhow::Result;
use indoc::indoc;
use rusqlite::{Connection, OpenFlags};

pub mod change;
pub mod commit;
pub mod contributor;
pub mod datetime;
pub mod message;
pub mod relation;
pub mod repository;
// pub mod branch;
// pub mod tag;

impl Config {
    pub fn open_db(&self) -> Result<Connection> {
        let db_name = self.path.join("rogit.db");
        let conn = match self.mode {
            Mode::Update => {
                let conn = Connection::open(db_name)?;
                init_table(&conn)?;
                conn
            }
            Mode::Server => {
                let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
                Connection::open_with_flags(db_name, flags)?
            }
        };
        Ok(conn)
    }
}

fn init_table(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "synchronous", "FULL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "journal_mode", "MEMORY")?;

    conn.execute_batch(indoc! {r#"
        CREATE TABLE IF NOT EXISTS "repositories" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL UNIQUE,
            show    TEXT,
            head    TEXT NOT NULL,
            fake    INTEGER DEFAULT 0   -- used to delete expired
        ) STRICT;

        UPDATE "repositories" SET fake = 1;

        CREATE TABLE IF NOT EXISTS contributors (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            mail    TEXT NOT NULL,
            UNIQUE(name, mail)
        ) STRICT;

        CREATE TABLE IF NOT EXISTS "messages" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            repo    INTEGER NOT NULL,
            author  INTEGER NOT NULL,
            msg     TEXT,
            date    TEXT,               -- same as date in commit
            FOREIGN KEY(repo) REFERENCES repositories(id) ON DELETE CASCADE,
            FOREIGN KEY(author) REFERENCES contributors(id)
        ) STRICT;

        CREATE TABLE IF NOT EXISTS "commits" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            hash    TEXT NOT NULL,
            repo    INTEGER NOT NULL,
            tree    TEXT NOT NULL,
            date    TEXT,               -- commit date
            c7r_id  INTEGER NOT NULL,   -- committer
            msg_id  INTEGER NOT NULL,   -- think author write it
            FOREIGN KEY(repo) REFERENCES repositories(id) ON DELETE CASCADE,
            FOREIGN KEY(c7r_id) REFERENCES contributors(id),
            FOREIGN KEY(msg_id) REFERENCES messages(id),
            UNIQUE(repo, hash)
        ) STRICT;

        CREATE TABLE IF NOT EXISTS "relations" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            repo    INTEGER NOT NULL,
            child   INTEGER NOT NULL,
            parent  INTEGER NOT NULL,
            FOREIGN KEY(repo) REFERENCES repositories(id) ON DELETE CASCADE,
            FOREIGN KEY(child) REFERENCES commits(id) ON DELETE CASCADE,
            FOREIGN KEY(parent) REFERENCES commits(id) ON DELETE CASCADE,
            UNIQUE(repo, child, parent)
        ) STRICT;

        CREATE INDEX IF NOT EXISTS idx_relations_child ON relations(child);
        CREATE INDEX IF NOT EXISTS idx_relations_parent ON relations(parent);

        CREATE TABLE IF NOT EXISTS "editfiles" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            path    TEXT NOT NULL UNIQUE
        ) STRICT;

        CREATE TABLE IF NOT EXISTS "changes" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            diff    INTEGER NOT NULL,
            mode    INTEGER NOT NULL,
            file    INTEGER NOT NULL,
            FOREIGN KEY(diff) REFERENCES relations(id) ON DELETE CASCADE
            FOREIGN KEY(file) REFERENCES editfiles(id) ON DELETE CASCADE
        ) STRICT;

        CREATE TABLE IF NOT EXISTS "branches" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            hash    INTEGER NOT NULL,   -- target commit
            repo    INTEGER NOT NULL,
            fake    INTEGER DEFAULT 0,
            FOREIGN KEY(repo) REFERENCES repositories(id) ON DELETE CASCADE,
            FOREIGN KEY(hash) REFERENCES commits(id),
            UNIQUE(repo, name)
        ) STRICT;

        UPDATE "branches" SET fake = 1;

        CREATE TABLE IF NOT EXISTS "tags" (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            hash    INTEGER NOT NULL,   -- target commit
            repo    INTEGER NOT NULL,
            msg_id  INTEGER,            -- only tag object has
            fake    INTEGER DEFAULT 0,
            FOREIGN KEY(repo) REFERENCES repositories(id) ON DELETE CASCADE,
            FOREIGN KEY(hash) REFERENCES commits(id),
            FOREIGN KEY(msg_id) REFERENCES messages(id),
            UNIQUE(repo, name)
        ) STRICT;

        UPDATE "tags" SET fake = 1;

    "#})?;
    Ok(())
}

pub fn cleanup(conn: &Connection) -> Result<()> {
    conn.execute_batch(indoc! { r#"
        DELETE FROM "repositories" WHERE fake = 1;
        DELETE FROM "branches" WHERE fake = 1;
        DELETE FROM "tags" WHERE fake = 1;
        "# })?;
    Ok(())
}
