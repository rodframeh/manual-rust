[TOC]
# Sintaxis
## Estilo
- Por lo general se utilizar *snake_case* para construcciones de `nivel de valor` y *CamelCase* para construcciones de `nivel de tipo`.
- Puede variar dependiendo del ítem, esto está determinado en el RFC #430.

| **Elemento**             | **Convención**                                       |
| ------------------------ | ---------------------------------------------------- |
| Crates                   | snake_case (pero preferible una sola palabra)        |
| Módulos                  | snake_case                                           |
| Tipos                    | CamelCase                                            |
| Traits                   | CamelCase                                            |
| Variantes enum           | CamelCase                                            |
| Funciones                | snake_case                                           |
| Métodos                  | snake_case                                           |
| Constructores generales  | new o con_mas_detalles                               |
| Conversión constructores | desde_algun_otro_tipo                                |
| Variables locales        | snake_case                                           |
| Variables estáticas      | SCREAMING_SNAKE_CASE                                 |
| Constantes               | SCREAMING_SNAKE_CASE                                 |
| Tipo parámetros          | concise CamelCase, usualmente una letra mayuscula: T |
| Lifetimes                | short, lowercase: 'a                                 |
## Identado
- No se usa tab para identar, sino 4 espacios
``` rust
fn main(){
    println!("Hello, world");
1234
}
```