use std::io;
use std::io::Write;
use xjb_db::inbound::parser::parse_sql;


fn main() -> io::Result<()> {
    run_main_loop()
}

fn run_main_loop() -> io::Result<()> {
    println!("XJB DB [Ver 0.0.1]");
    println!(">_> 2020 johnbanq. Don't use this in production.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let read = io::stdin().read_line(&mut input)?;

        if read == 0 {
            break;
        }

        if input.trim_end().eq_ignore_ascii_case("EXIT") {
            break;
        }

        let parsed = parse_sql(input.trim_end());
        match parsed {
            Ok(query) => println!("{:#?}", query),
            Err(e) => println!("{}", e)
        }
        io::stdout().flush().unwrap();
    }

    Ok(())
}
