fn main() {
	let mut max = 0;
	let mut min = 100;
	let f = |x: int| if x > max { max = x };
	let fref = |x: &int| if *x < min { min = *x };
	for x in [1, 2, 3].iter() {
		//Have to dereference 'x' since iter returns a reference
		f(*x);
		fref(x);
	}
}
