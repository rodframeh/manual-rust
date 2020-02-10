[TOC]
# Especificación
## Notación del lexer
| Notación          | Significado                                 |
| ----------------- | ------------------------------------------- |
| CAPITAL           | Un token producido por el analizador léxico |
| *ItalicCamelCase* | Una producción sintáctica                   |
| `string`          | Los caracteres exactos                      |
| \x                | El carácter representado por el escape      |
| x?                | Un item opcional                            |
| x*                | 0 o más de x                                |
| x+                | 1 o más de x                                |
| x a..b            | a a b repeticiones de x                     |
| \|                | Uno u otro                                  |
| [ ]               | Todos los caracteres listados               |
| [ - ]             | Todos los caracteres listados en el rango   |
| ~[ ]              | Todos los caracteres, excepto los listados  |
| ~`string`         | Todos los caracteres, excepto la secuencia  |
| ( )               | Grupos de elementos                         |
## Tokens
- Son producciones primitivas en la gramática definida por lenguajes regulares (no recursivos)
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
- Estas palabras clave aun no se utilizan, pero están reservadas para un futuro uso
#### Débiles
```
KW_DYN: dyn 
KW_UNION: union
KW_STATICLIFETIME: 'static
```
- Estas palabras clave tienen un signado especial solo en ciertos contextos
### Identificador
```
IDENTIFIER_OR_KEYWORD :
[a-z A-Z][a-z A-Z 0-9 _]*	|	_[a-z A-Z 0-9 _]+

RAW_IDENTIFIER : 
r# IDENTIFIER_OR_KEYWORD

NON_KEYWORD_IDENTIFIER : 
IDENTIFIER_OR_KEYWORD

IDENTIFIER : 
NON_KEYWORD_IDENTIFIER | RAW_IDENTIFIER
```
- Permiten identificar a las variables y funciones.
- Es cualquier cadena ASCII no vacía.
- Un identificador raw, puede usar cualquier palabra clave estricta o reservada, excepto crate, self, super, Self.
### Escapes
- Es un carácter que invoca una interpretación alternativa sobre los caracteres posteriores en una secuencia de caracteres.

- Un escapes inicia con (`\`) y continuado con `x`(hex>ASCII), `u`(hex>Unicode), `n`(LF), `r`(CR), `t`(HT), `0`(NUL), `/`(el mismo).

- **ASCII escapes**

  |        | Nombre                                                       |
  | ------ | ------------------------------------------------------------ |
  | `\x41` | Código de carácter de 7-bit (exactamente 2 dígitos, hasta `0x7F`) |
  | `\n`   | Nueva linea (Newline)                                        |
  | `\r`   | Retorno de carro (Carriage return)                           |
  | `\t`   | Tab                                                          |
  | `\\`   | Backslash                                                    |
  | `\0`   | Null                                                         |

- **Byte escapes**

  |        | Nombre                                              |
  | ------ | --------------------------------------------------- |
  | `\x7F` | Código de carácter de 8-bit (exactamente 2 dígitos) |
  | `\n`   | Nueva linea (Newline)                               |
  | `\r`   | Retorno de carro (Carriage return)                  |
  | `\t`   | Tab                                                 |
  | `\\`   | Backslash                                           |
  | `\0`   | Null                                                |

- **Unicode escapes**

  |            | **Nombre**                                               |
  | ---------- | -------------------------------------------------------- |
  | `\u{7FFF}` | Código de caracteres Unicode de 24-bit (hasta 6 dígitos) |

- **Quote escapes**

  |      | Nombre       |
  | ---- | ------------ |
  | `\'` | Single quote |
  | `\"` | Double quote |
### Suffixes

- Es un identificador **non-raw** que sigue a la parte primaria de un literal (sin espacio en blanco).

- *Cualquier* tipo de literal con cualquier suffix es valido como un token.

- Los suffixes son rechazados en tokens literales no numéricos y los tokens literales numéricos se aceptan solo con los siguientes suffixes

  | **Enteros**                                                  | **Punto flotante** |
  | ------------------------------------------------------------ | ------------------ |
  | u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize | f32, f64           |

### Literales
- Es una expresión que consiste en un solo token, en lugar de una secuencia de tokens.
- **Es una forma de expresión constante, por lo que se evalúa en tiempo de compilación**.
- Denota inmediata y directamente el valor al que evalúa, en lugar de referirse a él por su nombre o alguna otra regla de evaluación.
#### Literales de carácter
```
CHAR_LITERAL: 
' ( ~[' \ \n \r \t]	|	QUOTE_ESCAPE	|	ASCII_ESCAPE	|	UNICODE_ESCAPE ) '

QUOTE_ESCAPE: 
\'	|	\"

ASCII_ESCAPE: 
\x OCT_DIGIT HEX_DIGIT 	|	\n	|	\r	|	\t	|	\\	|	\0

UNICODE_ESCAPE: 
\u{ ( HEX_DIGIT _* ) 1..6 }
```
- Es **un** carácter **Unicode** encerrado dentro de comillas simples (`'`).
#### Literales de string
```
STRING_LITERAL: 
" ( ~[" \ IsolatedCR] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE | STRING_CONTINUE )* "

STRING_CONTINUE: 
\ followed by \n
```
- Es una secuencia de **uno a muchos** caracteres **Unicode** encerrados dentro de dos comillas dobles (`"`).
- Algunos escapes están disponibles en caracteres o literales de string.
- Ejemplo:
``` rust

#![allow(unused_variables)]
fn main() {
  let a = "foobar";
  let b = "foo\
         bar";

  assert_eq!(a,b);
}

```
#### Literales de raw string
```
RAW_STRING_LITERAL :
r RAW_STRING_CONTENT

RAW_STRING_CONTENT :
" ( ~IsolatedCR )* "	|	# RAW_STRING_CONTENT #
```
- **No procesan ningún escape**.
- Comienza con **un** carácter `r`, seguido de **cero o más** caracteres `#`, **un** carácter `"`, una secuencia de **uno a muchos** caracteres **Unicode** y termina con un carácter `"` y la misma cantidad de caracteres `#` que se establecieron al inicio.
- Ejemplo
``` rust
#![allow(unused_variables)]
fn main() {
  "foo"; r"foo";                     // foo
  "\"foo\""; r#""foo""#;             // "foo"

  "foo #\"# bar";
  r##"foo #"# bar"##;                // foo #"# bar

  "\x52"; "R"; r"R";                 // R
  "\\x52"; r"\x52";                  // \x52
}
```
#### Literales de byte
```
BYTE_LITERAL:  
b' ( ASCII_FOR_CHAR | BYTE_ESCAPE ) '

ASCII_FOR_CHAR:  
any ASCII (i.e. 0x00 to 0x7F), except ', \, \n, \r or \t

BYTE_ESCAPE:  
\x HEX_DIGIT HEX_DIGIT	|	\n | \r | \t | \\ | \0
```
- Es **un** carácter **ASCII** o un escape precedido por los caracteres (`b`)  y  (`'`),  y seguido por el carácter (`'`) .
- Si el carácter  (`'`)  esta presente dentro del literal, debe ser precedido por el carácter (`\`).
- Es equivalente a un número de 8 bits sin signo `u8`.
####  Literales de string byte
```
BYTE_STRING_LITERAL:  b" ( ASCII_FOR_STRING | BYTE_ESCAPE | STRING_CONTINUE )* "

ASCII_FOR_STRING: any ASCII (i.e 0x00 to 0x7F), except ", \ and IsolatedCR
```
- Es una secuencia de **uno a muchos** caracteres **ASCII** y/o escapes, precedido por los caracteres  (`b`)  y  (`"`),  y seguido por el carácter  (`"`) .
- Si el carácter  (`"`)  esta presente dentro del literal, debe ser precedido por el carácter  (`\`).
- El tipo de literal de string bytes de longitud `n` es `&'static [u8; n]`
- Algunos escapes están disponibles en literales de string byte
####  Literales de raw string byte
```
RAW_BYTE_STRING_LITERAL :
br RAW_BYTE_STRING_CONTENT

RAW_BYTE_STRING_CONTENT :
" ASCII* (non-greedy) "	|	# RAW_STRING_CONTENT #

ASCII :
any ASCII (i.e. 0x00 to 0x7F)
```
- **No procesan ningún escape**.
- Comienza con el carácter `b`, seguido de `r`, seguido de cero o más caracteres `#` y un carácter `"`, una secuencia de **uno a muchos** caracteres **ASCII** y termina con un carácter `"` y la misma cantidad de caracteres `#` que se establecieron al inicio.
- Ejemplo
``` rust

#![allow(unused_variables)]
fn main() {
  b"foo"; br"foo";                     // foo
  b"\"foo\""; br#""foo""#;             // "foo"

  b"foo #\"# bar";
  br##"foo #"# bar"##;                 // foo #"# bar

  b"\x52"; b"R"; br"R";                // R
  b"\\x52"; br"\x52";                  // \x52
}

```
#### Literales enteros
```
INTEGER_LITERAL :
(DEC_LITERAL | BIN_LITERAL | OCT_LITERAL | HEX_LITERAL) INTEGER_SUFFIX?

DEC_LITERAL :
DEC_DIGIT (DEC_DIGIT|_)*

TUPLE_INDEX :
0	|	NON_ZERO_DEC_DIGIT DEC_DIGIT*

BIN_LITERAL :
0b (BIN_DIGIT|_)* BIN_DIGIT (BIN_DIGIT|_)*

OCT_LITERAL :
0o (OCT_DIGIT|_)* OCT_DIGIT (OCT_DIGIT|_)*

HEX_LITERAL :
0x (HEX_DIGIT|_)* HEX_DIGIT (HEX_DIGIT|_)*

BIN_DIGIT : 
[0-1]

OCT_DIGIT : 
[0-7]

DEC_DIGIT : 
[0-9]

NON_ZERO_DEC_DIGIT : 
[1-9]

HEX_DIGIT : 
[0-9 a-f A-F]

INTEGER_SUFFIX :
u8 | u16 | u32 | u64 | u128 | usize | i8 | i16 | i32 | i64 | i128 | isize
```
- Formas del literal entero:
  - Un **literal decimal** comienza con un digito decimal y continua con cualquier combinación de dígitos decimales y guiones bajos
  - Un **indice de tupla** es 0 o comienza con un dígito decimal distinto de cero y continua con cero o más digitos decimales. Se usan para referirse a los campos de las tuplas, estructuras de tupla y variantes de tupla
  - Un **literal hexadecimal** comienza con la secuencia de caracteres (`0x`) y continua como cualquier mezcla de uno o más dígitos hexadecimales y guiones bajos.
  - Un **literal octal** comienza con la secuencia de caracteres (`0o`) y continua como cualquier mezcla de uno o más dígitos octales y guiones bajos.
  - Un **literal binario** comienza con la secuencia de caracteres (`0b`) y continua como cualquier mezcla de uno o más digitos binarios y guiones bajos.
- Un literal entero puede ser seguido por un **sufijo entero**, que establece por la fuerza el tipo de dato del literal
- El tipo de un literal entero unsuffixed es determinado por la **inferencia de tipos**
  -  Puede determinarse de manera única a partir del contexto del programa, el literal entero unsuffixed tiene ese tipo de dato
  -  Si el contexto del programa restringe el tipo, el valor predeterminado es i32,
  -  Si el contexto del programa sobre-restringe el tipo, se considera un static type error.
- Ejemplos
``` rust
#![allow(unused_variables)]
fn main() {
  123;                               // tipo i32
  123i32;                            // tipo i32
  123u32;                            // tipo u32
  123_u32;                           // tipo u32
  let a: u64 = 123;                  // tipo u64

  0xff;                              // tipo i32
  0xff_u8;                           // tipo u8

  0o70;                              // tipo i32
  0o70_i16;                          // tipo i16

  0b1111_1111_1001_0000;             // tipo i32
  0b1111_1111_1001_0000i64;          // tipo i64
  0b________1;                       // tipo i32

  0usize;                            // tipo usize
}
```
#### Literales punto flotante
```
FLOAT_LITERAL :
DEC_LITERAL . (not immediately followed by ., _ or an identifier)
| DEC_LITERAL FLOAT_EXPONENT
| DEC_LITERAL . DEC_LITERAL FLOAT_EXPONENT?
| DEC_LITERAL (. DEC_LITERAL)? FLOAT_EXPONENT? FLOAT_SUFFIX

FLOAT_EXPONENT :
(e|E) (+|-)? (DEC_DIGIT|_)* DEC_DIGIT (DEC_DIGIT|_)*

FLOAT_SUFFIX :
f32 | f64
```
- Formas del literal punto flotante:
  - Literal decimal seguido de un (`.`) y otro literal decimal con un exponente opcional 
  - Literal decimal seguido de un exponente.
- Un literal punto flotante puede ser seguido por un **sufijo de punto flotante**, que establece por la fuerza el tipo de dato del literal.
- El tipo de un literal punto flotante unsuffixed es determinado por la **inferencia de tipos**
  -  Puede determinarse de manera única a partir del contexto del programa, el literal punto flotante unsuffixed tiene ese tipo de dato
  -  Si el contexto del programa restringe el tipo, el valor predeterminado es f64,
  -  Si el contexto del programa sobre-restringe el tipo, se considera un static type error.
-  Ejemplo
``` rust

#![allow(unused_variables)]
fn main() {
  123.0f64;        // type f64
  0.1f64;          // type f64
  0.1f32;          // type f32
  12E+99_f64;      // type f64
  let x: f64 = 2.; // type f64
}

```
####  Literales de booleanos
```
BOOLEAN_LITERAL :
true | false
```
- Los valores del tipo booleano pueden ser `true` o `false`
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