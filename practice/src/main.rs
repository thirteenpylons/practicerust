use std::io;

fn main() {
    let mut n = set_name();

    println!("{}", n.len());

}

// ask for name
fn set_name() -> io::Result<()> {
    // gets the name
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    
    Ok(())
}