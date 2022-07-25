pub struct Chunk {
    length: u32,
    chunk_type: String,
    data: Vec<u8>,
    crc: [u8; 4],
}

impl Chunk {
    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &String {
        &self.chunk_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn crc(&self) -> [u8; 4] {
        self.crc
    }

    pub fn new(chunk_data: &[u8]) -> Chunk {
        let length = u32::from_be_bytes(chunk_data[0..4].try_into().unwrap());
        let chunk_type = String::from_utf8(chunk_data[4..8].try_into().unwrap()).unwrap();
        let (data, crc) = chunk_data[8..].split_at(length.try_into().unwrap());

        Chunk {
            length,
            chunk_type,
            data: data.to_vec(),
            crc: crc.try_into().unwrap(),
        }
    }
}
