# Instrucciones y expresiones
## Items
- Un item es un componente de un crate y estan organizados por un conjunto anidado de modulos
- Cada crate tiene un unico modulo anónimo "mas externo", todos los demás items de un crate tienen paths dentro de un arbol de modulos del crate
- Los items se determinan por completo en tiempo de compilacion, generalmente permanecen fijos durante la ejecución y pueden residir en la memoria de solo lectura
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
``` rust

#![allow(unused_variables)]
fn main() {
	fn fn_call() {}
    let _: () = {
        fn_call();
    };
    let five: i32 = {
        fn_call();
        5
    };
	assert_eq!(5, five);
}

```
- Es una expresion de control de flujo y un ambito (scope) de namespace anónimo para items y declaraciones de variables.
- Ejecuta secuencialmente sus instrucciones de declaracion de componentes no items y luego su expresión final opcional.
- Como un namespace anónimo, las declaraciones de items solo estan dentro del bloque en si y las variables declaradas con `let` estan dentro del alcance (scope) desde la siguiente instrucción hasta el final del bloque
- Por lo general se requiere que las instrucciones sigan con un punto y coma
- Excepciones
  - Las instrucciones de declaracion de items no necesitan ir seguidas de un punto y coma.
  - Las instrucciones de expresion generalmente requieren un punto y coma siguiente, excepto si su expresion externa es una expresion de control de flujo.
- El `;` no forma parte de la instrucción en si
- Al evaluar una expresion de bloque, cada instruccion se ejecuta secuencialmente, excepto las instrucciones de declaracion de items. Luego se ejecuta la expresion final si existiera
- Los bloques son siempre expresiones de valor y evaluan la ultima expresion en el contexto de expresion de valor, se usa para forzar el moviemiento de un valor.
- El tipo de el bloque es el tipo de expresion final o `()` si la expresion final es omitida.
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
- Los names declarados pueden denotar nuevas variables o nuevos items
#### Declaraciones de item
``` rust
#![allow(unused_variables)]
fn main() {
    fn outer() {
      let outer_var = true;
      fn inner() { /* outer_var, aqui no esta dentro del alcance (scope) */ }
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
``` rust
#![allow(unused_variables)]
fn main() {
    let mut v = vec![1, 2, 3];
    v.pop();          // Ignora los elementos retornados por pop
    if v.is_empty() {
        v.push(5);
    } else {
        v.remove(0);
    }                 // El punto y coma puede ser omitido.
    [1];              // Instruccion de expresion separada, no es una expresion de indexacion
}
```
``` rust
#![allow(unused_variables)]
fn main() {
    if true {
      1
    } else {
      2
    };
}
```
- Es aquella que evalua una expresion e ignora su resultado, por lo tanto, el proposito es desencadenar los efectos de evaluar su expresion
- Una expresion que consiste en un bloque o una expresion de flujo de control, si se usa en un contexto donde se permite una instruccion, puede omitirse el punto y coma final.
- Esto puede generar ambiguedad entre que se analice como una instruccion independiente y parte de otra expresion. En este caso es analizado como una instrucción


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
- El significado de cada tipo de expresion dicta varias cosas: 
  - Si evaluar o no las subexpresiones al evaluar la expresion
  - El orden en el que evalua las subexpresiones
  - Como combinar los valores de las subexpresiones para obtener el valor de la expresion
- Los bloques, las instrucciones, las expresiones pueden anidarse recursivamente entre si a una profundidad arbitraria
### Precedencia de expresion
| Operador / Expresion                                    | Asociatividad          |
| ------------------------------------------------------- | ---------------------- |
| Paths                                                   |                        |
| Llamadas al método                                      |                        |
| Expresiones de field                                    | de izquierda a derecha |
| Llamadas a funcion, indexacion de array                 |                        |
| `?`                                                     |                        |
| Unario `-` `*` `!` `&` `&mut`                           |                        |
| `as`                                                    | de izquierda a derecha |
| `*` `/` `%`                                             | de izquierda a derecha |
| `+` `-`                                                 | de izquierda a derecha |
| `<<` `>>`                                               | de izquierda a derecha |
| `&`                                                     | de izquierda a derecha |
| `^`                                                     | de izquierda a derecha |
| `|`                                                     | de izquierda a derecha |
| `==` `!=` `<` `>` `<=` `>=`                             | requiere parentesis    |
| `&&`                                                    | de izquierda a derecha |
| `||`                                                    | de izquierda a derecha |
| `..` `..=`                                              | requiere parentesis    |
| `=` `+=` `-=` `*=` `/=` `%=` `&=` `|=` `^=` `<<=` `>>=` | de derecha a izquierda |
| `return` `break` closures                               |                        |
- Se ordena pasando del fuerte al debil
### Expresiones de lugar y expresiones de valor
- Las expresiones se dividen en dos categorias lugar y valor
- La evaluacion de una expresion depende de su propia categoria como el contexto (lugar o valor) en el que se produce
- **Expresion de lugar**: Representa una ubicacion de memoria, historicamente llamado lvalues
  - Son paths que referencia a:
    - Variables locales: expresion
    - Variables estaticas: EXPRESION
    - Desreferencias: \*expresion
    - Indexacion de array: expresion[expresion]
    - Expresiones a field: expresion.identificador
    - Expresiones de lugar entre parentesis
- **Expresion de valor**: Representa un valor actual, historicamente llamado rvalues
#### Tipos movidos y copiados
- Cuando una expresion de lugar se evalua en un contexto de expresion de valor, o esta vinculada por valor en un patron, denota el valor contenido en esa ubicacion de memoria.
#### Mutabilidad
- Para que una expresion de lugar pueda ser asignada a: borrowed mutable, borrowed mutable implicito o unido a un patron que contiene `ref mut`. Llamamos a expresiones de lugar mutables
#### Lifetimes temporales
- Cuando se utiliza una expresion de valor en contextos de expresion de lugar, se crea una ubicacion de memoria sin nombre temporal inicializada en ese valor y la expresion se evalua en esa ubicacion
#### Prestamos implicitos
- Ciertas expresiones tratarán una expresion como una expresion de lugar al tomarla prestada implicitamente.
- Llamada al metodo, expresiones a field, expresiones de llamada, expresiones de indexacion de matrix, operador de referencia, comparacion, asignacion compuesta