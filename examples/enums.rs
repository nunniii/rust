

enum Mensage {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(u8, u8, u8)
}


/// Esta função verifica quais dados combinaram com os tipos definidos em enum Mensage e executa a expressão após "=>" caso a combinação seja real.
fn handle_mensage(mensage: Mensage){

    use Mensage::*;

    match mensage{
        Quit => println!("Exiting application..."),
        Move{x, y} => println!("Moving the cursor to ({x_position}, {y_position})...", x_position = x, y_position = y),
        Write(text) => println!("Writing '{text}'...", text = text),
        ChangeColor(red, green, blue) => println!("Change color to RGB({r}, {g}, {b})", r = red, g = green, b = blue)
    }

}


fn main(){


    let move_mensage = Mensage::Move{x: 12, y: 23};
    handle_mensage(move_mensage);

    let change_color_mensage = Mensage::ChangeColor(233, 34, 56);
    handle_mensage(change_color_mensage);

    let write_mensage = Mensage::Write(String::from("Hello, Rust!"));
    handle_mensage(write_mensage);

    let quit_mensage = Mensage::Quit;
    handle_mensage(quit_mensage);

}
