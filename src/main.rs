use core::slice::{self, from_raw_parts_mut};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    mem::size_of,
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

fn read_header(reader: &mut BufReader<&mut File>) -> BmpHeader {
    let mut header = BmpHeader::default();
    let header_size = size_of::<BmpHeader>();

    let header_slice = unsafe { from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size) };

    println!("reading {} bytes", header_size);
    reader
        .read_exact(header_slice)
        .expect("expecting at least enough bytes to read header");

    dbg!(&header);
    header
}

fn read_pixels(mut reader: BufReader<&mut File>, header: BmpHeader) -> Vec<u8> {
    reader
        .seek(std::io::SeekFrom::Start(header.data_offset as u64))
        .unwrap();

    let mut pixel_data = Vec::new();

    reader.read_to_end(&mut pixel_data).unwrap();

    // flip vertically
    pixel_data.reverse();

    let mut pixel_data: Vec<u8> = pixel_data
        .chunks_exact(4)
        .map(|chunk| if *chunk.first().unwrap() > 0 { 1 } else { 0 })
        .collect();

    pixel_data
        .chunks_exact_mut(header.width as usize)
        .for_each(|row| row.reverse());

    let pixel_data: Vec<u8> = pixel_data
        .chunks_exact(8) // encoding into a byte (size 8)
        .map(|c| {
            let mut result = 0u8;

            for (idx, item) in c.iter().enumerate() {
                result |= *item << 7 - idx;
            }

            result
        })
        .collect();

    pixel_data
}

fn main() {
    let path = PathBuf::from_str("images").unwrap();
    let mut output_path = PathBuf::from_str("output").unwrap();
    std::fs::create_dir_all(&output_path).unwrap();

    output_path.push("result.txt");

    let mut output_file = File::create(output_path).unwrap();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();

        let filepath = entry.path();

        dbg!(&filepath);
        let mut file = File::open(filepath).unwrap();

        let mut reader = BufReader::new(&mut file);

        let header = read_header(&mut reader);

        let pixel_data = read_pixels(reader, header);

        for chunk in pixel_data.chunks_exact(2) {
            for num in chunk {
                for bit_set in 0..8 {
                    let value = (*num << bit_set) & 128;

                    if value > 0 {
                        print!("{}", 1);
                    } else {
                        print!("{}", 0);
                    }
                }
            }

            println!()
        }

        output_file.write_all(&pixel_data.as_slice()).unwrap();
        output_file.write(b"\n").unwrap();
    }
}
