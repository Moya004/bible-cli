use std::ops::RangeInclusive;

use crate::constants::cons::{BOOKS, compiled_books};

#[derive(Debug, Clone)]
pub enum IndexVariation {
    Single(u8),
    List(RangeInclusive<u8>),
}

impl IndexVariation {
    pub fn as_string(&self) -> String {
        match self {
            IndexVariation::List(r) => format!("{}-{}", r.start(), r.end()),
            IndexVariation::Single(s) => format!("{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub book: Book,
    pub chapter: IndexVariation,
    pub verse: IndexVariation,
    pub translation: String,
}

impl Query {
    pub fn as_string(&self) -> String {
        format!(
            "Libro: {}\r\nCapitulo(s): {}\r\nVersiculo(s): {}\r\nTraduccion: {}\r\n",
            self.book.as_string(),
            self.chapter.as_string(),
            self.verse.as_string(),
            self.translation,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Book {
    // Pentateuco
    Genesis,
    Exodo,
    Levitico,
    Numeros,
    Deuteronomio,
    // Históricos
    Josue,
    Jueces,
    Rut,
    Samuel1,
    Samuel2,
    Reyes1,
    Reyes2,
    Cronicas1,
    Cronicas2,
    Esdras,
    Nehemias,
    Ester,
    // Poéticos
    Job,
    Salmos,
    Proverbios,
    Eclesiastes,
    Cantares,
    // Profetas Mayores
    Isaias,
    Jeremias,
    Lamentaciones,
    Ezequiel,
    Daniel,
    // Profetas Menores
    Oseas,
    Joel,
    Amos,
    Abdias,
    Jonas,
    Miqueas,
    Nahum,
    Habacuc,
    Sofonias,
    Hageo,
    Zacarias,
    Malaquias,
    // Evangelios y Hechos
    Mateo,
    Marcos,
    Lucas,
    Juan,
    Hechos,
    // Cartas Paulinas
    Romanos,
    Corintios1,
    Corintios2,
    Galatas,
    Efesios,
    Filipenses,
    Colosenses,
    Tesalonicenses1,
    Tesalonicenses2,
    Timoteo1,
    Timoteo2,
    Tito,
    Filemon,
    // Otras Cartas y Apocalipsis
    Hebreos,
    Santiago,
    Pedro1,
    Pedro2,
    Juan1,
    Juan2,
    Juan3,
    Judas,
    Apocalipsis,
}

impl Book {
    pub fn as_string(&self) -> &'static str {
        match self {
            // Pentateuco
            Book::Genesis => BOOKS[0].0,
            Book::Exodo => BOOKS[1].0,
            Book::Levitico => BOOKS[2].0,
            Book::Numeros => BOOKS[3].0,
            Book::Deuteronomio => BOOKS[4].0,
            // Históricos
            Book::Josue => BOOKS[5].0,
            Book::Jueces => BOOKS[6].0,
            Book::Rut => BOOKS[7].0,
            Book::Samuel1 => BOOKS[8].0,
            Book::Samuel2 => BOOKS[9].0,
            Book::Reyes1 => BOOKS[10].0,
            Book::Reyes2 => BOOKS[11].0,
            Book::Cronicas1 => BOOKS[12].0,
            Book::Cronicas2 => BOOKS[13].0,
            Book::Esdras => BOOKS[14].0,
            Book::Nehemias => BOOKS[15].0,
            Book::Ester => BOOKS[16].0,
            // Poéticos
            Book::Job => BOOKS[17].0,
            Book::Salmos => BOOKS[18].0,
            Book::Proverbios => BOOKS[19].0,
            Book::Eclesiastes => BOOKS[20].0,
            Book::Cantares => BOOKS[21].0,
            // Profetas Mayores
            Book::Isaias => BOOKS[22].0,
            Book::Jeremias => BOOKS[23].0,
            Book::Lamentaciones => BOOKS[24].0,
            Book::Ezequiel => BOOKS[25].0,
            Book::Daniel => BOOKS[26].0,
            // Profetas Menores
            Book::Oseas => BOOKS[27].0,
            Book::Joel => BOOKS[28].0,
            Book::Amos => BOOKS[29].0,
            Book::Abdias => BOOKS[30].0,
            Book::Jonas => BOOKS[31].0,
            Book::Miqueas => BOOKS[32].0,
            Book::Nahum => BOOKS[33].0,
            Book::Habacuc => BOOKS[34].0,
            Book::Sofonias => BOOKS[35].0,
            Book::Hageo => BOOKS[36].0,
            Book::Zacarias => BOOKS[37].0,
            Book::Malaquias => BOOKS[38].0,
            // Evangelios y Hechos
            Book::Mateo => BOOKS[39].0,
            Book::Marcos => BOOKS[40].0,
            Book::Lucas => BOOKS[41].0,
            Book::Juan => BOOKS[42].0,
            Book::Hechos => BOOKS[43].0,
            // Cartas Paulinas
            Book::Romanos => BOOKS[44].0,
            Book::Corintios1 => BOOKS[45].0,
            Book::Corintios2 => BOOKS[46].0,
            Book::Galatas => BOOKS[47].0,
            Book::Efesios => BOOKS[48].0,
            Book::Filipenses => BOOKS[49].0,
            Book::Colosenses => BOOKS[50].0,
            Book::Tesalonicenses1 => BOOKS[51].0,
            Book::Tesalonicenses2 => BOOKS[52].0,
            Book::Timoteo1 => BOOKS[53].0,
            Book::Timoteo2 => BOOKS[54].0,
            Book::Tito => BOOKS[55].0,
            Book::Filemon => BOOKS[56].0,
            // Otras Cartas y Apocalipsis
            Book::Hebreos => BOOKS[57].0,
            Book::Santiago => BOOKS[58].0,
            Book::Pedro1 => BOOKS[59].0,
            Book::Pedro2 => BOOKS[60].0,
            Book::Juan1 => BOOKS[61].0,
            Book::Juan2 => BOOKS[62].0,
            Book::Juan3 => BOOKS[63].0,
            Book::Judas => BOOKS[64].0,
            Book::Apocalipsis => BOOKS[65].0,
        }
    }

    pub fn from_string(input: &str) -> Option<Self> {
        let compiled = compiled_books();
        compiled
            .iter()
            .find(|(_, re)| re.is_match(input))
            .and_then(|(name, _)| match *name {
                "Génesis" => Some(Book::Genesis),
                "Éxodo" => Some(Book::Exodo),
                "Levítico" => Some(Book::Levitico),
                "Números" => Some(Book::Numeros),
                "Deuteronomio" => Some(Book::Deuteronomio),
                "Josué" => Some(Book::Josue),
                "Jueces" => Some(Book::Jueces),
                "Rut" => Some(Book::Rut),
                "1 Samuel" => Some(Book::Samuel1),
                "2 Samuel" => Some(Book::Samuel2),
                "1 Reyes" => Some(Book::Reyes1),
                "2 Reyes" => Some(Book::Reyes2),
                "1 Crónicas" => Some(Book::Cronicas1),
                "2 Crónicas" => Some(Book::Cronicas2),
                "Esdras" => Some(Book::Esdras),
                "Nehemías" => Some(Book::Nehemias),
                "Ester" => Some(Book::Ester),
                "Job" => Some(Book::Job),
                "Salmos" => Some(Book::Salmos),
                "Proverbios" => Some(Book::Proverbios),
                "Eclesiastés" => Some(Book::Eclesiastes),
                "Cantares" => Some(Book::Cantares),
                "Isaías" => Some(Book::Isaias),
                "Jeremías" => Some(Book::Jeremias),
                "Lamentaciones" => Some(Book::Lamentaciones),
                "Ezequiel" => Some(Book::Ezequiel),
                "Daniel" => Some(Book::Daniel),
                "Oseas" => Some(Book::Oseas),
                "Joel" => Some(Book::Joel),
                "Amós" => Some(Book::Amos),
                "Abdías" => Some(Book::Abdias),
                "Jonás" => Some(Book::Jonas),
                "Miqueas" => Some(Book::Miqueas),
                "Nahúm" => Some(Book::Nahum),
                "Habacuc" => Some(Book::Habacuc),
                "Sofonías" => Some(Book::Sofonias),
                "Hageo" => Some(Book::Hageo),
                "Zacarías" => Some(Book::Zacarias),
                "Malaquías" => Some(Book::Malaquias),
                "Mateo" => Some(Book::Mateo),
                "Marcos" => Some(Book::Marcos),
                "Lucas" => Some(Book::Lucas),
                "Juan" => Some(Book::Juan),
                "Hechos" => Some(Book::Hechos),
                "Romanos" => Some(Book::Romanos),
                "1 Corintios" => Some(Book::Corintios1),
                "2 Corintios" => Some(Book::Corintios2),
                "Gálatas" => Some(Book::Galatas),
                "Efesios" => Some(Book::Efesios),
                "Filipenses" => Some(Book::Filipenses),
                "Colosenses" => Some(Book::Colosenses),
                "1 Tesalonicenses" => Some(Book::Tesalonicenses1),
                "2 Tesalonicenses" => Some(Book::Tesalonicenses2),
                "1 Timoteo" => Some(Book::Timoteo1),
                "2 Timoteo" => Some(Book::Timoteo2),
                "Tito" => Some(Book::Tito),
                "Filemón" => Some(Book::Filemon),
                "Hebreos" => Some(Book::Hebreos),
                "Santiago" => Some(Book::Santiago),
                "1 Pedro" => Some(Book::Pedro1),
                "2 Pedro" => Some(Book::Pedro2),
                "1 Juan" => Some(Book::Juan1),
                "2 Juan" => Some(Book::Juan2),
                "3 Juan" => Some(Book::Juan3),
                "Judas" => Some(Book::Judas),
                "Apocalipsis" => Some(Book::Apocalipsis),
                _ => None,
            })
    }
}
