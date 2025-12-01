fn main() {
    
    let v = vec![20, 40, 60, 80];
    //vector v owns the object heap

    let v2 = v;
    let v2_return =  display(v2);
    println!("In main {:?}",v);
}

fn display(v:Vec<i32>)->Vec<i32> {
    // Returning same vector
    println!("Inside display {:?}",v);
    return v;
}
