[TOC]
# Funciones
- Es el componente básico de un código legible, mantenible y re-utilizable.
- Permiten organizar y dividir el código en partes más pequeñas, por lo que es más fácil de entender.
- Es un bloque lógico de codigo asociado a un nombre y un conjunto de parametros, el cual permite realizar una tarea específica
## Partes de una funcion
- **Header | Signature | Firma**: Esta compuesto por el nombre de la función, los parametros y el tipo de dato de retorno.
- **Body | Cuerpo**: Contiene un conjunto de instrucciones y expresiones que la función debe ejecutar.
## Declaración de una funcion
``` rust
fn saludar(){}
```
- Una declaración de función le indica al compilador sobre el nombre, el tipo de dato de retorno y los parámetros de una función (signature). 
- Permite especificar el **qué** tarea específica se realizara.
- Se declaran con la palabra clave `fn`. 
- Se puede declarar:
  - Un conjunto de variables de entrada como parametros, a traves de los cuales el llamante pasa argumentos a la funcion
  - El tipo de salida del valor que la funcion retornara al llamador al finalizar
## Definición de una función
``` rust
fn saludar(){
  println!("Hola a todos!!");
}
```
- Permite especificar el **qué y el cómo** se realizará una tarea específica.
- Una definición de función proporciona el cuerpo real de la función, le indica al compilador sobre el nombre, el tipo de dato de retorno, los parámetros y el conjunto de instrucciones y expresiones de una función.
- No importa el lugar donde se defina la funcion, solo que este definida en alguna parte.
- Se puede llamar a cualquier funcion que se haya definido, ingresando su nombre seguido de un conjunto de parentesis.
## La función `main()`
``` rust
fn main(){
    // Codigo rust
}
```
- `main()` es una función predefinida que actúa como punto de entrada al programa, es un orquestador y no tiene parámetros.
## Invocación de una función
``` rust
fn main(){
  saludar();
}
```
- Se llama o invoca una función para ejecutarla.
- A este proceso se denomina **function invocation** (invocación de función).
- La función que invoca a otra función se le llama **caller function** (función de llamada).
## Función de retorno
``` rust
fn pi()->f64{
  22.0/7.0
}
fn factorial(numero: u64)->u64{
  let mut fac=1;
  for i in 1..(numero+1){
    fac*=i;
  }
  fac
}
// número euler a 18 decimas
fn e()->f64{
  let mut e=0.0;
  for i in 0..18{
    e=e+1.0/factorial(i) as f64
  }
  return e;
}
fn main() {
  println!("Valor de PI: {}",pi());
  println!("Valor de e: {}", e());
}
```
- Las funciones permiten devolver el valor junto con el control.
- La mayoría de las funciones retorna la última expresión implícitamente o puede utilizar la palabra clave **return**.
- Es posible retornar más de un valor a través de tuplas.
## Función parametrizada
- **Parámetros**
  - Permiten pasar valores a funciones.
  - Requiere especificar el tipo de datos.
  - Forman parte del signature de la función.
- **Argumentos**
  - Son los valores de los parámetros.
  - Se pasan a una función durante su invocación.
  - El número de argumentos pasados a un función debe coincidir el número de parámetros definidos.
### Pase por valor
``` rust
fn mutar_copia_numero(mut numero:i32){
  numero=numero*0;
  println!("El valor de la copia del numero es {}", numero);
}
fn main() {
  let numero=5;
  mutar_copia_numero(numero);
  println!("El valor original del número es {}",numero);
}
```
- Cuando se invoca a una función, cada argumento se almacena en otra direccion de memoria virtual, es decir **se copian los valores**, por lo tanto, los cambios realizados a la copia del argumento dentro del método invocado no tienen efecto en el argumento original pasado.
### Pase por referencia
``` rust
fn mutar_original_numero(numero: &mut i32){
  *numero=*numero*0;
}
fn main() {
  let mut numero=5;
  mutar_original_numero(&mut numero);
  println!("El valor original del número es {}",numero);
}
```
- La **referencia** es la direccion de memoria virtual del valor de la variable, se obtiene al prefijar con un `&` el identificador de la variable.
- La **desreferencia** es la obtencion del valor almacenado en la direccion de memoria virtual, se obtiene al prefijar con un `*` el identificador de la variable referencia .
- Cuando se invoca a una función, cada argumento se almacena en otra direccion de memoria virtual, es decir **se copian los valores**, pero en este caso se crea una copia de la referencia (direccion de memoria virtual). Si se desea cambiar el valor al que apunta el argumento (la referencia), solo se cambia el valor en la direccion que apunta la referencia (se requiere desreferenciar), recordar que este cambio se aplica al valor y no a la referencia, entonces el argumento original (referencia) apuntara al valor cambiado.
## Avanzado
``` 
Function :
FunctionQualifiers fn IDENTIFIER Generics?
      ( FunctionParameters? )
      FunctionReturnType? WhereClause?
      BlockExpression

FunctionQualifiers :
AsyncConstQualifiers? unsafe? (extern Abi?)?

AsyncConstQualifiers :
async | const

Abi :
STRING_LITERAL | RAW_STRING_LITERAL

FunctionParameters :
FunctionParam (, FunctionParam)* ,?

FunctionParam :
OuterAttribute* Pattern : Type

FunctionReturnType :
-> Type
```
- Cuando se hace *referencia* a una funcion, produce un valor *first-class* del *tipo de item de funcion* de tamaño cero (zero-sized), que cuando se llama se evalua como una llamada directa a la funcion. Ese tipo identifica explicitamente la funcion.