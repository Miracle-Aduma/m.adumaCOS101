fn main() {
	let p:f64 = 1000.0;
	Let r:f64 = 1.0;
	let t:f64 = 2.0;

	// simple interest
	Let a = p * (1.0 + (r / 100.0)) * t
	println!("Amount is {}", a);
	Let si = a-p;
	println!("simple Interest is {}", si);

}