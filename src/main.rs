mod lib;

use lib::Chunk;
use std::fs;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

pub struct PngImage {
    width: u32,
    height: u32,
    bit_depth: u8,
    colour_type: u8,
}

impl PngImage {
    pub fn new(chunks: Vec<Chunk>) -> PngImage {
        let mut image = PngImage {
            width: 0,
            height: 0,
            bit_depth: 0,
            colour_type: 0,
        };

        for chunk in chunks {
            match chunk.chunk_type().as_str() {
                "IHDR" => Self::parse_ihdr(chunk, &mut image),
                _ => println!("{}", chunk.chunk_type()),
            }
        }

        image
    }

    fn parse_ihdr(chunk: Chunk, image: &mut PngImage) {
        image.width = u32::from_be_bytes(chunk.data()[0..4].try_into().unwrap());
        image.height = u32::from_be_bytes(chunk.data()[4..8].try_into().unwrap());
        image.bit_depth = chunk.data()[8];
        image.colour_type = chunk.data()[9];
    }
}

fn main() {
    println!("Reading image file...");

    let read_result = fs::read("./images/house.png").expect("Can't read the image file");
    let (header, data) = read_result.split_at(8);

    if header != PNG_HEADER {
        panic!("Incorrect image format! png image expected");
    }

    println!("Reading image file - OK");
    println!("Reading chunks...");

    let chunks = parse_chunks(data);

    for chunk in &chunks {
        println!("chunk_length: {}", chunk.length());
        println!("chunk_type: {}", chunk.chunk_type());
        println!("crc: {:?}", chunk.crc());
    }

    println!("Reading chunks - OK");
    println!("Creating PngImage...");

    let image = PngImage::new(chunks);

    println!("width: {}", image.width);
    println!("height: {}", image.height);
    println!("bit_depth: {}", image.bit_depth);
    println!("colour_type: {}", image.colour_type);
}

fn parse_chunks(image_data: &[u8]) -> Vec<Chunk> {
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
