
fn main(){

    let mut counter:u8 = 0;

    loop {

        counter = counter + 1u8;

        if counter < 0b101u8 {
            println!("lt 5 -> ({}).", counter);
        }else{
            println!("jumping to completion...");
            break;
        }
    }

    println!("End");

}

