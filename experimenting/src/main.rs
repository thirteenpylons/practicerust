#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File,
        save_to: &mut Vec<u8>) -> ! {
            unimplemented!()
        }

fn main() {
    let mut fl = File::from("fl.txt");
    open(&mut fl);
    //read(fl, vec![]);
    close(&mut fl);
}
