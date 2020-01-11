[TOC]
# Tipos de datos
- Cada valor en el lenguaje Rust es de cierto tipo de datos, por lo tanto las variables y constantes  están asociadas con un tipo de datos.
- El tipo de datos determina:
	- El tamaño y diseño de la memoria de la variable.
	- El rango de valores que se puede almacenar dentro de esa memoria.
	- El conjunto de operaciones que se puede realizar en la variable.
- El **sistema de tipos** representa los diferentes tipos de valores admitidos por el lenguaje y verifica la validez de los valores suministrados antes que el programador los manipule.
## Sintaxis
- Con tipo dato no especificado
``` rust
  let salario=3_500.00;
  println!("su salario es {}",salario);
```
-  Con tipo dato especificado
``` rust
  let salario:f64=3_500.00;
  println!("su salario es {}",salario);
```
## Clasificación
- Según la cantidad de valores almacenados y el valor almacenado
### Escalares
- Permite que la variable o constante con ese tipo de datos almacene **un solo valor**
#### Entero
``` rust
  let por_defecto=50;
  let edad:u8=26;
  let suma:i16=5_731-7_834;
  let marca:isize=20;
  let cantidad: usize=456;

  println!("por defecto: {}",por_defecto);
  println!("edad: {}",edad);
  println!("suma: {}",suma);
  println!("marca: {}",marca);
  println!("cantidad: {}",cantidad);
```
- Los enteros no tiene componente fraccionario.
- El lenguaje rust utilizar el tipo de tamaño i32 por defecto, funciona rápido en las máquinas con arquitectura de x86 y x64 bits.
- **Enteros con signo (i)**
	- Se almacenan utilizando la representación de complemento a dos.
	- **Limite** `-[2^(n - 1)] a [2^(n - 1)]-1`, donde `n` es el número de bits que la variante use.
- **Enteros sin signo (u)**
	- Permite establecer una mayor cantidad de elementos que con signo.
	- **Limite** `0 a (2^n)-1`, donde `n` es el número de bits que la variante use.
- Clasificación de enteros
  | **Longitud** | **Con signo** | **Sin signo** |
  | :----------: | :-----------: | :-----------: |
  |8 bits    |      i8       |      u8       |
  |16 bits    |      i16      |      u16      |
  |32 bits    |      i32      |      u32      |
  |64 bits    |      i64      |      u64      |
  |128 bits   |     i128      |     u128      |
  |arch     |     isize     |     usize     |
- Los tipos de tamaño **isize** y **usize** dependen del tipo de arquitectura de la máquina donde se ejecute y son utilizados para indexar algun tipo de colección.
``` rust
  let a:u8=256;   // 256 desborda en 1
  let b:i8=128;   // 129 desborda en 2
  println!("numeros: {},{}",a,b);
```
- **Overflow**, el desbordamiento ocurre cuando se sobrepasa la capacidad del entero.
#### Flotante
#### Booleano
#### Caracter
### Compuestos
- Permite que la variable o constante con ese tipo de datos almacene **más de un solo valor**
#### Tupla
#### Array
### Complejos
- Falta...