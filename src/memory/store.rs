use exn::Result;
use exn::ResultExt;
use rusqlite::Connection;

#[derive(Debug)]
pub struct MemoryError {
    message: String,
}

impl MemoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MemoryError {}

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn new(path: &str) -> Result<Self, MemoryError> {
        let conn = Connection::open(path)
            .or_raise(|| MemoryError::new(format!("Failed to open database at: {}", path)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                embedding BLOB,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .or_raise(|| MemoryError::new("Failed to create memories table"))?;

        Ok(Self { conn })
    }

    pub fn insert(&self, content: &str, embedding: Option<&[f32]>) -> Result<(), MemoryError> {
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
        ).or_raise(|| MemoryError::new("Failed to insert memory"))?;
        Ok(())
    }

    pub fn search(&self, _query: &str, _limit: usize) -> Result<Vec<String>, MemoryError> {
        Ok(vec![])
    }
}
