#[derive(Debug)]
struct File {
    name: String;
    date: Vec<u8>;
}

fn main() {
    let fl = File {
        name: String::from("fl.txt"),
        data: Vec::new(),
    };

    let fl_name = &fl.name;
    let fl_length = &fl.data.len();

    println!("{:?}", fl);
    println!("{} is {} bytes long", fl_name, fl_length);
}