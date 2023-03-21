use lossy_text_compression::{COMPRESSOR_LEVEL_5, DECOMPRESSOR_LEVEL_3};

fn main() {
    let to_be_or_not_to_be = "To be, or not to be, that is the question: Whether 'tis nobler in the mind to suffer The slings and arrows of outrageous fortune, Or to take arms against a sea of troubles And by opposing end them.";
    let compressor = COMPRESSOR_LEVEL_5;
    let decompressor = DECOMPRESSOR_LEVEL_3;
    let compressed = compressor.compress(to_be_or_not_to_be);
    let decompressed = decompressor.decompress(compressed.clone());
    println!("Original: {}", to_be_or_not_to_be);
    println!("Compressed: {}", compressed);
    println!("Decompressed: {}", decompressed);
}