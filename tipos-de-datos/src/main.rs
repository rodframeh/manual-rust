fn main() {
//---------- sintaxis ---------- 
  let salario=3_500.00;
  println!("su salario es {}",salario);

  let salario:f64=3_500.00;
  println!("su salario es {}",salario);
//---------- entero ----------
  let por_defecto=50;
  let edad:u8=27;
  let resta:i16=9_731-7_834;
  let marca:isize=20;
  let cantidad: usize=456;
  
  println!("por defecto: {}",por_defecto);
  println!("edad: {}",edad);
  println!("resta: {}",resta);
  println!("marca: {}",marca);
  println!("cantidad: {}",cantidad);
//---------- overflow ----------
  //let a:u8=256;   // 256 desborda en 1
  //let b:i8=128;   // 129 desborda en 2
  //println!("numeros: {},{}",a,b);
  
//---------- flotante ----------
  let resultado=10.01;
  let interes: f32=8.35;
  let costo: f64=12_000.785;
  
  println!("resultado: {}",resultado);
  println!("interes: {}",interes);
  println!("costo: {}",costo);
  
//---------- booleano ----------
  let esta_jugando: bool=false;
  println!("¿esta jugando?: {}", esta_jugando);
  
//---------- caracter ----------
  let caracter_especial='@';
  let caracter_alfanumerico: char='7';
  let caracter_emoji: char='😁';
  
  println!("caracter especial: {}", caracter_especial);
  println!("caracter alfanumérico: {}", caracter_alfanumerico);
  println!("caracter emoji: {}", caracter_emoji);
  
//---------- tupla ----------
  let tupla= (true, 'X','Y','Z', 56_854); // Se declara una tupla sin especificar los tipos de datos
  println!("valores de la tupla {:?}", tupla);
  
  let tupla:(f64,char,bool)=(789_345.789,'😁',true); // Se declara una tupla especificando los tipos
  println!("valores de la tupla {:?}",tupla);
  println!("primer valor de la tupla {:?}",tupla.0);
  println!("segundo valor de la tupla {:?}",tupla.1);
  println!("tercer valor de la tupla {:?}",tupla.2);

//---------- desestructuración de la tupla ----------
  let tupla=(true,'X','Y','Z', 56_854);
  let (a,b,c,d,e)=tupla;
  
  println!("primer valor de la tupla {:?}",a);
  println!("segundo valor de la tupla {:?}",b);
  println!("tercer valor de la tupla {:?}",c);
  println!("cuarto valor de la tupla {:?}",d);
  println!("quinto valor de la tupla {:?}",e);

//---------- array ----------
  let array=['A','B','C']; // Se declara e inicializa un array sin especificar sus tipos de datos
  println!("valores del array: {:?}", array);
  let array:[i32;5]=[1,2,3,4,5]; // Se declara e inicializa un array especificando sus tipos de datos
  println!("valores del array: {:?}", array);
  let array=[-2.5;10]; //Se declara e inicializa un array sin especificar sus tipos de datos y con valores predeterminados de -2.5
  println!("valores del array: {:?}", array);
  println!("tamaño del array: {:?}", array.len());
  let array:[f64;10]=[-2.5;10]; //Se declara e inicializa un array especificando sus tipos de datos y con valores predeterminados de -2.5
    println!("valores del array: {:?}", array);
  println!("tamaño del array: {:?}", array.len());
//---------- valores del array ----------
  let mut meses=["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"];
  println!("meses del año: {:?}", meses);
  println!("mes 6: {:?}",meses[5]);
  meses[5]="Marzo";// Se actualiza el valor con el indice 5 del array
  println!("mes 6 (modificado): {:?}",meses[5]);// Se consulta el valor modificado
}







