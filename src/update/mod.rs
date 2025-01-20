use anyhow::{anyhow, Result};
use core::str;
use git2::{Commit, DiffOptions, Oid, Repository};
use rusqlite::Connection;
use rusqlite::Transaction;
use std::fs;

use crate::config::Config;
use crate::database;

pub fn run(c: &Config) -> Result<()> {
    let mut conn = c.open_db()?;
    for r in &c.repo {
        let repo = Repository::open_bare(&r.path)?;
        let repo_id = update_repository(&conn, &repo, &r.name)?;
        update_commit_all(&mut conn, &repo, repo_id)?;
        // branch_sync(&mut conn, &repo)?;
        // tag_sync(&mut conn, &repo)?;
    }
    database::cleanup(&conn)?;
    Ok(())
}

fn update_repository(conn: &Connection, repo: &Repository, repo_name: &str) -> Result<i64> {
    let head_ref = repo.head()?;
    let head_name = head_ref.shorthand().expect("head should have valid name");
    if !head_ref.is_branch() {
        return Err(anyhow!("head should be branch: {}", head_name));
    }
    let desc = fs::read_to_string(repo.path().join("description")).unwrap_or_default();
    database::repository::insert(conn, repo_name, &desc, head_name)?;
    let id = database::repository::get_id(conn, repo_name)?;
    Ok(id)
}

fn update_commit_all(conn: &mut Connection, repo: &Repository, repo_id: i64) -> Result<u32> {
    let mut count = 0;
    let ref_all = repo.references()?;
    for ref_one in ref_all.flatten() {
        let Ok(commit) = ref_one.peel_to_commit() else {
            continue;
        };
        count += update_commit_from(conn, repo, repo_id, commit.id())?;
    }
    Ok(count)
}

pub fn update_commit_from(
    conn: &mut Connection,
    repo: &Repository,
    repo_id: i64,
    hash: Oid,
) -> Result<u32> {
    let mut count = 0;
    let mut obj_vec: Vec<(Oid, Option<i64>)> = vec![(hash, None)];

    while let Some((oid, child_id)) = obj_vec.pop() {
        let c = repo.find_commit(oid)?;
        let id = update_commit_one(conn, &c, repo_id, child_id, repo)?;
        if id.is_none() {
            continue;
        }
        count += 1;
        for p in c.parent_ids() {
            obj_vec.push((p, id));
        }
    }

    Ok(count)
}

fn update_commit_one(
    conn: &mut Connection,
    obj: &Commit,
    repo_id: i64,
    child_id: Option<i64>,
    repo: &Repository,
) -> Result<Option<i64>> {
    let tx = conn.transaction()?;
    let hash = obj.id().to_string();

    match child_id {
        Some(child_id) => {
            let exist_id = database::commit::get_id(&tx, &hash, repo_id)?;
            match exist_id {
                Some(id) => {
                    let diff_id = database::relation::get_id(&tx, id, child_id, repo_id)?;
                    match diff_id {
                        Some(_) => {
                            // stop, commit already have this child
                            return Ok(None);
                        }
                        None => {
                            // new child
                            // insert relations
                            let diff_id = database::relation::insert(&tx, id, child_id, repo_id)?;
                            // insert changes
                            update_change(&tx, obj, child_id, diff_id, repo_id, repo)?;
                            tx.commit()?;
                            // still stop, we have fixed relationship
                            return Ok(None);
                        }
                    }
                }
                None => {
                    // insert new commit
                    let id = database::commit::insert(&tx, obj, repo_id)?;
                    // insert relations
                    let diff_id = database::relation::insert(&tx, id, child_id, repo_id)?;
                    // insert changes
                    update_change(&tx, obj, child_id, diff_id, repo_id, repo)?;
                    tx.commit()?;
                    return Ok(Some(id));
                }
            }
        }
        None => {
            let exist_id = database::commit::get_id(&tx, &hash, repo_id)?;
            match exist_id {
                Some(_) => {
                    // stop, commit is exists and has no child
                    return Ok(None);
                }
                None => {
                    // insert new commit
                    let id = database::commit::insert(&tx, obj, repo_id)?;
                    tx.commit()?;
                    return Ok(Some(id));
                }
            }
        }
    }
}

fn update_change(
    tx: &Transaction,
    obj: &Commit,
    child_id: i64,
    relation_id: i64,
    repo_id: i64,
    repo: &Repository,
) -> Result<i64> {
    // get tree obj from commit obj
    let old_tree = obj.tree()?;
    // get tree_id from child_id and tree obj from repo
    let new_tree_id = database::commit::get_tree_by_id(tx, child_id, repo_id)?;
    let new_tree = match new_tree_id {
        Some(hash) => {
            let tree = repo.find_tree(Oid::from_str(&hash)?)?;
            Some(tree)
        }
        None => None,
    };
    // get diff from repo
    let mut options = DiffOptions::new();
    options.skip_binary_check(true).force_binary(true);
    let diff = repo.diff_tree_to_tree(Some(&old_tree), new_tree.as_ref(), Some(&mut options))?;
    // insert changs
    database::change::insert(tx, relation_id, &diff)?;
    Ok(0)
}

// pub fn update_branch(conn: &mut Connection, repo: &Repository) -> Result<()> {
//     let mut tx = conn.transaction()?;

//     let branches = repo.branches(Some(BranchType::Local))?;
//     for (branch, _) in branches.flatten() {
//         let Some(name) = branch.name()? else {
//             // warn: name is not valid utf-8
//             continue;
//         };

//         let reference = branch.get();
//         let Some(commit) = reference.resolve()?.target() else {
//             // error: branch target is None?
//             continue;
//         };
//         let old_commit_option = database::branch::commit(&mut tx, name)?;
//         if let Some(old_commit) = old_commit_option {
//             // branch is new, skip
//             if old_commit == commit.to_string() {
//                 continue;
//             }
//             // update with new commit
//             database::branch::upsert(&mut tx, name, &commit)?;
//         } else {
//             // insert new branch
//             database::branch::upsert(&mut tx, name, &commit)?;
//         };
//     }
//     // delete temp_branch that on longer exists
//     database::branch::clear(&mut tx)?;
//     tx.commit()?;
//     Ok(())
// }

// pub fn update_tag(conn: &mut Connection, repo: &Repository) -> Result<()> {
//     let mut tags: Vec<(Oid, String)> = Vec::new();
//     repo.tag_foreach(|tag_id, name_u8| {
//         let Ok(full_name) = str::from_utf8(name_u8) else {
//             return true;
//         };
//         let short_name = full_name.strip_prefix("refs/tags/").unwrap_or(full_name);
//         tags.push((tag_id, short_name.to_string()));
//         true
//     })?;
//     let mut tx = conn.transaction()?;
//     for (oid, name) in tags {
//         let old_tag_target_option = database::tag::target(&mut tx, &name)?;
//         let target_obj = repo.find_object(oid, None)?;
//         if let Some(old_tag_target) = old_tag_target_option {
//             if old_tag_target == oid.to_string() {
//                 continue;
//             }
//             database::tag::upsert(&mut tx, &name, &target_obj)?;
//         } else {
//             database::tag::upsert(&mut tx, &name, &target_obj)?;
//         }
//     }
//     database::tag::clear(&mut tx)?;
//     tx.commit()?;
//     Ok(())
// }
