<<<<<<< HEAD
// method to print the get value
fn value(n:Option<&char>) 
{
    println!("Element of vector {:?}",n);

}
fn main(){

    let v = vec!['R', 'U', 'T', 'A', 'C', 'I', 'A', 'N'];

    let mut input1 = String::new();
    println!("\nEnter an index value btw (0 - 8)");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");

    // Index is the non negotiable value which is smaller that the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");

    // Getting value at given index value
    let ch: Option<&char> = v.get(index);
    value(ch);
=======
fn main() {
    println!("Hello, world!");
>>>>>>> 565c5db249224695a0809aa5e0cb92269605c43f
}
