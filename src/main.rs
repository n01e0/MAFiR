mod bf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [source file]", args[0]);
        std::process::exit(1);
    }

    let src = std::fs::read_to_string(&args[1]);
    match src {
        Ok(code) => bf::fuck(code),
        Err(err) => eprintln!("{}", err),
    }
}
