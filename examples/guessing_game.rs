
use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    let secret_number = rand::thread_rng().gen_range(std::ops::Range{start: 0, end: 10});

    let mut controller:bool = false;
    while !controller {

    let mut guess:String = String::new();
   

        println!("Enter a guess between 0 and 9: ");
        stdin().read_line(&mut guess).expect("Error reading.");
        guess.pop();
        let guess:u8 = guess.parse().expect("Please, enter a number.");
        println!("Your guess is {}.", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your guess was low"),
            Ordering::Greater => println!("Your guess was hight"),
            Ordering::Equal => {
                
                controller = true;
                println!("you're right!")
            }
        }
    }
}

