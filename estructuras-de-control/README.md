[TOC]
# Estructuras de control
Las expresiones if y bucles permiten controlar el flujo de ejecución del código.
Toda variable declara dentro del ambito `{...}` de la estructura de control sólo es accesible dentro del ambito de esta.
## Toma de decisiones
- Se especifican condiciones a ser evaluadas, junto con un conjunto de declaraciones que se ejecutarán si la condición es verdadera o si es falsa.
- Permite bifurcar el código a partir de condiciones.
- **La condición debe ser de tipo booleano, no evalua números.**
### if
``` rust
    let numero=7;
    if numero>0{
	    println!("El número {} es positivo",numero);
    }
```
- La declaración `if` consiste en la evaluación de una expresión booleana, si la expresión es verdadera se ejecuta un conjunto de declaraciones añadidos a la condición verdadera, si la expresión es falsa se salta el conjunto de declaraciones añadidos a la condición verdadera y ejecuta el código después del final del bloque `if`.
### if ... else
``` rust
    let numero=6;
    if numero%2==0{
        println!("El número {} es par",numero);
    }else{
        println!("El número {} es impar",numero);
    }
```
- Una declaración `if` puede o no ser seguida por una declaración `else` (es opcional)
- La expresion `else` ejecuta un bloque de código alternativo cuando la expresión booleana es falsa.
### else ... if y if anidado
``` rust
	let numero=2;
    if numero>0{
        println!("El número {} es positivo", numero);
        if numero%2==0 {
            println!("El número {} es par", numero);
        }else{
            println!("El número {} es impar",numero);
        }
    }else if numero<0{
        println!("El número {} es negativo", numero);
        if numero%2==0 {
            println!("El número {} es par", numero);
        }else{
            println!("El número {} es impar",numero);
        }
    }else{
        println!("El número es cero")
    }
```
- Puedes usar la declaración `if` o `else if` dentro de otra declaración `if` o `else if`.
- Puntos a tener en cuenta:
	- Un `if` puede tener varios `else..if` y deben venir antes que el `else` si existiera.
	- Se evalua secuencialmente cada expresión `if`, si alguna expresion tiene éxito, todos los demás son descartados, es decir ninguno de los otros sera probado.
### let ... if
``` rust
	let numero=42;
    let resultado=if numero%7==0 {
        "es multipo de 7"
    }else{
        "no es multiplo de 7"
    };
    println!("El numero {} {}",numero,resultado);
```
- La expresión `if` es una expresión y podemos utilizarla como valor en una declaración `let`, para esto tenemos que hacer que el bloque de codigo de condicion verdadera o falsa termine en una expresión del mismo tipo de datos (Rust verifica en tiempo de compilación si es valido o no).
- La declaración `let` con la expresión `if` deben finalizar en `;`.
### match
``` rust
	let codigo_aeropuerto="TPP";
    let ciudad=match codigo_aeropuerto {
        "AQP"=>{
            println!("Se encontró una coincidencia para {}",codigo_aeropuerto);
            "Arequipa"
        },
        "CUZ"=>{
            println!("Se encontró una coincidencia para {}",codigo_aeropuerto);
            "Cusco"
        },
        "TPP"=>{
            println!("Se encontró una coincidencia para {}",codigo_aeropuerto);
            "Tarapoto"
        },
        "TBP"=>{
            println!("Se encontró una coincidencia para {}",codigo_aeropuerto);
            "Tumbes"
        },
        _=> "Desconocido"
    };
    if ciudad == "Desconocido"{
    	println!("No se encontró ninguna coincidencia para {}",codigo_aeropuerto);
    }else{
    	println!("La ciudad es {}",ciudad);
    }
```
- Es similar a `switch` en el lenguaje C.
- La expresión `match` permite:
	- Comparar una variable con una lista de valores (arms).
	- Si el valor de la variable coincide con algun arm (brazo), se ejecuta un bloque o expresion de codigo añadido a la condición.
	- Ejecutar secuencialmente cada arm, una vez que coincide y ejecuta un arm, no compara los siguientes arms.
- La expresión `match` es una expresión y podemos utilizarla como valor en una declaración `let`, para esto tenemos que cada bloque de codigo añadido a una condicion termine en una expresión del mismo tipo de datos (Rust verifica en tiempo de compilación si es valido o no).
- La declaración `let` con la expresión `match` deben finalizar en `;`.
- El caso predeterminado se establece con guion bajo `_`, el cual coincidirá con todos los casos posibles que no se hayan especificado antes.
## Bucles
- Son utiles cuando se requiere que un bloque de código o un conjunto de declaraciones se ejecute repetidamente a partir de condiciones establecidas.
- Un bucle cuyo número de iteraciones está determinado, se le denomina bucle definido.
#### for
``` rust
	for numero in 1..10{
	    println!("{}", numero);
    }
    
    for numero in (1..10).rev(){
        println!("{}", numero);
    }
```
- Se recomienda su uso para el manejo de indices
- Ofrece mayor seguridad que el buble while
- Es el bucle más utilzando en el lenguaje Rust
- Permite iterar entre números con range `..` y se puede revetir la iteración conel método `rev()`