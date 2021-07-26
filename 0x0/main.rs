

fn sum(num1:u64, num2:u64) -> u64 {
    num1 + num2
}

#[allow(unused_variables)]
fn main (){

    let _test:i32 = 7;
    let test2:i32 = 14;


    let num1 = 3;
    let num2 = 4;
    let sum = sum(num1, num2);
    
    println!("A soma de {num1} e {num2} é {sum}", num1 = num1, num2 = num2, sum = sum);

}

