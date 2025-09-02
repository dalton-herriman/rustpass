use rusqlite::{params, Connection, Result};

fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    // Safely build SQL using format! (make sure table_name is sanitized or controlled)
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            service TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            notes TEXT,                    
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        table_name
    );

    conn.execute(&sql, [])?;
    Ok(())
}

// Insert new record, create new row
fn insert_into_table(conn: &Connection, table_name: &str, service: &str, username: &str, password: &str, notes: Option<&str>) -> Result<()> {
    let sql = format!(
        "INSERT INTO {} (service, username, password, notes) VALUES (?1, ?2, ?3, ?4)",
        table_name
    );

    conn.execute(&sql, params![service, username, password, notes])?;
    Ok(())
}

// Update table, modify existing row
fn update_table(conn: &Connection, table_name: &str, id: i32, service: &str, username: &str, password: &str, notes: Option<&str>) -> Result<()> {
    let sql = format!(
        "UPDATE {} SET service = ?1, username = ?2, password = ?3, notes = ?4, updated_at = CURRENT_TIMESTAMP WHERE id = ?5",
        table_name
    );

    conn.execute(&sql, params![service, username, password, notes, id])?;
    Ok(())
}

fn main() -> Result<()> {
    // Open a connection to a file-based SQLite database
    let conn = Connection::open("my_database.sqlite3")?;

    // Try to create a base 'vault' table
    create_table(&conn, "vault")?;

    println!("Table created successfully!");
    Ok(())
}