// Fusion v2.0 Vortex Grammar
// Aligned with actual compiler (crates/fuc/src/parser.fu) on 2026-06-25.
// Previous version used Python-style 'class', ':', 'and/or/not' which do NOT
// match the hand-written Lexer + chumsky-based Parser.

grammar Fusion;

// ─── Top-level program ────────────────────────────────────────────────────────

program: declaration* EOF;

declaration:
    functionDecl
    | structDecl
    | enumDecl
    | traitDecl
    | implBlock
    | globalVar
    | externDecl
    | moduleDecl
    | useDecl
    | typeAlias
    ;

// ─── Module & Use ─────────────────────────────────────────────────────────────

moduleDecl: PUB? MOD IDENTIFIER LBRACE declaration* RBRACE;
useDecl: USE usePath (COMMA usePath)* SEMICOLON;
usePath: IDENTIFIER (COLONCOLON IDENTIFIER)* (COLONCOLON LBRACE IDENTIFIER (COMMA IDENTIFIER)* RBRACE)?;

// ─── Type alias ────────────────────────────────────────────────────────────────

typeAlias: PUB? TYPE IDENTIFIER ASSIGN type SEMICOLON;

// ─── Extern declaration ────────────────────────────────────────────────────────

externDecl: EXTERN FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? SEMICOLON;

// ─── Function declaration ──────────────────────────────────────────────────────

functionDecl:
    PUB? ASYNC? FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? block
    ;

// ─── Struct declaration ────────────────────────────────────────────────────────

structDecl:
    PUB? STRUCT IDENTIFIER LBRACE structField* RBRACE
    ;

structField: PUB? IDENTIFIER COLON type COMMA?;

// ─── Enum declaration ──────────────────────────────────────────────────────────

enumDecl:
    PUB? ENUM IDENTIFIER LBRACE enumVariant* RBRACE
    ;

enumVariant: IDENTIFIER (LPAREN type RPAREN)? COMMA?;

// ─── Trait declaration ─────────────────────────────────────────────────────────

traitDecl:
    PUB? TRAIT IDENTIFIER LBRACE traitItem* RBRACE
    ;

traitItem:
    FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? (block | SEMICOLON)
    ;

// ─── Impl block ────────────────────────────────────────────────────────────────

implBlock:
    IMPL IDENTIFIER (FOR IDENTIFIER)? LBRACE functionDecl* RBRACE
    ;

// ─── Parameter list ────────────────────────────────────────────────────────────

paramList: param (COMMA param)*;
param: IDENTIFIER COLON type;

// ─── Type annotations ──────────────────────────────────────────────────────────

type:
    INT | FLOAT | STRING | BOOL | U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64 | UNIT
    | IDENTIFIER
    | LBRACK type SEMICOLON expression RBRACK   // Fixed-size array: [int; 4096]
    | LBRACK type RBRACK                        // Slice/dynamic array
    | type QUESTION                             // Optional type
    | LPAREN typeList RPAREN ARROW type         // Function type
    | AMP type                                  // Reference (&T)
    | AMP MUT type                              // Mutable reference (&mut T)
    ;

typeList: type (COMMA type)*;

// ─── Block statements ──────────────────────────────────────────────────────────

block: LBRACE statement* RBRACE;

statement:
    varDecl
    | assignment
    | ifStmt
    | whileStmt
    | forStmt
    | matchStmt
    | returnStmt
    | expressionStmt
    | breakStmt
    | continueStmt
    ;

varDecl: LET IDENTIFIER (COLON type)? ASSIGN expression SEMICOLON;
assignment: IDENTIFIER ASSIGN expression SEMICOLON;
ifStmt: IF expression block (ELSE (ifStmt | block))?;
whileStmt: WHILE expression block;
forStmt: FOR IDENTIFIER IN expression block;
matchStmt: MATCH expression LBRACE matchArm* RBRACE;
matchArm: pattern ARROW expression COMMA?;
pattern: IDENTIFIER (LPAREN IDENTIFIER RPAREN)? | INT_LIT | STRING_LIT | UNDERSCORE;
returnStmt: RETURN expression? SEMICOLON;
expressionStmt: expression SEMICOLON;
breakStmt: BREAK SEMICOLON;
continueStmt: CONTINUE SEMICOLON;

// ─── Expressions ───────────────────────────────────────────────────────────────

expression: logicalOr;

logicalOr: logicalAnd (OROR logicalAnd)*;
logicalAnd: equality (ANDAND equality)*;
equality: comparison ((EQ | NE) comparison)*;
comparison: additive ((LT | LE | GT | GE) additive)*;
additive: multiplicative ((PLUS | MINUS) multiplicative)*;
multiplicative: unary ((MUL | DIV | MOD) unary)*;
unary: (BANG | MINUS | AMP | AMP MUT) unary | postfix;
postfix: primary (DOT IDENTIFIER | LBRACK expression RBRACK | LPAREN argList? RPAREN)*;

primary:
    INT_LIT | FLOAT_LIT | STRING_LIT | TRUE | FALSE
    | IDENTIFIER
    | LPAREN expression RPAREN
    | arrayLit
    | structLit
    ;

// ─── Composite literals ────────────────────────────────────────────────────────

arrayLit: LBRACK argList? RBRACK;
structLit: IDENTIFIER LBRACE fieldInit (COMMA fieldInit)* COMMA? RBRACE;
fieldInit: IDENTIFIER COLON expression;

// ─── Keywords ──────────────────────────────────────────────────────────────────

FN: 'fn';
STRUCT: 'struct';
ENUM: 'enum';
TRAIT: 'trait';
IMPL: 'impl';
LET: 'let';
CONST: 'const';
STATIC: 'static';
IF: 'if';
ELSE: 'else';
WHILE: 'while';
FOR: 'for';
IN: 'in';
MATCH: 'match';
RETURN: 'return';
BREAK: 'break';
CONTINUE: 'continue';
TRUE: 'true';
FALSE: 'false';
PUB: 'pub';
MOD: 'mod';
USE: 'use';
EXTERN: 'extern';
ASYNC: 'async';
AWAIT: 'await';
TYPE: 'type';
WHERE: 'where';
MUT: 'mut';

// ─── Built-in type keywords ────────────────────────────────────────────────────

INT: 'int';
FLOAT: 'float';
STRING: 'string';
BOOL: 'bool';
U8: 'u8';
U16: 'u16';
U32: 'u32';
U64: 'u64';
I8: 'i8';
I16: 'i16';
I32: 'i32';
I64: 'i64';
UNIT: '()';

// ─── Operators ─────────────────────────────────────────────────────────────────

PLUS: '+';
MINUS: '-';
MUL: '*';
DIV: '/';
MOD: '%';
ASSIGN: '=';
EQ: '==';
NE: '!=';
LT: '<';
LE: '<=';
GT: '>';
GE: '>=';
ANDAND: '&&';
OROR: '||';
BANG: '!';
AMP: '&';

// ─── Delimiters ────────────────────────────────────────────────────────────────

LPAREN: '(';
RPAREN: ')';
LBRACE: '{';
RBRACE: '}';
LBRACK: '[';
RBRACK: ']';
COMMA: ',';
COLON: ':';
SEMICOLON: ';';
DOT: '.';
ARROW: '->';
QUESTION: '?';
COLONCOLON: '::';
UNDERSCORE: '_';

// ─── Identifiers and Literals ──────────────────────────────────────────────────

IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
INT_LIT: [0-9]+;
FLOAT_LIT: [0-9]+ DOT [0-9]+;
STRING_LIT: '"' (~["\\\r\n])* '"';

// ─── Whitespace & Comments ─────────────────────────────────────────────────────

WS: [ \t\n\r]+ -> skip;
COMMENT: '//' ~[\r\n]* -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
grammar Fusion;

// Top-level program structure
program: declaration* EOF;

declaration: 
    functionDecl
    | classDecl
    | traitDecl
    | globalVar
    | moduleDecl
    ;

// Function declaration
functionDecl:
    FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? COLON block
    | FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? LBRACE block RBRACE
    ;

// Class declaration
classDecl:
    CLASS IDENTIFIER (LBRACE classBody RBRACE | COLON classBody)
    ;

classBody: (fieldDecl | methodDecl)*;
fieldDecl: IDENTIFIER COLON type;
methodDecl: FN IDENTIFIER LPAREN paramList? RPAREN (ARROW type)? COLON block;

// Parameter list
paramList: param (COMMA param)*;
param: IDENTIFIER COLON type;

// Type annotations
type:
    INT | FLOAT | STRING | BOOL
    | IDENTIFIER
    | type LBRACK RBRACK           // Array type
    | type QUESTION                 // Optional type
    | LPAREN typeList RPAREN ARROW type  // Function type
    ;

typeList: type (COMMA type)*;

// Block statements
block: statement*;
statement:
    varDecl
    | assignment
    | ifStmt
    | whileStmt
    | forStmt
    | returnStmt
    | expressionStmt
    | breakStmt
    | continueStmt
    ;

varDecl: LET IDENTIFIER (COLON type)? ASSIGN expression;
assignment: IDENTIFIER ASSIGN expression;
ifStmt: IF expression COLON block (ELSE COLON block)?;
whileStmt: WHILE expression COLON block;
forStmt: FOR IDENTIFIER IN expression COLON block;
returnStmt: RETURN expression?;
expressionStmt: expression;
breakStmt: BREAK;
continueStmt: CONTINUE;

// Expressions
expression: logicalOr;

logicalOr: logicalAnd (OR logicalAnd)*;
logicalAnd: equality (AND equality)*;
equality: comparison ((EQ | NE) comparison)*;
comparison: additive ((LT | LE | GT | GE) additive)*;
additive: multiplicative ((PLUS | MINUS) multiplicative)*;
multiplicative: unary ((MUL | DIV | MOD) unary)*;
unary: (NOT | MINUS) unary | postfix;
postfix: primary (DOT IDENTIFIER | LBRACK expression RBRACK)*;

primary:
    INT_LIT | FLOAT_LIT | STRING_LIT | TRUE | FALSE
    | IDENTIFIER
    | LPAREN expression RPAREN
    | functionCall
    | arrayLit
    | mapLit
    ;

functionCall: IDENTIFIER LPAREN argList? RPAREN;
argList: expression (COMMA expression)*;
arrayLit: LBRACK argList? RBRACK;
mapLit: LBRACE mapEntry (COMMA mapEntry)* RBRACE;
mapEntry: expression COLON expression;

// Keywords
FN: 'fn';
CLASS: 'class';
TRAIT: 'trait';
LET: 'let';
IF: 'if';
ELSE: 'else';
WHILE: 'while';
FOR: 'for';
IN: 'in';
RETURN: 'return';
BREAK: 'break';
CONTINUE: 'continue';
TRUE: 'true';
FALSE: 'false';

// Types
INT: 'int';
FLOAT: 'float';
STRING: 'string';
BOOL: 'bool';

// Operators
PLUS: '+';
MINUS: '-';
MUL: '*';
DIV: '/';
MOD: '%';
ASSIGN: '=';
EQ: '==';
NE: '!=';
LT: '<';
LE: '<=';
GT: '>';
GE: '>=';
AND: 'and';
OR: 'or';
NOT: 'not';

// Delimiters
LPAREN: '(';
RPAREN: ')';
LBRACE: '{';
RBRACE: '}';
LBRACK: '[';
RBRACK: ']';
COMMA: ',';
COLON: ':';
SEMICOLON: ';';
DOT: '.';
ARROW: '->';
QUESTION: '?';

// Identifiers and Literals
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
INT_LIT: [0-9]+;
FLOAT_LIT: [0-9]+ DOT [0-9]+;
STRING_LIT: '"' (~["\\\r\n])* '"';

WS: [ \t\n\r]+ -> skip;
COMMENT: '//' ~[\r\n]* -> skip;
