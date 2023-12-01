mod day1;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "src/calibrations.txt";
    let result = day1::day1_trebuchet(file_path)?;

    println!("{}", result);
    Ok(())
}
