mod day1;
mod day2;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a number to run the corresponding AoC day");
        return Ok(());
    }

    match args[1].as_str() {
        "1" => {
            let file_path = "src/calibrations.txt";
            let result = day1::day1_trebuchet(file_path)?;
            println!("{}", result);
        }
        "2" => day2::day2(),
        _ => println!("Invalid day number"),
    }

    Ok(())
}
