[TOC]
# Funciones
- Es un conjunto de declaraciones para realizar una tarea específica.
- Es el componente básico de un código legible, mantenible y reutilizable.
- Permiten organizar el programa en bloques lógicos de código.
- Permiten dividir el código en partes más pequeñas, por lo que es más fácil de entender.
## Partes
- Identificador
- Parametros
- Tipo de retorno
- Bloque


- **Header | Firma**: Contiene el nombre de la función, los parametros y el tipo de dato de retorno
- **Body | Cuerpo**: Contiene un conjunto de declaraciones y expresiones que la función debe ejecutar.
## Definiendo una función
``` rust
fn saludar(){
    println!("Hola a todos!!");
}
```
- Consiste 
- Las funciones se definen usando la palabra clave **fn**.
- Permite especificar el **qué** y el **cómo** se realizará una tarea específica
## Función `main()`
- La palabra clave `fn` se usa para definir una función. 
- `main()` es una función predefinida que actúa como punto de entrada al programa, es un orquestador y no tiene parámetros
``` rust
fn main(){
    // Codigo rust
}
```