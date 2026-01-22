; SPARQL Syntax Highlighting for Zed

; Comments
(comment) @comment

; Keywords - Query Types
[
  "SELECT"
  "CONSTRUCT"
  "DESCRIBE"
  "ASK"
] @keyword

; Keywords - Clauses
[
  "BASE"
  "PREFIX"
  "FROM"
  "NAMED"
  "WHERE"
  "ORDER"
  "BY"
  "ASC"
  "DESC"
  "LIMIT"
  "OFFSET"
  "VALUES"
  "GROUP"
  "HAVING"
] @keyword

; Keywords - Modifiers
[
  "DISTINCT"
  "REDUCED"
  "AS"
] @keyword

; Keywords - Graph Patterns
[
  "OPTIONAL"
  "UNION"
  "MINUS"
  "GRAPH"
  "SERVICE"
  "SILENT"
  "FILTER"
  "BIND"
  "NOT"
  "IN"
  "EXISTS"
] @keyword

; Keywords - Update Operations
[
  "INSERT"
  "DELETE"
  "DATA"
  "LOAD"
  "CLEAR"
  "DROP"
  "CREATE"
  "ADD"
  "MOVE"
  "COPY"
  "INTO"
  "TO"
  "WITH"
  "USING"
  "DEFAULT"
  "ALL"
] @keyword

; Variables
(var) @variable

; IRIs
(iri_reference) @string.special
(prefixed_name) @type

; Namespace prefix declaration
(prefix_declaration
  (namespace) @type.builtin)

; Literals
(rdf_literal) @string
(boolean_literal) @constant.builtin
(integer) @number
(decimal) @number
(double) @number

; String contents
(string) @string

; Language tags
(lang_tag) @attribute

; Aggregates and Built-in Functions
[
  "COUNT"
  "SUM"
  "MIN"
  "MAX"
  "AVG"
  "SAMPLE"
  "GROUP_CONCAT"
  "SEPARATOR"
] @function.builtin

; Built-in functions
[
  "STR"
  "LANG"
  "LANGMATCHES"
  "DATATYPE"
  "BOUND"
  "IRI"
  "URI"
  "BNODE"
  "RAND"
  "ABS"
  "CEIL"
  "FLOOR"
  "ROUND"
  "CONCAT"
  "STRLEN"
  "UCASE"
  "LCASE"
  "ENCODE_FOR_URI"
  "CONTAINS"
  "STRSTARTS"
  "STRENDS"
  "STRBEFORE"
  "STRAFTER"
  "YEAR"
  "MONTH"
  "DAY"
  "HOURS"
  "MINUTES"
  "SECONDS"
  "TIMEZONE"
  "TZ"
  "NOW"
  "UUID"
  "STRUUID"
  "MD5"
  "SHA1"
  "SHA256"
  "SHA384"
  "SHA512"
  "COALESCE"
  "IF"
  "STRLANG"
  "STRDT"
  "sameTerm"
  "isIRI"
  "isURI"
  "isBLANK"
  "isLITERAL"
  "isNUMERIC"
  "REGEX"
  "SUBSTR"
  "REPLACE"
] @function.builtin

; Operators
[
  "||"
  "&&"
  "="
  "!="
  "<"
  ">"
  "<="
  ">="
  "+"
  "-"
  "*"
  "/"
  "!"
  "^"
  "^^"
] @operator

; Special operator 'a' (rdf:type shorthand)
"a" @keyword.operator

; Punctuation
[
  "."
  ","
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "<"
  ">"
] @punctuation.special
