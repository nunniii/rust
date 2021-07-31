
use rand::Rng;

/// Converte 0u8 em false e 1u8 em true.
fn converter_to_boolean(num:u8) -> bool {
    if num == 0b1u8 {true} else {false}
}

fn main(){

    let mut a:u8;
    let mut b:u8;
    let mut value:bool = false;

    while !value{
        a = rand::thread_rng().gen_range(std::ops::Range{start: 0, end: 2});
        b = rand::thread_rng().gen_range(std::ops::Range{start: 0, end: 2});

        value = converter_to_boolean(a) & converter_to_boolean(b);

        println!("(a, b) = ({}, {}) -> {}", a, b, value);
        
    }   
}
