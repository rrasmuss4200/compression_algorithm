fn main() {
    let bytes_to_compress = &mut [
    0x03, 0x74, 0x04, 0x04, 0x04, 0x35, 0x35, 0x64,
    0x64, 0x64, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x56, 0x45, 0x56, 0x56, 0x56, 0x09, 0x09, 0x09
    ];
    println!("Original size {}", bytes_to_compress.len());
    let compressed_bytes: usize = compress_bytes(&mut bytes_to_compress[..], 24);
    println!("Compressed size {}", compressed_bytes);

    println!("Try large data set:");
    let mut large_array = Vec::new();
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
    let compressed_bytes: usize = compress_bytes(&mut large_array[..], 600);
    println!("Compressed size large {}", compressed_bytes);

}

fn compress_bytes(data_ptr: &mut [u8], data_size: usize) -> usize {
    let mut byte_count: usize = 1;
    let mut i = 0;
    let mut new_vec: Vec<u8> = Vec::new();
    while i < data_size - 1 {
        if data_ptr[i] == data_ptr[i+1] {
            byte_count += 1;
        } else {
            new_vec.push(byte_count as u8);
            new_vec.push(data_ptr[i]);
            byte_count = 1;
        }
        i += 1
    }

    // handle last run
    new_vec.push(byte_count as u8);
    new_vec.push(data_ptr[data_size - 1]);
    let data_ptr = &new_vec;
    data_ptr.len()
}