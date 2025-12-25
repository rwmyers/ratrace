use super::{Status, TodoItem};
use color_eyre::Result;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

lazy_static! {
    pub static ref DB_POOL: Pool<SqliteConnectionManager> = {
        if let Some(proj_dirs) = ProjectDirs::from("dev", "propria", "ratrace") {
            let db_dir = proj_dirs.data_dir();
            if !db_dir.exists() {
                std::fs::create_dir_all(db_dir).expect("failed to create db dir");
            }
            let db_path = db_dir.join("ratrace.db");
            let manager = SqliteConnectionManager::file(db_path);
            Pool::new(manager).expect("failed to create pool")
        } else {
            let manager = SqliteConnectionManager::memory();
            Pool::new(manager).expect("failed to create pool")
        }
    };
}

pub fn init_db() -> Result<()> {
    let conn = DB_POOL.get()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            status INTEGER NOT NULL,
            todo TEXT NOT NULL,
            info TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn get_all_todos() -> Result<Vec<TodoItem>> {
    let conn = DB_POOL.get()?;
    let mut stmt = conn.prepare("SELECT id, status, todo, info FROM todos")?;
    let todos = stmt
        .query_map([], |row| {
            let id = row.get(0)?;
            let status: u8 = row.get(1)?;
            let todo: String = row.get(2)?;
            let info: String = row.get(3)?;
            Ok(TodoItem::new(
                id,
                Status::try_from(status).unwrap(),
                &todo,
                &info,
            ))
        })?
        .collect::<Result<Vec<TodoItem>, _>>()?;
    Ok(todos)
}

pub fn add_todo(status: Status, todo: &str, info: &str) -> Result<()> {
    let conn = DB_POOL.get()?;
    conn.execute(
        "INSERT INTO todos (status, todo, info) VALUES (?1, ?2, ?3)",
        params![status as u8, todo, info],
    )?;
    Ok(())
}

pub fn update_status(id: i32, status: Status) -> Result<()> {
    let conn = DB_POOL.get()?;
    conn.execute(
        "UPDATE todos SET status = ?1 WHERE id = ?2",
        params![status as u8, id],
    )?;
    Ok(())
}

pub fn delete_todo(id: i32) -> Result<()> {
    let conn = DB_POOL.get()?;
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    Ok(())
}


