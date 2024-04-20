use std::io::stdin;

fn add(a: f32, b:f32) -> f32{
    a + b


}
fn subtract(a: f32, b:f32) -> f32{
    a - b
}


fn multiply(a: f32, b:f32) -> f32{
    a * b
}


fn divide(a: f32, b:f32) -> f32{
    a /b
}


fn main() {
    loop {
        let mut task = String::new();
        println!("enter task ");
        stdin().read_line(&mut task)
            .expect("string invalid");

        let task  = task.trim();
        println!("{}", task);
        if task == "add"{

            let mut num1 = String::new();
            println!("Enter Number 1 ");
            stdin().read_line(&mut num1)
                .expect("Invalid STR");

            let mut num2 = String::new();
            println!("Enter Number 2 ");
            stdin().read_line(&mut num2)
                .expect("Invalid STR");

            let float1: f32 = num1.trim().parse().unwrap();
            let float2: f32 = num2.trim().parse().unwrap();
            let result = add(float1,float2);
            println!("{}",result);


        }
        else if task == "sub"{

            let mut num1 = String::new();
            println!("Enter Number 1 ");
            stdin().read_line(&mut num1)
                .expect("Invalid STR");

            let mut num2 = String::new();
            println!("Enter Number 2 ");
            stdin().read_line(&mut num2)
                .expect("Invalid STR");

            let float1: f32 = num1.trim().parse().unwrap();
            let float2: f32 = num2.trim().parse().unwrap();
            let result = subtract(float1,float2);
            println!("{}",result);


        }
        else if task == "mul"{

            let mut num1 = String::new();
            println!("Enter Number 1 ");
            stdin().read_line(&mut num1)
                .expect("Invalid STR");

            let mut num2 = String::new();
            println!("Enter Number 2 ");
            stdin().read_line(&mut num2)
                .expect("Invalid STR");

            let float1: f32 = num1.trim().parse().unwrap();
            let float2: f32 = num2.trim().parse().unwrap();
            let result = multiply(float1,float2);
            println!("{}",result);

        }
        else if task == "div" {

            let mut num1 = String::new();
            println!("Enter Number 1 ");
            stdin().read_line(&mut num1)
                .expect("Invalid STR");

            let mut num2 = String::new();
            println!("Enter Number 2 ");
            stdin().read_line(&mut num2)
                .expect("Invalid STR");

            let float1: f32 = num1.trim().parse().unwrap();
            let float2: f32 = num2.trim().parse().unwrap();
            let result = divide(float1,float2);
            println!("{}",result);

        }
        else {
            print!("Sorry invalid opp");
        }
    }
}
