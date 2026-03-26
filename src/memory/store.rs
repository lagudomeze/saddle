use crate::SaddleResult;
use exn::ResultExt;
use rusqlite::Connection;

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn new(path: &str) -> SaddleResult<Self> {
        let conn = Connection::open(path)
            .or_raise(|| crate::SaddleError::Memory(format!("Failed to open database at: {}", path)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                embedding BLOB,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .or_raise(|| crate::SaddleError::Memory("Failed to create memories table".into()))?;

        Ok(Self { conn })
    }

    pub fn insert(&self, content: &str, embedding: Option<&[f32]>) -> SaddleResult<()> {
        let blob = embedding.map(|e| {
            let mut bytes = Vec::with_capacity(e.len() * 4);
            for f in e {
                bytes.extend_from_slice(&f.to_le_bytes());
            }
            bytes
        });
        self.conn.execute(
            "INSERT INTO memories (content, embedding) VALUES (?1, ?2)",
            (content, blob),
        ).or_raise(|| crate::SaddleError::Memory("Failed to insert memory".into()))?;
        Ok(())
    }

    pub fn search(&self, _query: &str, _limit: usize) -> SaddleResult<Vec<String>> {
        Ok(vec![])
    }
}
