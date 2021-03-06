fn main() {
  //---------- invocando una funcion
  saludar();
  println!("Valor de PI: {}",pi());
  println!("Valor de e: {}", e());
  
  let mut numero=5;
  mutar_copia_numero(numero);
  println!("El valor original del número es {}",numero);
  
  mutar_original_numero(&mut numero);
  println!("El valor original del número es {}",numero);
}
//---------- definiendo una funcion
fn saludar(){
    println!("Hola a todos!!");
}
//---------- funciones de retorno
fn pi()->f64{
  22.0/7.0
}
fn factorial(numero: u64)->u64{
  let mut fac=1;
  for i in 1..(numero+1){
    fac*=i;
  }
  fac
}
// número euler a 18 decimas
fn e()->f64{
  let mut e=0.0;
  for i in 0..18{
    e=e+1.0/factorial(i) as f64
  }
  return e;
}
// funcion parametrizada > pase por valor
fn mutar_copia_numero(mut numero:i32){
  numero=numero*0;
  println!("El valor de la copia del numero es {}", numero);
}
// funcion parametrizada > pase por referencia
fn mutar_original_numero(numero: &mut i32){
  *numero=*numero*0;
}

