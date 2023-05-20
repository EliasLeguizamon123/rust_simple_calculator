use std::io;

pub fn calculator() {
  println!("Ingrese dos numeros separados por espacio ");
  
  let mut user_input = String::new();
  
  io::stdin().read_line(&mut user_input).expect("Error al leer la entrada");
  
  let mut numbers = user_input.split_whitespace();
  let a: i32 = numbers.next().unwrap().parse().unwrap();
  let b: i32 = numbers.next().unwrap().parse().unwrap();
  
  println!("Ingrese la operacion a realizar (+, -, *, /)");
  
  let mut user_operation = String::new();

  io::stdin().read_line(&mut user_operation).expect("Error al leer la entrada");

  let operation: &str = user_operation.trim();

  let result = match operation {
    "+" => add(a, b),
    "-" => sub(a, b),
    "*" => mul(a, b),
    "/" => div(a, b),
    _ => panic!("La operacion que se intenta realizar no es soportada  ?")
  };

  println!("El resultado de la operacion es: {}", result);
}

fn add (a: i32, b: i32) -> i32 {
  return a + b;
}

fn sub (a: i32, b: i32) -> i32 {
  return a - b;
}

fn mul (a: i32, b: i32) -> i32 {
  return a * b;
}

fn div (a: i32, b: i32) -> i32 {
  return a / b;
}

fn main (){
  calculator();
}