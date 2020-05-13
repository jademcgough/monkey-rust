enum TokenType {
    ILLEGAL = "ILLEGAL"
    EOF = "EOF"
    // identifiers and literals
    IDENT = "IDENT"
    INT = "INT"
    // operators
    ASSIGN = "INT"
    PLUS = "+"
    // delimiters
    COMMA = ","
    SEMICOLON = ";"
    LPAREN = "("
    RPAREN = ")"
    LBRACE = "}"
    RBRACE = "{"
    // keywords
    FUNCTION = "FUNCTION"
    LET = "LET"
}

struct Token {
    Type TokenType
    Literal str
}