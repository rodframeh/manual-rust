[TOC]
# Sintaxis
## Estilo
- Por lo general se utilizar snake_case para construcciones de `nivel de valor` y CamelCase para construcciones de `nivel de tipo`.
- Puede variar dependiendo del ítem, esto está determinado en el RFC #430.

| **Elemento**             | **Convención**                                       |
| ------------------------ | ---------------------------------------------------- |
| Crates                   | snake_case (pero preferible una sola palabra)        |
| Módulos                  | snake_case                                           |
| Tipos                    | CamelCase                                            |
| Traits                   | CamelCase                                            |
| Variantes enum           | CamelCase                                            |
| Funciones                | snake_case                                           |
| Métodos                  | snake_case                                           |
| Constructores generales  | new o con_mas_detalles                               |
| Conversión constructores | desde_algun_otro_tipo                                |
| Variables locales        | snake_case                                           |
| Variables estáticas      | SCREAMING_SNAKE_CASE                                 |
| Constantes               | SCREAMING_SNAKE_CASE                                 |
| Tipo parámetros          | concise CamelCase, usualmente una letra mayuscula: T |
| Lifetimes                | short, lowercase: 'a                                 |
## Identado
- No se usa tab para identar, sino 4 espacios
``` rust
fn main(){
    println!("Hello, world");
1234
}
```
## Extensión
- Los archivos en rust tiene la extensión .rs `main.rs`.
## Función `main()`
- La palabra clave `fn` se usa para definir una función. 
- `main()` es una función predefinida que actúa como punto de entrada al programa, es un orquestador y no tiene parámetros
``` rust
fn main(){
    // Codigo rust
}
```
## Declaraciones
- Las declaraciones terminan con `;`, las expresiones no, pero si se le añade se les convierten en declaraciones.
- Las declaraciones no retornan ningún valor.
``` rust
fn main(){
    let saludo; // se declara la variable saludo
    saludo=String::from("Hello, world"); // se asigna una cadena a la variable saludo, se convierte esta expresion en una declaracion para que pueda ejecutarse la siguiente linea
    println!("{}",saludo)// es una expresion y puede o no terminar en ";" ya que no existe ninguna linea siguiente a ejecutarse
}
```
## Comentarios
```lexer
LINE_COMMENT :
// (~[/ !] | //) ~\n*	|	//

BLOCK_COMMENT :
/* (~[* !] | ** | BlockCommentOrDoc) (BlockCommentOrDoc | ~*/)* */	|	/**/	|	/***/

INNER_LINE_DOC :
//! ~[\n IsolatedCR]*

INNER_BLOCK_DOC :
/*! ( BlockCommentOrDoc	|	~[*/ IsolatedCR] )* */

OUTER_LINE_DOC :
/// (~/ ~[\n IsolatedCR]*)?

OUTER_BLOCK_DOC :
/** (~* | BlockCommentOrDoc ) (BlockCommentOrDoc | ~[*/ IsolatedCR])* */

BlockCommentOrDoc :
BLOCK_COMMENT	|	OUTER_BLOCK_DOC	|	INNER_BLOCK_DOC

IsolatedCR :
A \r not followed by a \n
```
- Permiten agregar información adicional.
- Se agregan encima de la línea de código.
- El compilador ignora los comentarios.
### Tipos según su interpretación
- **Comentarios no documentales**: Se interpretan como una forma de espacio en blanco, se tiene los comentario de linea `//` y de bloque `/* ... */` 
- **Comentarios documentales**: Se interpretan como una sintaxis especial para los atributos de doc, es decir son equivalentes a `#[doc="..."]`,  se tiene los comentarios de linea `///` y de bloque `/** ...*/`
- **Comentarios documentales aplicados al padre**: Son comentarios de documentos que se aplican al padre del comentario, se tiene los comentarios de linea `//!` y de bloque `/*! ...*/`
``` rust
fn main(){
    // Comentarios de una línea
    /* 
    Comentarios de 
    varias líneas
    */
}
```