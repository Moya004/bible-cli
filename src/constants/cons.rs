use regex::Regex;
use std::{path::PathBuf, sync::OnceLock};

pub static BOOKS: &[(&str, &str, &str)] = &[
    // Pentateuco
    ("Génesis", "Gn", r"(?i)\b(Génesis|Genesis|Gen|Gn)\b"),
    ("Éxodo", "Ex", r"(?i)\b(Éxodo|Exodo|Exod|Ex|Exo)\b"),
    ("Levítico", "Lv", r"(?i)\b(Levítico|Levitico|Lev|Lv)\b"),
    ("Números", "Nm", r"(?i)\b(Números|Numeros|Num|Nm)\b"),
    ("Deuteronomio", "Dt", r"(?i)\b(Deuteronomio|Deut|Dt)\b"),
    // Historicos
    ("Josué", "Js", r"(?i)\b(Josué|Josue|Jos|Js)\b"),
    ("Jueces", "Jc", r"(?i)\b(Jueces|Jue|Juec|Jz)\b"),
    ("Rut", "Rt", r"(?i)\b(Rut|Rt)\b"),
    ("1 Samuel", "1Sm", r"(?i)\b(1\s*-?\s*Samuel|1\s*-?\s*Sam|1\s*-?\s*Sm)\b"),
    ("2 Samuel", "2Sm", r"(?i)\b(2\s*-?\s*Samuel|2\s*-?\s*Sam|2\s*-?\s*Sm)\b"),
    ("1 Reyes", "1Re", r"(?i)\b(1\s*-?\s*Reyes|1\s*-?\s*Rey|1\s*-?\s*R)\b"),
    ("2 Reyes", "2Re", r"(?i)\b(2\s*-?\s*Reyes|2\s*-?\s*Rey|2\s*-?\s*R)\b"),
    ("1 Crónicas", "1Cr", r"(?i)\b(1\s*-?\s*Crónicas|1\s*-?\s*Cronicas|1\s*-?\s*Cro|1\s*-?\s*Chr)\b"),
    ("2 Crónicas", "2Cr", r"(?i)\b(2\s*-?\s*Crónicas|2\s*-?\s*Cronicas|2\s*-?\s*Cro|2\s*-?\s*Chr)\b"),
    ("Esdras", "Esd", r"(?i)\b(Esdras|Esd|Ezr)\b"),
    ("Nehemías", "Neh", r"(?i)\b(Nehemías|Nehemias|Neh|Ne)\b"),
    ("Ester", "Est", r"(?i)\b(Ester|Est)\b"),
    // Poeticos
    ("Job", "Job", r"(?i)\b(Job|Jb)\b"),
    ("Salmos", "Sal", r"(?i)\b(Salmos|Sal|Ps)\b"),
    ("Proverbios", "Pr", r"(?i)\b(Proverbios|Prov|Pr)\b"),
    ("Eclesiastés", "Ec", r"(?i)\b(Eclesiastés|Eclesiastes|Ecl|Qo|Ec)\b"),
    ("Cantar de los cantares", "Ct", r"(?i)\b(Cantares|Cant|Ct|Can)\b"),
    // Profetas Mayores
    ("Isaías", "Is", r"(?i)\b(Isaías|Isaias|Isa|Is)\b"),
    ("Jeremías", "Jer", r"(?i)\b(Jeremías|Jeremias|Jer|Jr)\b"),
    ("Lamentaciones", "Lm", r"(?i)\b(Lamentaciones|Lam|Lm)\b"),
    ("Ezequiel", "Ez", r"(?i)\b(Ezequiel|Eze|Ez)\b"),
    ("Daniel", "Dn", r"(?i)\b(Daniel|Dan|Dn)\b"),
    // Profetas Menores
    ("Oseas", "Os", r"(?i)\b(Oseas|Ose|Os)\b"),
    ("Joel", "Jl", r"(?i)\b(Joel|Jl)\b"),
    ("Amós", "Am", r"(?i)\b(Amós|Amos|Am)\b"),
    ("Abdías", "Abd", r"(?i)\b(Abdías|Abdias|Abd|Ab)\b"),
    ("Jonás", "Jon", r"(?i)\b(Jonás|Jonas|Jon)\b"),
    ("Miqueas", "Miq", r"(?i)\b(Miqueas|Miq|Mi)\b"),
    ("Nahúm", "Nah", r"(?i)\b(Nahúm|Nahum|Nah|Na)\b"),
    ("Habacuc", "Hab", r"(?i)\b(Habacuc|Hab|Ha)\b"),
    ("Sofonías", "Sof", r"(?i)\b(Sofonías|Sofonias|Sof|Sf)\b"),
    ("Hageo", "Hag", r"(?i)\b(Hageo|Hag|Hg)\b"),
    ("Zacarías", "Zac", r"(?i)\b(Zacarías|Zacarias|Zac|Zc)\b"),
    ("Malaquías", "Mal", r"(?i)\b(Malaquías|Malaquias|Mal|Ml)\b"),
    // Evangelios y Hechos
    ("Mateo", "Mt", r"(?i)\b(Mateo|Mat|Mt)\b"),
    ("Marcos", "Mc", r"(?i)\b(Marcos|Mar|Mc|Mr)\b"),
    ("Lucas", "Lc", r"(?i)\b(Lucas|Luc|Lc)\b"),
    ("Juan", "Jn", r"(?i)\b(Juan|Jn)\b"),
    ("Hechos", "Hch", r"(?i)\b(Hechos|Hch|Ac)\b"),
    // Cartas Paulinas
    ("Romanos", "Rm", r"(?i)\b(Romanos|Rom|Ro)\b"),
    ("1 Corintios", "1Co", r"(?i)\b(1\s*-?\s*Corintios|1\s*-?\s*Cor|1\s*-?\s*Co)\b"),
    ("2 Corintios", "2Co", r"(?i)\b(2\s*-?\s*Corintios|2\s*-?\s*Cor|2\s*-?\s*Co)\b"),
    ("Gálatas", "Ga", r"(?i)\b(Gálatas|Galatas|Gal|Gl)\b"),
    ("Efesios", "Ef", r"(?i)\b(Efesios|Efe|Ef)\b"),
    ("Filipenses", "Fil", r"(?i)\b(Filipenses|Fil|Flp)\b"),
    ("Colosenses", "Col", r"(?i)\b(Colosenses|Col|Cl)\b"),
    ("1 Tesalonicenses", "1Ts", r"(?i)\b(1\s*-?\s*Tesalonicenses|1\s*-?\s*Tes|1\s*-?\s*Ts)\b"),
    ("2 Tesalonicenses", "2Ts", r"(?i)\b(2\s*-?\s*Tesalonicenses|2\s*-?\s*Tes|2\s*-?\s*Ts)\b"),
    ("1 Timoteo", "1Tm", r"(?i)\b(1\s*-?\s*Timoteo|1\s*-?\s*Tim|1\s*-?\s*Tm)\b"),
    ("2 Timoteo", "2Tm", r"(?i)\b(2\s*-?\s*Timoteo|2\s*-?\s*Tim|2\s*-?\s*Tm)\b"),
    ("Tito", "Tit", r"(?i)\b(Tito|Tit|Tt)\b"),
    ("Filemón", "Flm", r"(?i)\b(Filemón|Filemon|Filem|Flm)\b"),
    // Otras Cartas y Apocalipsis
    ("Hebreos", "Heb", r"(?i)\b(Hebreos|Heb|Hb)\b"),
    ("Santiago", "Stg", r"(?i)\b(Santiago|Sant|Stgo|Jas)\b"),
    ("1 Pedro", "1P", r"(?i)\b(1\s*-?\s*Pedro|1\s*-?\s*Pe|1\s*-?\s*P)\b"),
    ("2 Pedro", "2P", r"(?i)\b(2\s*-?\s*Pedro|2\s*-?\s*Pe|2\s*-?\s*P)\b"),
    ("1 Juan", "1Jn", r"(?i)\b(1\s*-?\s*Juan|1\s*-?\s*Jn)\b"),
    ("2 Juan", "2Jn", r"(?i)\b(2\s*-?\s*Juan|2\s*-?\s*Jn)\b"),
    ("3 Juan", "3Jn", r"(?i)\b(3\s*-?\s*Juan|3\s*-?\s*Jn)\b"),
    ("Judas", "Jud", r"(?i)\b(Judas|Jud|Jd)\b"),
    ("Apocalipsis", "Ap", r"(?i)\b(Apocalipsis|Apoc|Ap|Rev)\b"),
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
            .map(|(name, _, pattern)| (*name, Regex::new(pattern).unwrap()))
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
