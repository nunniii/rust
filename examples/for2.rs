
fn main(){

    for num in (std::ops::Range{start: 0, end: 10}.step_by(3).rev()){
        println!("{}", num);
    }

    let a = std::ops::Range{start: 0, end: 10};
    let b = 0..10;

    assert_eq!(a, b);


}
