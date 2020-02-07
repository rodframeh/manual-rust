[TOC]
# Especificación
## Notacion
| Notation          | Meaning                                     |
| ----------------- | ------------------------------------------- |
| CAPITAL           | Un token producido por el analizador lexico |
| *ItalicCamelCase* | Una produccion sintáctica                   |
| `string`          | Los caracteres exactos                      |
| \x                | El caracter representado por el escape      |
| x?                | Un item opcional                            |
| x*                | 0 o más de x                                |
| x+                | 1 o más de x                                |
| xa..b             | a a b repeticiones de x                     |
| \|                | Uno u otro                                  |
| [ ]               | Todos los caracteres listados               |
| [ - ]             | Todos los caracteres listados en el rango   |
| ~[ ]              | Todos los caracteres, excepto los listados  |
| ~`string`         | Todos los caracteres, excepto la secuencia  |
| ( )               | Grupos de elementos                         |

## Comentarios
- **Comentarios no documentales**: Se interpretan como una forma de espacio en blanco, se tiene los comentario de linea `//` y de bloque `/* ... */` 
- **Comentarios documentales**: Se interpretan como una sintaxis especial para los atributos de doc, es decir son equivalentes a `#[doc="..."]`,  se tiene los comentarios de linea `///` y de bloque `/** ...*/`
- **Comentarios documentales aplicados al padre**: Son comentarios de documentos que se aplican al padre del comentario, se tiene los comentarios de linea `//!` y de bloque `/*! ...*/`
## Tokens
- Son producciones primitivas en la gramática definida por lenguajes reguales (no recursivos)
- Tipos de tokens
  - Palabras clave
  - Identificadores
  - Literales
  - Lifetimes
  - Puntuación
  - Delimitadores
### Palabras clave
#### Estrictas
```
KW_AS : as
KW_ASYNC : async
KW_AWAIT : await
KW_BREAK : break
KW_CONST : const
KW_CONTINUE : continue
KW_CRATE : crate
KW_DYN : dyn
KW_ELSE : else
KW_ENUM : enum
KW_EXTERN : extern
KW_FALSE : false
KW_FN : fn
KW_FOR : for
KW_IF : if
KW_IMPL : impl
KW_IN : in
KW_LET : let
KW_LOOP : loop
KW_MATCH : match
KW_MOD : mod
KW_MOVE : move
KW_MUT : mut
KW_PUB : pub
KW_REF : ref
KW_RETURN : return
KW_SELFVALUE : self
KW_SELFTYPE : Self
KW_STATIC : static
KW_STRUCT : struct
KW_SUPER : super
KW_TRAIT : trait
KW_TRUE : true
KW_TYPE : type
KW_UNSAFE : unsafe
KW_USE : use
KW_WHERE : where
KW_WHILE : while
```
- Solo se pueden usar en contextos correctos
- No pueden usarse como nombres de:
  - Items
  - Variables y parámetros de función
  - Campos y variantes
  - Tipos de parámetros
  - Parámetros de lifetime o labels de bucle
  - Macros o atributos
  - Macro placeholders
  - Crates
#### Reservadas
```
KW_ABSTRACT: abstract
KW_BECOME: become
KW_BOX: box
KW_DO: do
KW_FINAL: final
KW_MACRO: macro
KW_OVERRIDE: override
KW_PRIV: priv
KW_TYPEOF: typeof
KW_TRY: try
KW_UNSIZED: sin unsized
KW_VIRTUAL: virtual
KW_YIELD: yield
```
- Estas palabras clave aun no se utilizan, pero estan reservadas para un futuro uso
#### Débiles
```
KW_DYN: dyn 
KW_UNION: union
KW_STATICLIFETIME: 'static
```
- Estas palabras clave tienen un signado especial solo en ciertos contextos
### Identificador
```
IDENTIFIER_OR_KEYWORD : [a-z A-Z] [a-z A-Z 0-9 _]* | _ [a-z A-Z 0-9 _]+
RAW_IDENTIFIER : r# IDENTIFIER_OR_KEYWORD
NON_KEYWORD_IDENTIFIER : IDENTIFIER_OR_KEYWORD
IDENTIFIER : NON_KEYWORD_IDENTIFIER | RAW_IDENTIFIER
```
- Es cualquier cadena ASCII no vacía.
- Un identificador raw, puede usar cualquier palabra clave estricta o reservada, excepto crate, self, super, Self.
### Literales
- Es una expresión que consiste en un solo token, en lugar de una secuencia de tokens
- Es una forma de expresion constante, por lo que se evalua en tiempo de compilación.
- Denota inmediata y directamente el valor al que evalúa, en lugar de referirse a él por su nombre o alguna otra regla de evaluación
#### Literales de caracter
```
CHAR_LITERAL: ' ( ~[' \ \n \r \t] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE ) '
QUOTE_ESCAPE: \' | \"
ASCII_ESCAPE: \x OCT_DIGIT HEX_DIGIT  | \n | \r | \t | \\ | \0
UNICODE_ESCAPE: \u{ ( HEX_DIGIT _* )1..6 }
```
- Es un unico caracter unicode encerrado dentro de comillas simples
#### Literales de string
```
STRING_LITERAL: " ( ~[" \ IsolatedCR] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE | STRING_CONTINUE )* "
STRING_CONTINUE: \ followed by \n
```
- Es una secuencia de cualquier carácter unicode encerrado dentro de dos comillas dobles
####  Literales de byte
```
BYTE_LITERAL:  b' ( ASCII_FOR_CHAR | BYTE_ESCAPE ) '
ASCII_FOR_CHAR:  any ASCII (i.e. 0x00 to 0x7F), except ', \, \n, \r or \t
BYTE_ESCAPE:  \x HEX_DIGIT HEX_DIGIT | \n | \r | \t | \\ | \0
```
- 
####  Literales de string byte
```
BYTE_STRING_LITERAL:  b" ( ASCII_FOR_STRING | BYTE_ESCAPE | STRING_CONTINUE )* "

ASCII_FOR_STRING: any ASCII (i.e 0x00 to 0x7F), except ", \ and IsolatedCR
```
-
####  Literales de numeros
##### Literales enteros
```
INTEGER_LITERAL: ( DEC_LITERAL | BIN_LITERAL | OCT_LITERAL | HEX_LITERAL ) INTEGER_SUFFIX?
DEC_LITERAL: DEC_DIGIT (DEC_DIGIT|_)*
TUPLE_INDEX: 0    | NON_ZERO_DEC_DIGIT DEC_DIGIT*
BIN_LITERAL: 0b (BIN_DIGIT|_)* BIN_DIGIT (BIN_DIGIT|_)*
OCT_LITERAL: 0o (OCT_DIGIT|_)* OCT_DIGIT (OCT_DIGIT|_)*
HEX_LITERAL: 0x (HEX_DIGIT|_)* HEX_DIGIT (HEX_DIGIT|_)*
BIN_DIGIT: [0-1]
OCT_DIGIT: [0-7]
DEC_DIGIT: [0-9]
NON_ZERO_DEC_DIGIT: [1-9]
HEX_DIGIT: [0-9 a-f A-F]
INTEGER_SUFFIX:  u8 | u16 | u32 | u64 | u128 | usize | i8 | i16 | i32 | i64 | i128 | isize
```
- 
##### Literales punto flotante
```
FLOAT_LITERAL : DEC_LITERAL . (not immediately followed by ., _ or an identifier) | DEC_LITERAL FLOAT_EXPONENT  | DEC_LITERAL . DEC_LITERAL FLOAT_EXPONENT? | DEC_LITERAL (. DEC_LITERAL)? FLOAT_EXPONENT? FLOAT_SUFFIX

FLOAT_EXPONENT :
   (e|E) (+|-)? (DEC_DIGIT|_)* DEC_DIGIT (DEC_DIGIT|_)*

FLOAT_SUFFIX :
   f32 | f64
```
- 
####  Literales de booleanos
```

```
- 
### Lifetimes
```
LIFETIME_TOKEN : ' IDENTIFIER_OR_KEYWORD | '_
LIFETIME_OR_LABEL : ' NON_KEYWORD_IDENTIFIER
```
- Los parámetros lifetime y las labels de etiqueta usan tokens LIFETIME_OR_LABEL
### Puntuación
- Son los símbolos como: 

  | Simbolo | Nombre     | Uso                                                          |
  | ------- | ---------- | ------------------------------------------------------------ |
  | +       | Plus       | Addition, Trait Bounds, Macro Kleene Matcher                 |
  | -       | Minus      | Subtraction, Negation                                        |
  | *       | Star       | Multiplication, Dereference, Raw Pointers, Macro Kleene Matcher |
  | /       | Slash      | Division                                                     |
  | %       | Percent    | Remainder                                                    |
  | ^       | Caret      | Bitwise and Logical XOR                                      |
  | !       | Not        | Bitwise and Logical NOT, Macro Calls, Inner Attributes, Never Type |
  | &       | And        | Bitwise and Logical AND, Borrow, References, Reference patterns |
  | \|      | Or         | Bitwise and Logical OR, Closures, Match                      |
  | &&      | AndAnd     | Lazy AND, Borrow, References, Reference patterns             |
  | \|\|    | OrOr       | Lazy OR, Closures                                            |
  | <<      | Shl        | Shift Left, Nested Generics                                  |
  | >>      | Shr        | Shift Right, Nested Generics                                 |
  | +=      | PlusEq     | Addition assignment                                          |
  | -=      | MinusEq    | Subtraction assignment                                       |
  | *=      | StarEq     | Multiplication assignment                                    |
  | /=      | SlashEq    | Division assignment                                          |
  | %=      | PercentEq  | Remainder assignment                                         |
  | ^=      | CaretEq    | Bitwise XOR assignment                                       |
  | &=      | AndEq      | Bitwise And assignment                                       |
  | \|=     | OrEq       | Bitwise Or assignment                                        |
  | <<=     | ShlEq      | Shift Left assignment                                        |
  | >>=     | ShrEq      | Shift Right assignment, Nested Generics                      |
  | =       | Eq         | Assignment, Attributes, Various type definitions             |
  | Err:520 | EqEq       | Equal                                                        |
  | !=      | Ne         | Not Equal                                                    |
  | >       | Gt         | Greater than, Generics, Paths                                |
  | <       | Lt         | Less than, Generics, Paths                                   |
  | >=      | Ge         | Greater than or equal to, Generics                           |
  | <=      | Le         | Less than or equal to                                        |
  | @       | At         | Subpattern binding                                           |
  | _       | Underscore | Wildcard patterns, Inferred types                            |
  | .       | Dot        | Field access, Tuple index                                    |
  | ..      | DotDot     | Range, Struct expressions, Patterns                          |
  | ...     | DotDotDot  | Variadic functions, Range patterns                           |
  | ..=     | DotDotEq   | Inclusive Range, Range patterns                              |
  | ,       | Comma      | Various separators                                           |
  | ;       | Semi       | Terminator for various items and statements, Array types     |
  | :       | Colon      | Various separators                                           |
  | ::      | PathSep    | Path separator                                               |
  | ->      | RArrow     | Function return type, Closure return type                    |
  | Err:510 | FatArrow   | Match arms, Macros                                           |
  | #       | Pound      | Attributes                                                   |
  | $       | Dollar     | Macros                                                       |
  | ?       | Question   | Question mark operator, Questionably sized                   |
### Delimitadores
- Un bracket abierto siempre debe estar emparejado a un breaket cerrado

- Los brackets y los tokens dentro de ellos se les denomina arbol de tokens en macros

- Tipos
  
  | Bracket | Tipo            |
  | ------- | --------------- |
  | `{` `}` | Llaves   |
  | `[` `]` | Corchetes |
  | `(` `)` | Paréntesis     |
## Declaración

## Expresión
## Atributo
## Bloque
```
BlockExpression:
{
  InnerAttribute*
  Declaraciones?
}

Declaraciones :
  Declaracion+ | Declaracion + ExpressionWithoutBlock
   | ExpressionWithoutBlock
```
- 