# Declaraciones y expresiones
## Declaraciones
- Las declaraciones terminan con `;`, las expresiones no, pero si se le añade se les convierten en declaraciones.
- Las declaraciones no retornan ningún valor.
``` rust
fn main(){
    let saludo; // se declara la variable saludo
    saludo=String::from("Hello, world"); // se asigna una cadena a la variable saludo, se convierte esta expresion en una declaracion para que pueda ejecutarse la siguiente linea
    println!("{}",saludo)// es una expresion y puede o no terminar en ";" ya que no existe ninguna linea siguiente a ejecutarse
}
```