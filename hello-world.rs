fn main(){
	hello_by_user();
}

fn hello_by_user(){
	let mut name = String::new();
	print!("What's your name? ");
	std::io::stdin().read_line(&mut name).unwrap();
	println!("Hello {}", name);
}

