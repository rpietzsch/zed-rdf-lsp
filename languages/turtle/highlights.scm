; Turtle/TriG Syntax Highlighting for Zed

; Comments
(comment) @comment

; Directives
(prefix_id) @keyword
(base) @keyword
(sparql_base) @keyword
(sparql_prefix) @keyword

; Keywords
[
  "@prefix"
  "@base"
  "PREFIX"
  "BASE"
  "GRAPH"
] @keyword

; Special predicate 'a' (rdf:type shorthand)
"a" @keyword.operator

; IRIs
(iri_reference) @string.special

; Prefixed names (like foaf:name)
(prefixed_name
  (namespace) @type
  (pn_local)? @property)

; Namespace in prefix declarations
(prefix_id
  (namespace) @type.builtin)
(sparql_prefix
  (namespace) @type.builtin)

; Blank nodes
(blank_node_label) @variable.special
(anon) @variable.special

; Literals
(string) @string

(rdf_literal
  (string) @string
  (lang_tag)? @attribute)

(rdf_literal
  (string) @string
  "^^" @operator
  (_) @type)

; Boolean literals
(boolean_literal) @constant.builtin

; Numeric literals
(integer) @number
(decimal) @number
(double) @number

; Language tags
(lang_tag) @attribute

; Escape sequences in strings
(echar) @string.escape

; Punctuation - Delimiters
[
  "."
  ","
  ";"
] @punctuation.delimiter

; Datatype operator
"^^" @operator

; Punctuation - Brackets
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
