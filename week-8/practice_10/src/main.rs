fn main() {
    
    let b:(i32,bool,f64) = (30,true,4.9);
    print(b);

}

fn print(x:(i32,bool,f64)) {

    println!("Inside print mmethod");

    // Assigns a tuple to distinct varaibles
    let (age,is_male,cgpa) = x;
    println!("Age is {}, is_Male? {}, CGPA is {}",age,is_male,cgpa);
}
