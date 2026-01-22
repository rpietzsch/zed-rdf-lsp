; SPARQL Syntax Highlighting for Zed

; Comments
(comment) @comment

; Variables
(var) @variable

; IRIs and Prefixed Names
(iri_reference) @string.special
(prefixed_name) @type
(namespace) @type.builtin
(pn_prefix) @type.builtin
(pn_local) @property

; Literals
(string) @string
(rdf_literal) @string
(lang_tag) @attribute
(boolean_literal) @constant.builtin
(integer) @number
(decimal) @number
(double) @number

; Escape sequences
(echar) @string.escape

; Blank nodes
(blank_node_label) @variable.special
(anon) @variable.special

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
  "UNDEF"
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
  "INSERT DATA"
  "DELETE DATA"
  "DELETE WHERE"
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

; Aggregate Functions
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

; Built-in Functions
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
  "REGEX"
  "SUBSTR"
  "REPLACE"
  "sameTerm"
  "isIRI"
  "isURI"
  "isBLANK"
  "isLITERAL"
  "isNUMERIC"
] @function.builtin

; Operators
[
  "||"
  "&&"
  "="
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
  "|"
] @operator

; Boolean literals
[
  "true"
  "false"
] @constant.builtin

; Special predicate 'a' (rdf:type shorthand)
"a" @keyword

; Punctuation - Delimiters
[
  "."
  ","
  ";"
  ":"
] @punctuation.delimiter

; Punctuation - Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket
