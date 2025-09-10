use std::io;
use std::process;
use regex::Regex;

pub mod parser;

fn main() {
    println!("Select from Vector or Exit\n");

    let mut ask = typeInput(String::new());

    find(ask);
}

fn typeInput(_i: String) -> String {
    let mut _i: String = String::new();
    io::stdin().read_line(&mut _i).expect("Failed to read");
    return _i.to_lowercase();
}

fn find(input: String){
    match input.trim(){
        "vector" => VectorMath(),
        "exit" => Exit(),
        _ => main(),
    }
}

fn VectorMath(){
    println!("You selected vector mathematics.\n \n Select from Magnitude, Dotproduct, or Crossproduct\n");

    let mut input: String = typeInput(String::new());

    match input.trim(){
        "magnitude" => {
            println!("Input numbers for vector calculations consecutively (x,y,z)\n");
            MagnitudeCalculation(ParseNumbers());
        },
        "dotproduct" => {
            println!("Write as x1,y1,z1,x2,y2,z2\n");
            DotProductCalculation(ParseNumbers());
        }
        "crossproduct" => {
            println!("Write as x1,y1,z1,x2,y2,z2\n");
            CrossCalculation(ParseNumbers());
        },
        "exit" => Exit(),
        _ => ResetToMain(),
    }
}

fn ParseNumbers() -> Vec<i32>{
    let byte_string: &str = &typeInput(String::new());

    return parser::TakeNewInput(byte_string);
}

fn MagnitudeCalculation(vec: Vec<i32>){
    if(vec.len() == 3){
    let product = ((vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]) as f32).powf(0.5 as f32);
    println!("{}",product);}

    else if(vec.len() == 2){
    let product = ((vec[0] * vec[0] + vec[1] * vec[1]) as f32).powf(0.5 as f32);
    println!("{}",product);
    }

    else{
        ResetToMain();
    }
}

fn CrossCalculation(vec: Vec<i32>){
    if(vec.len() == 6){
        let product: Vec<i32> = vec![vec[1]*vec[5] - vec[2]*vec[4],vec[2]*vec[3]-vec[0]*vec[5],vec[0]*vec[4]-vec[1]*vec[3]];

        println!("{}, {}, {}",product[0], product[1], product[2]);
    }else{
        ResetToMain();

    }
}

fn DotProductCalculation(vec: Vec<i32>){
    if(vec.len() == 6){
    let product = (vec[0] * vec[3] + vec[1] * vec[4] + vec[2] * vec[5]) as f32;
    println!("{}",product);}

    else if(vec.len() == 4){
    let product = (vec[0] * vec[2] + vec[1] * vec[3]) as f32;
    println!("{}",product);}

    else{
        ResetToMain();
    }
}

fn ResetToMain(){
    println!("Invalid Input.");
    main();
}

fn Exit(){
    println!("Closing software...");
    std::process::exit(0);
}
