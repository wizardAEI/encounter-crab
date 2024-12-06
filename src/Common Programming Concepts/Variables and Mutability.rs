fn main() {
     // by default, variable is immutable
     let a = 1;
     println!("variable a is: {a}");
     // mutable variable
     let mut b = 2;
     println!("mutable variable b is: {b}");
     b = 3;
     println!("change the variable b and the value is {b}");
     // const must annotate the type, declare value a
     const C_VALUE: i32 = 1;
     println!("constant c: {C_VALUE}");
     // shadowing
     let a = "hello crab"; 
     println!("that the first a is shadowed by the second a: {a}");
     // scope
     {
         let a = format!("{a}!");
         println!("scope a: {a}");
     }
     println!("outside scope and a is: {a}")
}