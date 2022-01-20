use std::io;

fn main() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] =     [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(sv);
    print(sa);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}


// ask for name
fn set_name() -> io::Result<()> {
    // gets the name
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    
    Ok(())
}