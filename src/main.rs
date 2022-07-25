mod chunk;

use chunk::Chunk;
use std::fs;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

fn main() {
    println!("Reading image file...");

    let read_result = fs::read("./images/house.png").expect("Can't read the image file");
    let (header, data) = read_result.split_at(8);

    if header != PNG_HEADER {
        panic!("Incorrect image format! png image expected");
    }

    println!("Reading image file - OK");
    println!("Reading chunks...");

    let chunks = read_chunks(data);

    for chunk in chunks {
        println!("chunk_length: {}", chunk.length());
        println!("chunk_type: {}", chunk.chunk_type());
        println!("crc: {:?}", chunk.crc());
    }

    println!("Reading chunks - OK");
}

fn read_chunks(image_data: &[u8]) -> Vec<Chunk> {
    let mut data = image_data;
    let mut chunks: Vec<Chunk> = Vec::new();

    while !data.is_empty() {
        let chunk_length = u32::from_be_bytes(data[0..4].try_into().unwrap()) + 12;
        let (head, tail) = data.split_at(chunk_length.try_into().unwrap());
        let chunk = Chunk::new(head);

        chunks.push(chunk);
        data = tail;
    }

    return chunks;
}
