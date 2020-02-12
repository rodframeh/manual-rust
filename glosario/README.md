[TOC]
# Glosario
## Compilacion
### Abtract syntax tree | Arbol de sintaxis abstracta | AST
- Es una representación intermedia de la estructura del programa cuando el compilador lo esta compilando.

## Diseño de tipo
## Alignment -- FALTA
- El alignment (alineación) de un valor especifica en que direcciones son validas para almacenar un valor.

## Argumentos
## Arity
- Se refiere al número de argumentos que toma una función u operador
- Ejemplos: la funcion f(2, 3) tiene 2 arity, el operador ! tiene 1 arity 

## Tipos de datos
### Array | Fixed-size array | Inline array
- Es un valor que describe una coleccion de elementos, cada elemento seleccionado por un indice que el programa puede calcular en tiempo de ejecucion.
- Ocupa una región contigua de memoria

## Associated item -- REVISAR
- Es un item que es asociado con otro item
- Los items asociados se **definen** en implementations y se **declaran** en traits
- Solo se puede funciones, constantes y alias de tipo (type aliases)

## Tipo
### Blanket implementation -- FALTA
- Cualquier implementación donde un tipo aparece uncovered

## Tipo | Trait
### Bound | Limite
- Los bounds son restricciones en un tipo o trait. 
- Ejemplo: colocar un limite en el argumento que toma una función

## Funciones
### Combinator --REVISAR
- Son funciones de orden superior que aplican solo funciones y combinators definidos anteriormente para proporcionar un resultado de sus argumentos
- Se pueden utilizar para gestionar el flujo de control de forma modular

## Compilacion
### Dispatch | Despachador --REVISAR
- Es el mecanismo para determinar que version especifica de codigo se ejecuta realmente cuando se trata de poliformismo
- Dos formas principales de dispatch, son el estatico (Rust favorece) y el dinamico (a traves de un mecanismo llamado 'objetos trait')

## Compilacion
### Dynamically sized type | DST
- Es un tipo sin un tamaño (size) o alineacion (alignment) estaticamente conocido

## Variables
### Expression
- Es una combinacion de valores, constantes, variables, operadores y funciones que se evaluan en un solo valor, con o sin efectos secundarios.
- Ejemplo: 2+(3+4) es una expresion que devuelve el valor 14.

## Free item --REVISAR
- Un item que no es miembro de una implementacion
- Ejemplo: una funcion libre o una constante libre

## Traits
### Fundamental traits --REVISAR
- Es un trait donde agregar un impl para un tipo existente es un cambio radical
- Ejemplos: traits Fn y Sized

## Tipos 
### Fundamental type constructors
- Es un tipo en el que implementar una implementacion general sobre el es un cambio radical
- Los tipos fundamentales no pueden abarcar(cover) otros tipos
- Ejemplos: &, &mut, Box y Pin

## Tipos
### Inhabited | habitado
- Un tipo esta inhabited si tiene constructores y, por lo tanto puede ser instanciado.
- Un tipo inhabited no esta "vacio" debido a que puede haber valores del tipo.

## Tipos
## Inherent implementation --REVISAR
- Una implementacion que se aplica a un tipo nominal, no a un par de traits

## Tipos
## Inherent method --REVISAR
- Un metodo definido en una implementacion inherente, no en una implementacion de trait

## Variables
## Initialized
- Una variable se inicializa si se le ha asignado un valor y desde entonces no se ha movido
- Se asume que todas las demas ubicaciones de memoria no estan inicializadas
- Solo unsafe Rust puede crear tal memoria sin inicializarla

## Traits
## Local trait
- Es un trait que se definio en el crate actual.
- Una definicion de trait es local o no independiente de los argumentos de tipo aplicados.

## Tipo
### Local type
- Un `struct`, `enum` o `union` que se definio en el crate actual.
- Esto no se ve afectado por los argumentos de tipo aplicado.
- Los alias de tipo no afectan a la localidad

## Tipo
### Nominal types
- Tipos que pueden ser referenciados por el path directamente
- Especificamente: enums, structs, unions y trait objects

## Trait
### Object safe traits
- Trait que puede usarse como objetos traits
- Solo los traits que siguen reglas especificas son seguros para los objetos

## Trait
### Prelude
- Es una pequeña coleccion de items, principalmente traits, que se importan en cada modulo de cada crate.
- Los traits del preludio son generalizados (pervasive).

## Estructuras de control
###  Scrutinee
- Es la expresion que coincide con expresiones de `match` y construcciones de coincidencia de patros similares
- Ejemplo: match x { A=>1, B=>2 }, la expresion `x` es el scrutinee

## Variables
###  Size
- Es la cantidad de memoria que se debe asignar para almacenar un valor
- Es el desplazamiento en bytes entre elementos sucesivos en un array con ese tipo de item
- Es un multiplo de la alineacion (alignment), incluido cero. Puede cambiar dependiendo de la version del compilador y la plataforma de destino

## Tipo de datos
###  Slice
- Es una vista de tamaño dinamico en una secuencia contigua, escrita como `[T]`, donde `T` representa el tipo de elemento.
- A menudo se ve en sus formas prestadas (borrowed), ya sean mutables `&mut [T]` o compartidas (shared) `&[T]`.

## Variable
###  Statement
- Es el elemento independiente más pequeño de un lenguaje de programacion que ordena a una computadora que realice una acción.

## String
### String literal
- Es una cadena almacenada directamente en el binario final, por lo que sera valida durante la  `'static` duracion.
- Su tipo es `'static` slice de string prestado (borrowed) de duración, `&'static str`

## String
### String slice
- Es el tipo de string más primitvo en rust, escrito como `str`.
- A menudo se ve en sus formas prestadas (borrowed), ya sean mutables o compartidas (shared)
- El tipo de string slice compartida (shared) es `&str` y mutable es `&mut str`

## Trait
- Es un elemento de lenguaje que se utiliza para describir las funcionalidades que debe proporcionar un tipo
- Permite que un tipo haga ciertas promesas sobre su comportamiento
- Las funciones genericas y los strucs genericos pueden usar traits para restringir (constrain) o limitar (bound) los tipos que aceptan.

## Uncoreved type
- Un tipo que no aparece como argumento para otro tipo
- Esto solo es relevante para los argumentos de tipo
- Ejemplo: `T` esta descubierto (uncovered), pero el `T` en `Vec<T>` esta cubierto (covered)

## Undefined behavior
- Comportamiento en tiempo de compilación o tiempo de ejecución que no se especifica
- Esto puede ocasionar, pero no se limita a: process termination or corruption; improper, incorrect, or unintended computation; or platform-specific results

## Tipo
###  Uninhabited | Deshabitado
- Es un tipo que no tiene constructores y por lo tanto nunca puede ser instanciado
- Esta "vacio" debido a que no hay valores del tipo
- Ejemplo: tipo `!` o una enum sin variantes