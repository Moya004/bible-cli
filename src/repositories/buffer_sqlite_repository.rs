use crate::{business::repositories::BufferRepository, constants::cons::project_root};
use array_deque::ArrayDeque as Deque;
use rusqlite::Connection;

pub struct BufferSqliteRepository {
    connection: Connection,
}

impl BufferSqliteRepository {
    pub fn new() -> Option<Self> {
        let r#try = Connection::open(project_root().join("./database.sqlite3"));

        match r#try {
            Ok(success) => Some(BufferSqliteRepository {
                connection: success,
            }),
            Err(err) => {
                println!("Error conectando a la base de datos, {}", err);
                None
            }
        }
    }
}

impl BufferRepository for BufferSqliteRepository {
    fn load_history(&self) -> Result<Vec<String>, rusqlite::Error> {
        todo!()
    }

    fn save_history(&self, history: Deque<String>) -> Result<(), rusqlite::Error> {
        let mut stm = self.connection.prepare(
            "INSERT INTO BUFFER_HISTORICO (contenido, fecha) VALUES (?1, datetime('now', 'localtime'))",
        )?;

        for entry in history {
            stm.execute([entry])?;
        }
        Ok(())
    }
}
