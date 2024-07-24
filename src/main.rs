use core::slice;
use std::{
    io::{BufReader, Read, Seek},
    path::PathBuf,
    str::FromStr,
};

#[repr(C, packed)]
#[derive(Debug, Default)]
struct BmpHeader {
    signature: [u8; 2], // "BM" signature
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    data_offset: u32,
    info_header_size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    image_size: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    // ... other optional header fields (compression, color table size, etc.)
}

fn main() {
    let path = PathBuf::from_str("images").unwrap();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();

        let filepath = entry.path();
        dbg!(&filepath);
        let mut file = std::fs::File::open(filepath).unwrap();

        let mut reader = BufReader::new(&mut file);

        let mut header = BmpHeader::default();
        let header_size = std::mem::size_of::<BmpHeader>();

        let header_slice =
            unsafe { slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size) };

        println!("reading {} bytes", header_size);
        reader.read_exact(header_slice).unwrap();
        dbg!(&header);

        reader
            .seek(std::io::SeekFrom::Start(header.data_offset as u64))
            .unwrap();

        let mut pixel_data = Vec::new();

        reader.read_to_end(&mut pixel_data).unwrap();

        let mut result_upside_down = [0u8; 16 * 32];
        let mut index = 0usize;

        for pixel_data in pixel_data.chunks_exact(4) {
            let first = pixel_data.first().unwrap();
            if *first > 0 {
                result_upside_down[index] = 1;
            }

            index += 1
        }

        let mut result = [0u8; 2 * 32];
        index = 0usize;

        for chunk in result_upside_down.chunks_exact_mut(8).rev() {
            for (idx, num) in chunk.iter().enumerate() {
                result[index] |= *num << 7 - idx;
            }

            dbg!(index, &result[index]);
            index += 1;
        }

        dbg!(result);

        break;
    }
}
