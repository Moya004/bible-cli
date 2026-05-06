use regex::Regex;
use std::{path::PathBuf, sync::OnceLock};

pub static BOOKS: &[(&str, &str)] = &[
    // Pentateuco
    ("Génesis", r"(?i)\b(Genesis|Gen|Gn)\b"),
    ("Éxodo", r"(?i)\b(Exodo|Exod|Ex|Exo)\b"),
    ("Levítico", r"(?i)\b(Levitico|Lev|Lv)\b"),
    ("Números", r"(?i)\b(Numeros|Num|Nm)\b"),
    ("Deuteronomio", r"(?i)\b(Deuteronomio|Deut|Dt)\b"),
    // Históricos
    ("Josué", r"(?i)\b(Josue|Jos|Js)\b"),
    ("Jueces", r"(?i)\b(Jueces|Jue|Juec|Jz)\b"),
    ("Rut", r"(?i)\b(Rut|Rt)\b"),
    ("1 Samuel", r"(?i)\b(1\s*-?\s*Samuel|1\s?Sam|1\s?Sm)\b"),
    ("2 Samuel", r"(?i)\b(2\s*-?\s*Samuel|2\s?Sam|2\s?Sm)\b"),
    ("1 Reyes", r"(?i)\b(1\s*-?\s*Reyes|1\s?Rey|1\s?R)\b"),
    ("2 Reyes", r"(?i)\b(2\s*-?\s*Reyes|2\s?Rey|2\s?R)\b"),
    ("1 Crónicas", r"(?i)\b(1\s*-?\s*Cronicas|1\s?Cro|1\s?Chr)\b"),
    ("2 Crónicas", r"(?i)\b(2\s*-?\s*Cronicas|2\s?Cro|2\s?Chr)\b"),
    ("Esdras", r"(?i)\b(Esdras|Esd|Ezr)\b"),
    ("Nehemías", r"(?i)\b(Nehemias|Neh|Ne)\b"),
    ("Ester", r"(?i)\b(Ester|Est)\b"),
    // Poéticos
    ("Job", r"(?i)\b(Job|Jb)\b"),
    ("Salmos", r"(?i)\b(Salmos|Sal|Ps)\b"),
    ("Proverbios", r"(?i)\b(Proverbios|Prov|Pr)\b"),
    ("Eclesiastés", r"(?i)\b(Eclesiastes|Ecl|Qo|Ec)\b"),
    ("Cantar de los cantares", r"(?i)\b(Cantares|Cant|Ct|Can)\b"),
    // Profetas Mayores
    ("Isaías", r"(?i)\b(Isaias|Isa|Is)\b"),
    ("Jeremías", r"(?i)\b(Jeremias|Jer|Jr)\b"),
    ("Lamentaciones", r"(?i)\b(Lamentaciones|Lam|Lm)\b"),
    ("Ezequiel", r"(?i)\b(Ezequiel|Eze|Ez)\b"),
    ("Daniel", r"(?i)\b(Daniel|Dan|Dn)\b"),
    // Profetas Menores
    ("Oseas", r"(?i)\b(Oseas|Ose|Os)\b"),
    ("Joel", r"(?i)\b(Joel|Jl)\b"),
    ("Amós", r"(?i)\b(Amos|Am)\b"),
    ("Abdías", r"(?i)\b(Abdias|Abd|Ab)\b"),
    ("Jonás", r"(?i)\b(Jonas|Jon)\b"),
    ("Miqueas", r"(?i)\b(Miqueas|Miq|Mi)\b"),
    ("Nahúm", r"(?i)\b(Nahum|Nah|Na)\b"),
    ("Habacuc", r"(?i)\b(Habacuc|Hab|Ha)\b"),
    ("Sofonías", r"(?i)\b(Sofonias|Sof|Sf)\b"),
    ("Hageo", r"(?i)\b(Hageo|Hag|Hg)\b"),
    ("Zacarías", r"(?i)\b(Zacarias|Zac|Zc)\b"),
    ("Malaquías", r"(?i)\b(Malaquias|Mal|Ml)\b"),
    // Nuevo Testamento - Evangelios y Hechos
    ("Mateo", r"(?i)\b(Mateo|Mat|Mt)\b"),
    ("Marcos", r"(?i)\b(Marcos|Mar|Mc|Mr)\b"),
    ("Lucas", r"(?i)\b(Lucas|Luc|Lc)\b"),
    ("Juan", r"(?i)\b(Juan|Jn)\b"),
    ("Hechos", r"(?i)\b(Hechos|Hch|Ac)\b"),
    // Cartas Paulinas
    ("Romanos", r"(?i)\b(Romanos|Rom|Ro)\b"),
    (
        "1 Corintios",
        r"(?i)\b(1\s*-?\s*Corintios|1\s?Cor|1\s?Co)\b",
    ),
    (
        "2 Corintios",
        r"(?i)\b(2\s*-?\s*Corintios|2\s?Cor|2\s?Co)\b",
    ),
    ("Gálatas", r"(?i)\b(Galatas|Gal|Gl)\b"),
    ("Efesios", r"(?i)\b(Efesios|Efe|Ef)\b"),
    ("Filipenses", r"(?i)\b(Filipenses|Fil|Flp)\b"),
    ("Colosenses", r"(?i)\b(Colosenses|Col|Cl)\b"),
    (
        "1 Tesalonicenses",
        r"(?i)\b(1\s*-?\s*Tesalonicenses|1\s?Tes|1\s?Ts)\b",
    ),
    (
        "2 Tesalonicenses",
        r"(?i)\b(2\s*-?\s*Tesalonicenses|2\s?Tes|2\s?Ts)\b",
    ),
    ("1 Timoteo", r"(?i)\b(1\s*-?\s*Timoteo|1\s?Tim|1\s?Tm)\b"),
    ("2 Timoteo", r"(?i)\b(2\s*-?\s*Timoteo|2\s?Tim|2\s?Tm)\b"),
    ("Tito", r"(?i)\b(Tito|Tit|Tt)\b"),
    ("Filemón", r"(?i)\b(Filemon|Filem|Flm)\b"),
    // Otras Cartas y Apocalipsis
    ("Hebreos", r"(?i)\b(Hebreos|Heb|Hb)\b"),
    ("Santiago", r"(?i)\b(Santiago|Sant|Stgo|Jas)\b"),
    ("1 Pedro", r"(?i)\b(1\s*-?\s*Pedro|1\s?Pe|1\s?P)\b"),
    ("2 Pedro", r"(?i)\b(2\s*-?\s*Pedro|2\s?Pe|2\s?P)\b"),
    ("1 Juan", r"(?i)\b(1\s*-?\s*Juan|1\s?Jn)\b"),
    ("2 Juan", r"(?i)\b(2\s*-?\s*Juan|2\s?Jn)\b"),
    ("3 Juan", r"(?i)\b(3\s*-?\s*Juan|3\s?Jn)\b"),
    ("Judas", r"(?i)\b(Judas|Jud|Jd)\b"),
    ("Apocalipsis", r"(?i)\b(Apocalipsis|Apoc|Ap|Rev)\b"),
];

pub const TABLES_SCHEMA: &str = include_str!("../db/schemas/tables.sql");

pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

static COMPILED_BOOKS_RE: OnceLock<Vec<(&'static str, Regex)>> = OnceLock::new();

static SINGLE: OnceLock<Regex> = OnceLock::new();
static LIST: OnceLock<Regex> = OnceLock::new();
static ATOM: OnceLock<Regex> = OnceLock::new();
static GROUP: OnceLock<Regex> = OnceLock::new();
static COMPLETE: OnceLock<Regex> = OnceLock::new();
static COLLECTION: OnceLock<Regex> = OnceLock::new();

static TRANSLATION_FLAG: OnceLock<Regex> = OnceLock::new();

/** Return a Vec of the compiled regular expresions of the BOOKS constant*/
pub fn compiled_books() -> &'static Vec<(&'static str, Regex)> {
    COMPILED_BOOKS_RE.get_or_init(|| {
        BOOKS
            .iter()
            .map(|(name, pattern)| (*name, Regex::new(pattern).unwrap()))
            .collect()
    })
}

pub fn single_regex() -> &'static Regex {
    SINGLE.get_or_init(|| Regex::new(r"\d+").unwrap())
}

pub fn list_regex() -> &'static Regex {
    LIST.get_or_init(|| {
        Regex::new(&format!(
            r"\s*{}\s*\-\s*{}\s*",
            single_regex(),
            single_regex()
        ))
        .unwrap()
    })
}
pub fn atom_regex() -> &'static Regex {
    ATOM.get_or_init(|| {
        Regex::new(&format!(
            r"(?:\s*{}\s*|\s*{}\s*)",
            list_regex(),
            single_regex()
        ))
        .unwrap()
    })
}
pub fn group_regex() -> &'static Regex {
    GROUP.get_or_init(|| {
        Regex::new(&format!(
            r"\s*{}(?:\s*,\s*{}\s*)*",
            atom_regex(),
            atom_regex()
        ))
        .unwrap()
    })
}
pub fn complete_regex() -> &'static Regex {
    COMPLETE.get_or_init(|| {
        Regex::new(&format!(
            r"(?:\s*{}\s*:\s*{}\s*)",
            group_regex(),
            group_regex(),
        ))
        .unwrap()
    })
}
pub fn collection_regex() -> &'static Regex {
    COLLECTION.get_or_init(|| {
        Regex::new(&format!(
            r"\s*{}\s*(?:\.\s*{}\s*)*",
            complete_regex(),
            complete_regex()
        ))
        .unwrap()
    })
}

pub fn translation_flag_regex() -> &'static Regex {
    TRANSLATION_FLAG.get_or_init(|| Regex::new(r"--\w+").unwrap())
}
