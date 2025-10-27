fn main() {
    let bytes_to_compress: &mut [u8; 24] = &mut [
    0x03, 0x74, 0x04, 0x04, 0x04, 0x35, 0x35, 0x64,
    0x64, 0x64, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x56, 0x45, 0x56, 0x56, 0x56, 0x09, 0x09, 0x09
    ];
    println!("Original size {}", bytes_to_compress.len());
    let compressed_size: usize = compress_bytes(&mut bytes_to_compress[..], 24);
    println!("Data is now size {}", compressed_size);

    println!("Trying large data set:");
    let mut large_array: Vec<u8> = Vec::new();
    for _ in 0..100 {
        large_array.push(0x00)
    }
    for _ in 0..100 {
        large_array.push(0x56)
    }
    for _ in 0..100 {
        large_array.push(0x08)
    }
    for _ in 0..100 {
        large_array.push(0x58)
    }
    for _ in 0..100 {
        large_array.push(0x7A)
    }
    for _ in 0..100 {
        large_array.push(0x00)
    }

    println!("Original size large {}", large_array.len());
    let large_compressed_size: usize = compress_bytes(&mut large_array[..], 600);
    println!("Compressed size large {}", large_compressed_size);
}

fn compress_bytes(data_ptr: &mut [u8], data_size: usize) -> usize {
    let mut byte_count: usize = 1;
    let mut i: usize = 0;
    let mut new_vec: Vec<u8> = Vec::new();
    while i < data_size - 1 {
        if data_ptr[i] == data_ptr[i+1] {
            byte_count += 1;
        } else {
            new_vec.push(byte_count as u8);
            new_vec.push(data_ptr[i]);
            byte_count = 1;
        }
        i += 1;
    }

    // handle last run
    new_vec.push(byte_count as u8);
    new_vec.push(data_ptr[data_size - 1]);

    let compressed_size: usize = new_vec.len();

    // populate data_ptr with compressed data
    data_ptr[..compressed_size].copy_from_slice(&new_vec);

    for b in &mut data_ptr[compressed_size..] {
        *b = 0;
    }

    compressed_size
}