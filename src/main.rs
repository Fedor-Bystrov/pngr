use std::fs;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

fn main() {
    println!("Reading image file...");

    let read_result = fs::read("./images/house.png").expect("Can't read the image file");
    let (header, _right) = read_result.split_at(8);

    if header != PNG_HEADER {
        panic!("Incorrect image format! png image expected");
    }

    println!("Reading image file - OK");
}
