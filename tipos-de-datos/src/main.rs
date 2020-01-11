fn main() {
//---------- sintaxis ---------- 
  let salario=3_500.00;
  println!("su salario es {}",salario);

  let salario:f64=3_500.00;
  println!("su salario es {}",salario);
//---------- entero ----------
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
//---------- overflow ----------
  //let a:u8=256;   // 256 desborda en 1
  //let b:i8=128;   // 129 desborda en 2
  //println!("numeros: {},{}",a,b);
}
