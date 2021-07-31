#[allow(unused_parens)]
fn main(){
    
    for number in (0..(9+3)).rev() {
        println!("{}", number);
    }
    
    println!("\n\n---");

    for number in 0..(9+3) {
        println!("{}", number);
    }

}