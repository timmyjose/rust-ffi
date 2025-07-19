use std::io;

extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn sub(a: i32, b: i32) -> i32;
    fn mul(a: i32, b: i32) -> i32;
    fn div(a: i32, b: i32) -> i32;
}

fn get_num() -> eyre::Result<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<i32>()?)
}

fn main() -> eyre::Result<()> {
    let (a, b) = (get_num()?, get_num()?);

    unsafe {
        println!("{a} + {b} = {}", add(a, b));
        println!("{a} - {b} = {}", sub(a, b));
        println!("{a} * {b} = {}", mul(a, b));
        println!("{a} / {b} = {}", div(a, b));
    }

    Ok(())
}
