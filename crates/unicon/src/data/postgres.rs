// All keywords with `reserved` in the Postgres column from https://www.postgresql.org/docs/current/sql-keywords-appendix.html.
// - macro-time error when a Rust struct field uses a reserved keyword.
pub static POSTGRES_RESERVED_KEYWORDS: &[&str] = &[
    "ABS",
    "ACOS",
    "ALL",
    "ALLOCATE",
    "ANALYSE",
    "ANALYZE",
    "AND",
    "ANY",
    "ARE",
    "ARRAY",
    "ARRAY_AGG",
    "AS",
    "ASC",
    "ASENSITIVE",
    "ASIN",
    "ASYMMETRIC",
    "ATAN",
    "ATOMIC",
    "AUTHORIZATION",
    "AVG",
    "BEGIN_FRAME",
    "BEGIN_PARTITION",
    "BINARY",
    "BIT_LENGTH",
    "BLOB",
    "BOTH",
    "CARDINALITY",
    "CASE",
    "CAST",
    "CEIL",
    "CEILING",
    "CHARACTER_LENGTH",
    "CHAR_LENGTH",
    "CHECK",
    "CLASSIFIER",
    "CLOB",
    "COLLATE",
    "COLLATION",
    "COLLECT",
    "COLUMN",
    "CONCURRENTLY",
    "CONDITION",
    "CONNECT",
    "CONSTRAINT",
    "CONTAINS",
    "CONVERT",
    "CORR",
    "CORRESPONDING",
    "COS",
    "COSH",
    "COUNT",
    "COVAR_POP",
    "COVAR_SAMP",
    "CREATE",
    "CROSS",
    "CUME_DIST",
    "CURRENT_CATALOG",
    "CURRENT_DATE",
    "CURRENT_PATH",
    "CURRENT_ROLE",
    "CURRENT_ROW",
    "CURRENT_SCHEMA",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "CURRENT_USER",
    "DATALINK",
    "DATE",
    "DECFLOAT",
    "DEFAULT",
    "DEFERRABLE",
    "DEFINE",
    "DENSE_RANK",
    "DEREF",
    "DESC",
    "DESCRIBE",
    "DETERMINISTIC",
    "DISCONNECT",
    "DISTINCT",
    "DLNEWCOPY",
    "DLPREVIOUSCOPY",
    "DLURLCOMPLETE",
    "DLURLCOMPLETEONLY",
    "DLURLCOMPLETEWRITE",
    "DLURLPATH",
    "DLURLPATHONLY",
    "DLURLPATHWRITE",
    "DLURLSCHEME",
    "DLURLSERVER",
    "DLVALUE",
    "DO",
    "DYNAMIC",
    "ELEMENT",
    "ELSE",
    "EMPTY",
    "END",
    "END_FRAME",
    "END_PARTITION",
    "EQUALS",
    "EVERY",
    "EXCEPT",
    "EXCEPTION",
    "EXEC",
    "EXP",
    "FALSE",
    "FETCH",
    "FIRST_VALUE",
    "FLOOR",
    "FOR",
    "FOREIGN",
    "FRAME_ROW",
    "FREE",
    "FREEZE",
    "FROM",
    "FULL",
    "FUSION",
    "GET",
    "GRANT",
    "GROUP",
    "HAVING",
    "ILIKE",
    "IN",
    "INDICATOR",
    "INITIAL",
    "INITIALLY",
    "INNER",
    "INTERSECT",
    "INTERSECTION",
    "INTO",
    "IS",
    "ISNULL",
    "JOIN",
    "JSON_ARRAY",
    "JSON_ARRAYAGG",
    "JSON_EXISTS",
    "JSON_OBJECT",
    "JSON_OBJECTAGG",
    "JSON_QUERY",
    "JSON_TABLE",
    "JSON_TABLE_PRIMITIVE",
    "JSON_VALUE",
    "LAG",
    "LAST_VALUE",
    "LATERAL",
    "LEAD",
    "LEADING",
    "LEFT",
    "LIKE",
    "LIKE_REGEX",
    "LIMIT",
    "LISTAGG",
    "LN",
    "LOCALTIME",
    "LOCALTIMESTAMP",
    "LOG",
    "LOG10",
    "LOWER",
    "MATCHES",
    "MATCH_NUMBER",
    "MATCH_RECOGNIZE",
    "MAX",
    "MEASURES",
    "MEMBER",
    "MERGE",
    "MIN",
    "MOD",
    "MODIFIES",
    "MODULE",
    "MULTISET",
    "NATURAL",
    "NCLOB",
    "NOT",
    "NOTNULL",
    "NTH_VALUE",
    "NTILE",
    "NULL",
    "OCCURRENCES_REGEX",
    "OCTET_LENGTH",
    "OFFSET",
    "OMIT",
    "ON",
    "ONE",
    "ONLY",
    "OPEN",
    "OR",
    "ORDER",
    "OUTER",
    "OVERLAPS",
    "PARAMETER",
    "PATTERN",
    "PER",
    "PERCENT",
    "PERCENTILE_CONT",
    "PERCENTILE_DISC",
    "PERCENT_RANK",
    "PERIOD",
    "PERMUTE",
    "PLACING",
    "PORTION",
    "POSITION_REGEX",
    "POWER",
    "PRECEDES",
    "PRIMARY",
    "PTF",
    "RANK",
    "READS",
    "REFERENCES",
    "REGR_AVGX",
    "REGR_AVGY",
    "REGR_COUNT",
    "REGR_INTERCEPT",
    "REGR_R2",
    "REGR_SLOPE",
    "REGR_SXX",
    "REGR_SXY",
    "REGR_SYY",
    "RESULT",
    "RETURN",
    "RETURNING",
    "RIGHT",
    "ROW_NUMBER",
    "RUNNING",
    "SCOPE",
    "SEEK",
    "SELECT",
    "SENSITIVE",
    "SESSION_USER",
    "SIMILAR",
    "SIN",
    "SINH",
    "SOME",
    "SPECIFIC",
    "SPECIFICTYPE",
    "SQLCODE",
    "SQLERROR",
    "SQLEXCEPTION",
    "SQLSTATE",
    "SQLWARNING",
    "SQRT",
    "STATIC",
    "STDDEV_POP",
    "STDDEV_SAMP",
    "SUBMULTISET",
    "SUBSET",
    "SUBSTRING_REGEX",
    "SUCCEEDS",
    "SUM",
    "SYMMETRIC",
    "SYSTEM_TIME",
    "SYSTEM_USER",
    "TABLE",
    "TABLESAMPLE",
    "TAN",
    "TANH",
    "THEN",
    "TIMEZONE_HOUR",
    "TIMEZONE_MINUTE",
    "TO",
    "TRAILING",
    "TRANSLATE",
    "TRANSLATE_REGEX",
    "TRANSLATION",
    "TRIM_ARRAY",
    "TRUE",
    "UNION",
    "UNIQUE",
    "UNMATCHED",
    "UNNEST",
    "UPPER",
    "USER",
    "USING",
    "VALUE_OF",
    "VARBINARY",
    "VARIADIC",
    "VAR_POP",
    "VAR_SAMP",
    "VERBOSE",
    "VERSIONING",
    "WHEN",
    "WHENEVER",
    "WHERE",
    "WIDTH_BUCKET",
    "WINDOW",
    "WITH",
    "XMLAGG",
    "XMLBINARY",
    "XMLCAST",
    "XMLCOMMENT",
    "XMLDOCUMENT",
    "XMLITERATE",
    "XMLQUERY",
    "XMLTEXT",
    "XMLVALIDATE",
];