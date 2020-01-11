fn main() {
    //---------- declarar una variable ----------
    let persona_string = "Rodrigo Mendoza";  // variable tipo string
    let edad_integer = 32;
    let calificacion_float = 14.3;           // variable tipo float
    let es_mayor_edad = true;           // variable tipo boolean
    let icono_char = 'Ⓐ';                   // variable tipo caracter unicode
    println!("nombre: {}",persona_string);
    println!("edad: {}",edad_integer);
    println!("calificación: {}",calificacion_float);
    println!("mayor de edad: {}",es_mayor_edad);
    println!("icono: {}",icono_char);
    //---------- inmutablidad de variables ----------
    let x = 5;
    println!("El valor de x es: {}", x);
    //x = 6; //--> ERROR, la variable inmutable no puede ser asignada 2 veces
    println!("El valor de x es: {}", x);
    //---------- mutablidad de variables ----------
    let mut x = 5;
    println!("El valor de x es: {}", x);
    x = 6; //--> ERROR, la variable inmutable no puede ser asignada 2 veces
    println!("El valor de x es: {}", x);
    //---------- shadowing de variables ----------
    let saludo=8014_4_7005;
    let saludo="Hola a todos";
    println!("saludo: {}",saludo);
    
    
    //---------- declarar una constante ----------
    const LIMITE_USUARIOS:i32 =100;
    const PI:f32=3.14;
    
    println!("limite de usuarios: {}",LIMITE_USUARIOS);
    println!("valor de PI: {}",PI);
}
