pub struct Config {
    argument: String,
    print_screen: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let argument = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get argument")
        };

        let print_screen = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get string to print")
        };

        Ok(Config { argument, print_screen })
    }
}

pub fn print_to_screen(config: Config) -> Result<(), &'static str>{
    if config.argument != String::from("echo") {
        return Err("command not found");
    }else {
        println!("{:#?}", config.print_screen);
    }

    Ok(())
}