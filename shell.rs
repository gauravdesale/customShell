fn main() {
	lsh_loop();
}
fn lsh_loop {
	let x = &line as *const i32;
	let args = os::args_as_bytes();
	let status : i32;
	while(status) {
		println!("> ");
		x = lsh_read_line();
		args = lsh_split_line(x);
		status = lsh_execute(args);
		free(line);
		free(args);
	}
}	
