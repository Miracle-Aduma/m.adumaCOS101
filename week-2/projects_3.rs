fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// compound interest (depreciation)
	let a = p * (1.0 + (r / 100.0).powf(n));
	println!("Amount is {:.2}", a);
	let ci = p-a;
	println!("compound interest is {:.2}", ci);

} 