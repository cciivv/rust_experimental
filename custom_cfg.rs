/*
	Condition	 				|	Result
	blank						|	error(unresolved function)
	--cfg rustc_condition=false	|	error(non-string literals are not allowed in meta-items)
	--cfg rustc_condition		|	builds
*/
#[cfg(rustc_condition)]
fn conditional_function() {
	println!("Condition met!");
}

fn main() {
	conditional_function();
}
