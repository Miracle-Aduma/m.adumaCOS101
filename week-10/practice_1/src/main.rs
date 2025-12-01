fn main() {
    let v =  vec![101, 250, 330, 400];
    // Vector v owns the object in heap

    // Only a single variable owns the heap at any given time
    let v2 = v;
    // Here two varaible owns the heap value, two pointers to the sam econtengt is not allowed in rust

    //Rust is a very smart in terms of memory access, so it detcts a race condition as two varaibles point to same heap

    println!("{:?}",v);
}
