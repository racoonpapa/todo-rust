mod todo;

use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("todo.json")?;

    let mut start = Instant::now();
    let items = todo::List::from_json(data.as_str())?;
    let elapsed_from = start.elapsed().as_secs_f64();

    // items.print();

    start = Instant::now();
    let output = items.to_json()?;
    let elapsed_to = start.elapsed().as_secs_f64();

    // println!("{}", output);
    println!("fromJson: {} seconds, toJson: {} seconds", elapsed_from, elapsed_to);

    Ok(())
}
