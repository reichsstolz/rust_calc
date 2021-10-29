
use std::collections::HashMap;
use std::io;

fn bin_plus(a: u32, b: u32) -> u32{
    a+b - a*b
}

fn bin_x(a: u32, b: u32) -> u32{
    a*b
}

fn not(a: u32) -> u32{
    if a==1{
        0
    }else{
        1
    }

}

fn bin(a: u32, num: u32)-> String{
    let mut bin = String::new();
    let mut  a = a;

    while a!=1 && a!=0{
        bin.insert(0, (a%2).to_string().chars().next().unwrap());
        a = a/2;
        //println!("OK {}!", bin);
    }
    bin.insert(0, a.to_string().chars().next().unwrap());
    while bin.len()<num as usize{
        bin.insert(0, '0');
    }
    bin
}

fn calc(input_seq: &str, values: &str)-> u32{
    const RADIX: u32 = 10;
    let mut op_cost = HashMap::new();
    let mut val_map = HashMap::new();
    let mut op_stack = Vec::new();
    let mut val_stack = Vec::new();
    let mut vars = "abcde".chars();
    let values= values.chars();

    op_cost.insert('*', 2);
    op_cost.insert('+', 1);
    op_cost.insert('(', 4);
    op_cost.insert(')', 0);
    op_cost.insert('-', 3);
    //print!("Starting {}", input_seq);

    for i in values{
        val_map.insert(vars.next().unwrap(), i.to_digit(RADIX).unwrap());
    }

    let mut chars = input_seq.chars();


    let mut op = chars.next();

    while op!=None{
        //println!("=Iteration=");

        if op_stack.is_empty() && op_cost.contains_key(&op.unwrap()){
            //println!("1 P");
            if op.unwrap()==')'{
                op = chars.next();
                continue;
            }
            op_stack.push(op.unwrap());

            op = chars.next();
        }
        else if val_map.contains_key(&op.unwrap()){
            //println!("2 P");
            val_stack.push(*val_map.get(&op.unwrap()).unwrap());
            op = chars.next();
        }else if !op_stack.is_empty(){
            if op_cost.get(op_stack.last().unwrap())>op_cost.get(&op.unwrap()){
                //println!("3 P");
                match op_cost.get(&op_stack.pop().unwrap()){
                    Some(1) => {
                        //println!("Executing +");
                        let a = val_stack.pop().unwrap();
                        let b = val_stack.pop().unwrap();
                        let x:u32=bin_plus(a, b);
                        //println!("{} + {} = {}", a, b, x);
                        val_stack.push(x);
                    },
                    Some(2) => {
                        //println!("Executing *");
                        let a = val_stack.pop().unwrap();
                        let b = val_stack.pop().unwrap();
                        let x:u32=bin_x(a, b);
                        //println!("{} * {} = {}", a, b, x);
                        val_stack.push(x);
                    },
                    Some(3) => {
                        //println!("Executing not");
                        let a = val_stack.pop().unwrap();
                        let x:u32=not(a);
                        //println!("not {} = {}", a, x);
                        val_stack.push(x);
                    },
                    _ => {1+1;},
                }
            }else if  op_cost.get(op_stack.last().unwrap())<=op_cost.get(&op.unwrap()){
                op_stack.push(op.unwrap());
                op = chars.next();
            }
        }else{
            op = chars.next();
        }

        /*print!("\nOperations: ");
        for i in &op_stack{
            print!("{} ", i)
        }
        print!("\nValues: ");
        for i in &val_stack{
            print!("{} ", i)
        }
        println!()*/

    }

    while !op_stack.is_empty(){
        match op_cost.get(&op_stack.pop().unwrap()){
            Some(0) => {op_stack.pop();},
            Some(1) => {
                let a = val_stack.pop().unwrap();
                let b = val_stack.pop().unwrap();
                let x:u32=bin_plus(a, b);
                val_stack.push(x);
            },
            Some(2) => {

                let a = val_stack.pop().unwrap();
                let b = val_stack.pop().unwrap();
                let x:u32=bin_x(a, b);
                val_stack.push(x);
            },
            Some(3) => {
                let a = val_stack.pop().unwrap();
                let x:u32=not(a);
                val_stack.push(x);
            },
            _ => print!("none"),

        }
        /*print!("\nOperations: ");
        for i in &op_stack{
            print!("{} ", i)
        }
        print!("\nValues: ");
        for i in &val_stack{
            print!("{} ", i)
        }
        println!()*/
    }
    val_stack[0]
}

fn main() {
  let mut input_seq = String::new();
  let mut x = 0;
  let mut vars = String::from("abcde");
  print!("- = not\n+ = or\n* = and\nfor variebles use a b c d\nExample: -b+a*c\n");
  print!("Enter function:\n");
  io::stdin().read_line(&mut input_seq);
  //calc("b+a+c","000");
  for char in input_seq.chars(){
      if vars.contains(char) {
          x = x + 1;
          vars.retain(|a| a!= char)
      }
  }
  print!("Starting {}", input_seq);
  println!("Found {} Vars", x );
  for i in 0..2_u32.pow(x){
      //println!("{}",i);
      let vals = bin(i, x);
      println!("f({})  = {}", &vals, calc(&input_seq, &vals));
  }
  //println!("{}",calc (&input_seq, "101"));
  //println!();

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc1(){
        assert_eq!(calc("-b*a+c","000"), 0);
        assert_eq!(calc("(a+c)*b", "101"), 0);
        assert_eq!(calc("-b+a*c","111"), 1);
    }

    #[test]
    fn test_calc2(){
        assert_eq!(calc("-b*a+(c*b)","100"), 1);
        assert_eq!(calc("(a+(-c))*b", "101"), 0);
        assert_eq!(calc("a+b+c", "101"), 1);
        assert_eq!(calc("a+b+c+d", "1011"), 1);
    }

    #[test]
    fn test_bin(){
        assert_eq!(bin(2, 3), "010");
        assert_eq!(bin(1, 1), "1");
        assert_eq!(bin(0, 1), "0");
        assert_eq!(bin(8, 4), "1000");
        assert_eq!(bin(34, 6), "100010");
        assert_eq!(bin(15, 4), "1111");
        assert_eq!(bin(20, 5), "10100");
    }
}
