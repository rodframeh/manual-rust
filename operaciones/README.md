[TOC]
# Operaciones
- Define alguna función (operación) que se realizará en los datos.
- Las operaciones se evaluan a un solo valor, luego se enlaza a una variable
- Ejemplo:
	- `5 + 4`
	- Donde
		- `5` y `4` son operandos
		- `+` es el operador
## Aritmeticas
- Son utilizados para sumar, restar, multiplicar, dividir y obtener el residuo entre **dos operandos**
- Retornan un numero del mismo tipo de dato que los operandos
### Adición +
``` rust
let a=23;
let b= 7;
println!("suma: {}",a+b);
```
- Devuelve la suma de los operandos `a + b`.
### Sustracción -
``` rust
let a=23;
let b= 7;
println!("resta: {}",a-b);
```
- Devuelve la diferencia de los operandos`a - b`.
### Multiplicación *
``` rust
let a=23;
let b= 7;
println!("multiplicación: {}",a*b);
```
- Devuelve el producto de los operandos `a * b`.
### División /
``` rust
let a=23;
let b= 7;
println!("división: {}",a/b);
```
- Devuelve el cociente de la división`a / b`.
### Modulo %
``` rust
let a=23;
let b= 7;
println!("modulo: {}",a%b);
```
- Devuelve el resto de la división `a % b`
## Relacionales
- Son utilizados para comparar valores
- Retornar un valor booleano: **true y false**
### Mayor que >
``` rust
let a=6;
let b=4;
println!("mayor que: {}",a>b);
```
- Devuelve la veracidad de la premisa `a > b`
### Menor que <
``` rust
let a=6;
let b=4;
println!("menor que: {}",a<b);
```
- Devuelve la veracidad de la premisa `a < b`
### Mayor o igual que >=
``` rust
let a=6;
let b=4;
println!("mayor o igual que: {}",a>=b);
```
- Devuelve la veracidad de la premisa `a >= b`
### Menor o igual que <=
``` rust
let a=6;
let b=4;
println!("menor o igual que: {}",a<=b);
```
- Devuelve la veracidad de la premisa `a <= b`
### Igual que ==
``` rust
let a=6;
let b=4;
println!("igual: {}",a==b);
```
- Devuelve la veracidad de la premisa `a == b`
### Diferente !=
``` rust
let a=6;
let b=4;
println!("diferente: {}",a!=b);
```
- Devuelve la veracidad de la premisa `a != b`
## Logicas
- Utilizados para combinar más de una condición
- Retornar un valor booleano: **true o false**
### AND &&
``` rust
let a=true;
let b=false;
println!("and: {}", a && b);
```
- Devuelve verdadero si todas las expresiones retornan verdadero
### OR ||
``` rust
let a=true;
let b=false;
println!("or: {}", a && b);
```
- Devuelve verdadero si al menos una expresión retorna verdadero
### NO !
``` rust
let a=true;
let b=false;
println!("no: {}",!b);
```
- Devuelve la negación del valor booleano
## Bit a bit
### AND en bits &
### OR en bits |
### XOR en bits ^
### Invertir bits !	
### Desplazar bits a la izquierda <<
### Desplazar bits a la derecha >>