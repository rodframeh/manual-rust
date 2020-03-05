[TOC]
# Propiedad y prestamo
- La memoria se puede asignar en el stack o en el heap, cuando se ejecuta el programa los tipos de memoria crecen hacia arriba o hacia abajo
## Enfoques para la administracion de memoria
- En C, C++ el programador debe explicitamente asignar (`malloc`) memoria para utlizarla y liberar (`free`) memoria para no desperdiciar. 
  - **Ventaja**: No afecta el rendimiento de la aplicacion. 
  - **Desventaja**: El programador requiere gestionar la memoria.
- En Java, C#, Go existe un recolector de basura que se busca constantemente la memoria que ya no se usa para liberarla. 
  - **Ventaja**: El programador no requiere gestionar la memoria
  - **Desventaja**: Reduce el rendimiento de la aplicacion.
- En Rust en tiempo de compilacion, el compilador se agrega codigo de asignacion y liberacion de memoria a traves del sistema de propiedad con un conjunto de reglas. La funcion `drop()`se invoca cuando la variable queda fuera del alcance, se parece al Resource Acquisition Is Initialization (RAII) de C++. 
  - **Ventaja**: El programador no requiere gestionar la memoria y no afecta el rendimiento de la aplicacion. 
  - **Desventaja**: El tiempo de compilacion es mayor.
## Stack
- Es una estructura de datos que permite almacenar valores en el orden en que los obtiene y elimina los valores en orden inverso (*último en entrar, primero en salir*).
- Almacena valores con tamaño fijo que se conoce en tiempo de compilación.
- Almacena valores con tipo de datos escalares, compuestos y punteros a los datos en el heap.
- Tiene una rápida velocidad de acceso, debido a la posición de acceso y colocación de los datos, siempre es la parte superior del stack.
  - No requiere buscar un lugar para colocar datos
  - No requiere buscar un lugar para obtener datos
- Proceso de funcionamiento del stack en una función
  - Se llama a la función
  - Se agregan valores al stack
    - Los argumentos
    - Las variables locales de la función
  - La función termina de ejecutarse
    - Se remueven los valores del stack.
## Heap
- Es la memoria de almacenamiento dinámico