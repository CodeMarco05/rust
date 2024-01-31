fn main() {

    let x:String = String::from("my Value");
    let y:&str = "Test";

    print_value(&x);


    println!("The value afterwards {y}");
}

fn print_value (x: &String){
    println!("The value {x}");
}
