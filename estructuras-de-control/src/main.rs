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
    for numero in 1..10{
	    println!("{}", numero);
    }
    for numero in (1..10).rev(){
        println!("{}", numero);
    }

}
