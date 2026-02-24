pub trait Dialect {
    fn name(&self) -> &'static str;
    fn quote_char(&self) -> &'static str;
    fn supports_distinct_on(&self) -> bool;
    fn supports_window_function(&self) -> bool;
    fn supports_returning(&self) -> bool;
    fn supports_standalone_offset(&self) -> bool;
}

pub struct MySQL;
impl Dialect for MySQL {
    #[inline]
    fn name(&self) -> &'static str {
        "MySQL"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "`"
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        false
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        true // 8+
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        false
    }
    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        false
    }
}

pub struct PostgreSQL;
impl Dialect for PostgreSQL {
    #[inline]
    fn name(&self) -> &'static str {
        "PostgreSQL"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        true
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        true
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        true
    }
    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        true
    }
}
pub struct Sqlite;
impl Dialect for Sqlite {
    #[inline]
    fn name(&self) -> &'static str {
        "Sqlite"
    }
    #[inline]
    fn quote_char(&self) -> &'static str {
        "\""
    }
    #[inline]
    fn supports_distinct_on(&self) -> bool {
        false
    }
    #[inline]
    fn supports_window_function(&self) -> bool {
        false
    }
    #[inline]
    fn supports_returning(&self) -> bool {
        true
    }

    #[inline]
    fn supports_standalone_offset(&self) -> bool {
        true
    }
}

pub static MY: MySQL = MySQL {};
pub static PG: PostgreSQL = PostgreSQL {};
pub static LITE: Sqlite = Sqlite {};
