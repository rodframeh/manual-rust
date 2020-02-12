# Comentarios
``` rust
fn main(){
    // Comentarios de una línea
    /* 
    Comentarios de 
    varias líneas
    */
}
```
- Permiten agregar información adicional.
- Se agregan encima de la línea de código.
- El compilador ignora los comentarios.
### Tipos según su interpretación
- **Comentarios no documentales**: Se interpretan como una forma de espacio en blanco, se tiene los comentario de linea `//` y de bloque `/* ... */` 
- **Comentarios documentales**: Se interpretan como una sintaxis especial para los atributos de doc, es decir son equivalentes a `#[doc="..."]`,  se tiene los comentarios de linea `///` y de bloque `/** ...*/`
- **Comentarios documentales aplicados al padre**: Son comentarios de documentos que se aplican al padre del comentario, se tiene los comentarios de linea `//!` y de bloque `/*! ...*/`
### Analizador léxico
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