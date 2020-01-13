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
}
