fn main() {
    
    // A list of nos
    let v = Vec![15, 25, 35, 45, 55];
    print_vector(v);
    println!("{}",v[0]); //This line gives an error
}

fn print_vecor(x:Vec<i32>){

    println!("Insoide print_vector function {:?}",x);
}
