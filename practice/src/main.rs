use std::io;

fn main() {

    let mut m = "this";
    let x = *m;
    println!("{}", &String x);
}

// ask for name
fn set_name() -> io::Result<()> {
    // gets the name
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    
    Ok(())
}