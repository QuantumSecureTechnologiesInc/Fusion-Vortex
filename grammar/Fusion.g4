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
