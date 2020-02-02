fn main() {
    //operaciones aritméticas
    let a=27;
    let b=7;
    // sumar
    println!("suma: {}", a+b );
    // restar
    println!("resta: {}", a-b );
    // multiplicar
    println!("multiplicacion: {}", a*b );
    // dividir
    println!("division: {}", a/b );
    // modulo
    println!("modulo:{}",a%b);
    // operaciones relacionales
    let a=6;
    let b=4;
    // mayor
    println!("mayor: {}", a>b );
    // menor
    println!("menor: {}", a<b );
    // mayor o igual que
    println!("mayor o igual que: {}", a>=b );
    // menor o igual que
    println!("menor o igual que: {}", a<=b );
    // igual que
    println!("igual que: {}", a==b );
    // diferente
    println!("diferente: {}", a!=b );
    // operaciones logicas
    let a=true;
    let b=false;
    // and
    println!("and: {}", a && b);
    // or
    println!("or: {}", a || b);
    // negacion
    println!("negacion: {}",!b);
    // operaciones bit a bit
    let a=4;
    let b=2;
    // AND en bits &
    println!("and en bits: {}",a&b);
    // OR en bits |
    println!("or en bits: {}",a|b);
    // XOR en bits ^
    println!("xor en bits: {}",a^b);
    // Invertir bits !	
    println!("invertir bits: {}",!b);
    // Desplazar bits a la izquierda <<
    println!("desplazar bits a la izquierda: {}",a<<1);
    // Desplazar bits a la derecha >>
    println!("desplazar bits a la derecha: {}",a>>1);
}
