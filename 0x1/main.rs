
//structs


struct Person {
    name:String,
    age:i32,
    oq_gosta_de_fazer:String
}

impl Person {
    fn sayhi(&self) -> String {
        format!("Oi, meu nome é {nome}, tenho {idade} anos de idade e gosto de {oq_gosta_de_fazer}.", nome = self.name, idade = self.age, oq_gosta_de_fazer = self.oq_gosta_de_fazer)
    }
}



#[allow(unused_variables)]
fn main() {

    let mateus = Person { name: String::from("Mateus"), age: 18, oq_gosta_de_fazer: String::from("Programar")};
    let bruno = Person { name: String::from("Bruno"), age: 19, oq_gosta_de_fazer: String::from("Plantar")};
    
    println!("{}", mateus.sayhi());
    println!("{}", pedro.sayhi());

}

