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
## Instrucciones | Sentencias | Statements
```
Statement :
  ;
  | Item
  | LetStatement
  | ExpressionStatement
  | MacroInvocationSemi
```
- Sirven principalmente para contener y secuenciar explicitamente la evaluacion de expresiones.
- Es un componente de un bloque, que asu vez es componente de una expresion o funcion externa
### Las instrucciones de declaracion
- Es aquella que introduce uno o más names en el bloque de instrucción adjunto
- Los nombres declarados pueden denotar nuevas variables o nuevos items
#### Declaraciones de item
``` rust
#![allow(unused_variables)]
fn main() {
    fn outer() {
      let outer_var = true;
      fn inner() { /* outer_var is not in scope here */ }
      inner();
    }
}
```
- Tiene la forma sintactica identica a una declaracion de un item en un modulo
- Las declaraciones de un item dentro de un bloque de declaracion restringe su alcance (scope) al bloque que contiene la declaracion
- El item no tiene un path canonico, ni tampoco hay sub-items que pueda declarar
- La excepcion a esto es que los items asociados definidos por las implementaciones todavia son accesibles en ambitos externos, siempre que el item sea accesible,
#### Instrucciones `let` 
```
LetStatement :
  OuterAttribute* let Pattern ( : Type )? (= Expression )? ;
```
- Introduce un nuevo conjunto de variables, daoo por un patron irrefutable
- El patron es seguido opcionalmente por una anotacion de tipo y luego opcionalmente por una expresion inicializadora
- Cuando no se proporciona una anotación de tipo, el compilador inferira el tipo o señalara un error si no hay suficiente informacion de tipo disponible para una inferencia definitiva
- Cualquier variable introducida por una declaracion de variable es visible desde el punto de declaracion hasta el final del alcance (scope) del bloque que lo encierra
### Las instrucciones de expresion
```
ExpressionStatement :
  ExpressionWithoutBlock ;	|	ExpressionWithBlock
```
- Es aquella que evalua una expresion e ignora su resultado, por lo tanto, el proposito es desencadenar los efectos de evaluar su expresion
- Una expresion que consiste en una `ExpressionWithBlock` o una expresion de flujo de control, si se usa en un contexto donde se permite una instruccion, puede omitirse el punto y coma final.




- Las declaraciones terminan con `;`, las expresiones no, pero si se le añade se les convierten en declaraciones.
- Las declaraciones no retornan ningún valor.
``` rust
fn main(){
    let saludo; // se declara la variable saludo
    saludo=String::from("Hello, world"); // se asigna una cadena a la variable saludo, se convierte esta expresion en una declaracion para que pueda ejecutarse la siguiente linea
    println!("{}",saludo)// es una expresion y puede o no terminar en ";" ya que no existe ninguna linea siguiente a ejecutarse
}
```
## Expresiones
- En el lenguaje Rust la mayoria de las formas que producen valor o causan efecto estan dirigidas por la categoria de sintaxis de expresiones.
- Cada tipo de expresion normalmente puede anidarse dentro de otro tipo de expresion
- Las reglas para la evaluacion de expresiones implican especificar tanto el valor producido por la expresion como el orden en que se evaluan sus subexpresiones
- Puede tener dos roles
  - Siempre produce un valor
  - Puede tener efectos secundarios
- Una expresion se evalua como un valor y  tiene efectos durante la evaluacion
- Muchas expresiones tiene subexpresiones (operandos)
- La estructura de las expresiones dicta la estructura de la ejecucion: 
  - Si evaluar o no las subexpresiones al evaluar la expresion
  - El orden en el que evalua las subexpresiones
  - Como combinar los valores de las subexpresiones para obtener el valor de la expresion
- Los bloques, las instrucciones, las expresiones pueden anidarse recursivamente entre a una profundidad arbitraria
### Precedencia de expresion
### Expresiones de lugar y expresiones de valor
#### Tipos movidos y copiados
#### Mutabilidad
#### Lifetimes temporales
#### Prestamos implicitos
### Sobrecargando traits
### Atributos de expresion