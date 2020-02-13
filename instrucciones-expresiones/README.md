# Instrucciones y expresiones
## Items
- Un item es un componente de un crate y estan organizados por un conjunto anidado de modulos
- Cada crate tiene un unico modulo anónimo "mas externo", todos los demás items de un crate tienen paths dentro de un arbol de modulos del crate
- Los items se determinan por completo en tiempo de compilacion, generalmente permanecen fijos durante la ejecución y puden residir en la memoria de solo lectura
- Tipos de items
  - modules
  - `extern crate` declarations
  - `use` declarations
  - function definitions
  - type definitions
  - struct definitions
  - enumeration definitions
  - union definitions
  - constant items
  - static items
  - trait definitions
  - implementations
  - `extern` blocks
- Algunos items forman un alcance (scope) implicito para la declaración de subelementos 
## Expresiones de bloque | Bloque
- Es una expresion de control de flujo y un ambito (scope) de namespace anónimo para items y declaraciones de variables.
- Se ejecuta secuencialmente sus instrucciones de declaracion de componentes no items y luego su expresión final opcional.
- Como un namespace anónimo, las declaraciones de items solo estan dentro del bloque en si y las variables declaradas con `let` estan dentro del alcance (scope) desde la siguiente instrucción hasta el final del bloque
- Sintaxis
```
BlockExpression :
{
  InnerAttribute*
  Statements?
}

Statements :
  Statement+
  | Statement+ ExpressionWithoutBlock
  | ExpressionWithoutBlock
```
- Por lo general se requiere que las instrucciones sigan con un punto y coma
- Excepciones
  - Las instrucciones de declaracion de items no necesitan ir seguidas de un punto y coma.
  - Las instrucciones de expresion generalmente requieren un punto y coma siguiente, excepto si su expresion externa es una expresion de control de flujo.
- El `;` no forma parte de la instrucción en si
- Al evaluar una expresion de bloque, cada instruccion se ejecuta secuencialmente, excepto las instrucciones de declaracion de items. Luego se ejecuta la expresion final si existiera
- Los bloques son siempre expresiones de valor y evaluan la ultima expresion en el contexto de expresion de valor, se usa para forzar el moviemiento de un valor.
## Instrucciones | Sentencias
- Es un componente de un bloque, expresion externa o funcion externa
### Las instrucciones de declaracion
- 
### Las instrucciones de expresion
- 

- Las declaraciones terminan con `;`, las expresiones no, pero si se le añade se les convierten en declaraciones.
- Las declaraciones no retornan ningún valor.
``` rust
fn main(){
    let saludo; // se declara la variable saludo
    saludo=String::from("Hello, world"); // se asigna una cadena a la variable saludo, se convierte esta expresion en una declaracion para que pueda ejecutarse la siguiente linea
    println!("{}",saludo)// es una expresion y puede o no terminar en ";" ya que no existe ninguna linea siguiente a ejecutarse
}
```