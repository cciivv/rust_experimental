fn main() {

/* have to comment out uninit for compilation ("unbounded variable type (0.12? July 26th rustc build) */
//	let uninit;
	let uninit_unshadowed;
	let init = 4i;

	//begin scoped block
	{
		println!("init = {} before", init);
		let uninit = 2i;
		println!("uninit = {} after shadow", uninit);
		let init = -3i;
		println!("init = {} after shadow", init);
		uninit_unshadowed = 42i;
		println!("uninit_unshadowed = {} after being set (inner scope)", uninit_unshadowed);
	}

	println!("init = {}, outer scope, should be 4", init);
//	println!("uninit = {}, outer scope, should error", uninit);
	println!("uninit_unshadowed = {}, outer scope, should be 42", uninit_unshadowed);
}
