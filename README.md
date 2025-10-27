# RLE Compression
This is my implementation of an RLE compression algorithm in Rust.

## Algorithm Explanation

I chose this compression algorithm because of the relevance for repeated bytes in the buffer given to it. Since it counts the occurrences of repeated consective bytes, it is efficient at dealing with large data sets that have these patterns.

It uses a while loop to go through the whole array, counting how many bytes of the same value it sees. Upon encountering a new byte, it appends the *byte_count* and the byte itself to a new vector. This new vector is in charge of having the pattern where the pattern is <count_of_byte, byte> repeated for however many times it's needed.

Lastly, it assigns the new vector to *data_ptr* and zeros any leftover bytes.

This algorithm also allows for decompression since there is a fixed structure of <count, byte>. The decompression algorithm would just need to loop over the new_size, making an output buffer out of the number of occurrences of each byte.

## Running
1. [Install Rust](https://rust-lang.org/tools/install/)
2. Run `rustc main.rs` to compile an executable
3. Run `./main` to run.