use crate::{
    business::repositories::BufferRepository,
    constants::{cons::project_root, types::BufferEntryVariation},
};
use array_deque::ArrayDeque as Deque;
use rusqlite::{Connection, Error, params};

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
    fn load_history(&self, limit: u16) -> Result<Vec<BufferEntryVariation>, Error> {
        let mut to_return = vec![];
        let mut sql = self
            .connection
            .prepare("SELECT * FROM BUFFER_HISTORICO bf ORDER BY bf.fecha limit ?1")?;

        let query_iter = match sql.query_map(params![limit], |row| {
            Ok(row.get_unwrap::<_, String>("contenido"))
        }) {
            Ok(r) => r,
            Err(error) => {
                println!("An error occured when trying to get translation: {}", error);
                return Err(error);
            }
        };

        for i in query_iter {
            match i {
                Ok(text) => to_return.push(BufferEntryVariation::Historic(text)),
                Err(_) => {}
            }
        }

        Ok(to_return)
    }

    fn save_history(&self, history: &Deque<BufferEntryVariation>) -> Result<(), Error> {
        let mut stm = self.connection.prepare(
            "INSERT INTO BUFFER_HISTORICO (contenido, fecha) VALUES (?1, datetime('now', 'localtime'))",
        )?;

        for entry in history {
            match entry {
                BufferEntryVariation::Historic(_) => {
                    continue;
                }
                BufferEntryVariation::New(_) => {}
            }
            match stm.execute([entry.to_string()]) {
                Ok(_) => {}
                Err(error) => return Err(error),
            };
        }
        Ok(())
    }
}
