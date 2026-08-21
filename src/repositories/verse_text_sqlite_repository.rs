use crate::{
    business::{
        domain::{Book, Cite, Query, Translation},
        repositories::VerseRepository,
    },
    constants::{
        cons::project_root,
        types::{IndexVariation, Passage},
    },
};

use rusqlite::{Connection, Error};

pub struct VerseTextSqliteRepository {
    connection: Connection,
}

impl VerseTextSqliteRepository {
    pub fn new() -> Option<Self> {
        let r#try = Connection::open(project_root().join("./database.sqlite3"));

        // Sin `println!`: la interfaz toma la pantalla en modo directo y
        // cualquier escritura suelta a stdout rompe el fotograma. Los errores
        // viajan de vuelta y se muestran en la barra de estado.
        r#try.ok().map(|connection| VerseTextSqliteRepository { connection })
    }

    fn translation(&self, code: &str) -> Result<Translation, Error> {
        self.connection.query_one(
            "SELECT * FROM TRADUCCIONES t WHERE t.codigo = ?1;",
            (code.to_uppercase(),),
            |row| {
                Ok(Translation {
                    id: row.get_unwrap("id"),
                    code: row.get_unwrap("codigo"),
                    name: row.get_unwrap("nombre"),
                })
            },
        )
    }
}

impl VerseRepository for VerseTextSqliteRepository {
    fn get_text(&self, query: &Query) -> Result<Passage, Error> {
        let traduction = match self.connection.query_one(
            "SELECT * FROM TRADUCCIONES t WHERE t.codigo = ?1;",
            (&query.translation.to_uppercase(),),
            |row| {
                Ok(Translation {
                    id: row.get_unwrap("id"),
                    code: row.get_unwrap("codigo"),
                    name: row.get_unwrap("nombre"),
                })
            },
        ) {
            Ok(t) => t,
            Err(error) => return Err(error),
        };

        let chapter_constraint = match &query.chapters {
            IndexVariation::Single(unique) => {
                format!("capitulo = {}", unique)
            }
            IndexVariation::List(range) => {
                format!("capitulo BETWEEN {} AND {}", range.start(), range.end())
            }
        };

        let verse_constraint = match &query.verses {
            IndexVariation::Single(unique) => {
                format!("verso = {}", unique)
            }
            IndexVariation::List(range) => {
                format!("verso BETWEEN {} AND {}", range.start(), range.end())
            }
        };
        let book_constraint = format!("libro = '{}'", &query.book);
        let translation_constraint = format!("translation_id = {}", &traduction.id);

        let mut sql = self
            .connection
            .prepare(
                [
                    String::from("SELECT * FROM TEXTO_VERSO WHERE"),
                    [
                        book_constraint,
                        chapter_constraint,
                        verse_constraint,
                        translation_constraint,
                    ]
                    .join(" AND "),
                ]
                .join(" ")
                .as_str(),
            )
            .unwrap();

        let mut to_return = Passage(vec![]);

        let query_iter = match sql.query_map((), |row| {
            Ok(Cite {
                book: Book::from_string(&row.get_unwrap::<_, String>("libro")).unwrap(),
                chapter: row.get_unwrap("capitulo"),
                verse: row.get_unwrap("verso"),
                text: row.get_unwrap("texto"),
                translation: traduction.clone(),
            })
        }) {
            Ok(t) => t,
            Err(error) => return Err(error),
        };

        for res in query_iter {
            match res {
                Ok(cite) => to_return.0.push(cite),
                Err(_) => {}
            }
        }
        Ok(to_return)
    }

    /// Se consulta `BIBLIA` y no `TEXTO_VERSO` porque es el indice canonico: la
    /// estructura de libros y capitulos es la misma en cualquier traduccion, y
    /// asi los paneles de navegacion no cambian al cambiar de version.
    fn chapters(&self, book: Book) -> Result<Vec<u8>, Error> {
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT capitulo FROM BIBLIA WHERE libro = ?1 ORDER BY capitulo;",
        )?;

        let rows = statement.query_map((book.as_string(),), |row| row.get::<_, u8>(0))?;

        Ok(rows.flatten().collect())
    }

    fn verses(&self, book: Book, chapter: u8, translation: &str) -> Result<Vec<Cite>, Error> {
        let translation = self.translation(translation)?;

        let mut statement = self.connection.prepare(
            "SELECT verso, texto FROM TEXTO_VERSO \
             WHERE libro = ?1 AND capitulo = ?2 AND translation_id = ?3 \
             ORDER BY verso;",
        )?;

        let rows = statement.query_map(
            (book.as_string(), chapter, translation.id),
            |row| {
                Ok(Cite {
                    book,
                    chapter,
                    verse: row.get_unwrap("verso"),
                    text: row.get_unwrap("texto"),
                    translation: translation.clone(),
                })
            },
        )?;

        Ok(rows.flatten().collect())
    }
}
