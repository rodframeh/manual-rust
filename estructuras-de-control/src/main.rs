fn main() {
    //---------- if
    let numero=7;
    if numero>0{
	    println!("El número {} es positivo",numero);
    }
    //---------- if ... else
    let numero=6;
    if numero%2==0{
        println!("El número {} es par",numero);
    }else{
        println!("El número {} es impar",numero);
    }
    //---------- else ... if y if anidado
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
    //---------- let ... if
    let numero=42;
    let resultado=if numero%7==0 {
        "es multipo de 7"
    }else{
        "no es multiplo de 7"
    };
    println!("El numero {} {}",numero,resultado);
    //---------- match
    let codigo_aeropuerto="X";
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
    //---------- for 
    let limite=7;
    let mut factorial=1;
    for contador in 1..limite{
        factorial*=contador;
	    println!("numero: {} factorial: {}",contador, factorial);
    }
    for contador in (1..limite).rev(){
        println!("numero: {} factorial: {}",contador, factorial);
        factorial/=contador;
    }
        
    //---------- loop
    let mut cantidad=0;
    println!("se registraron un total de {} entradas", loop{
        let mut entrada=String::new();
        std::io::stdin().read_line(&mut entrada).expect("fallo al leer la linea");
        if entrada.trim()=="exit" {
            break cantidad;
        }
        println!("valor recibido:{}",entrada);
        cantidad+=1;
    });
    //---------- while
    println!("cantidad de numeros a sumar:");
    let mut limite=String::new();
    std::io::stdin().read_line(&mut limite).expect("fallo al leer la linea");
    let limite=limite.trim().parse::<i32>().unwrap();
    let mut acumulador=0;
    let mut contador=0;
    while contador<limite{
        let mut entrada=String::new();
        std::io::stdin().read_line(&mut entrada).expect("fallo al leer la linea");
        let numero=entrada.trim().parse::<i32>().unwrap();
        acumulador+=numero;
        contador+=1;
    }
    println!("el resultado de la suma es: {}",acumulador);
    //---------- continue
    for numero in 1..10 {
        if numero%2 ==0{
            continue;
        }
        println!("el numero {} es impar",numero);
    }
}
