use anyhow::Result;
use rusqlite::Connection;

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                embedding BLOB,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, content: &str, embedding: Option<&[f32]>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO memories (content, embedding) VALUES (?1, ?2)",
            (content, embedding),
        )?;
        Ok(())
    }

    pub fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>> {
        Ok(vec![])
    }
}
