
use rand::Rng;

fn return_value() -> bool {
    if rand::thread_rng().gen_range(std::ops::Range{start: 0, end: 2}) == 0 {false} else {true}
}

fn main(){

    let mut condition:bool = false;

    while !condition{

        let n1:bool = return_value();
        let n2:bool = return_value();
        let n3:bool = return_value();
        let n4:bool = return_value();

        let nn1:bool = n1 && n3;
        let nn2:bool = n2 && n4;

        let r:bool = nn1 & nn2;

        println!("[(n1, n3), (n2, n4)] = [({}, {}), ({}, {})] => [{}, {}] => {}", n1, n3, n2, n4, nn1, nn2, r);

        if r {condition = true};

    }
}
