use std::io::prelude::*;
use std::fs::File;
use std::io::Bytes;

const COUNTER_MAX: u8 = u8::MAX;

fn write_compressed(character: u8, counter: u8, file_out: &mut File) {
    file_out.write_all(&[character, counter]).expect("Couldn't write all data");
}

fn compress_iter(file_in: &mut File, file_out: &mut File) {
    let mut previous: u8 = 0;
    let mut counter: u8 = 0;

    for byte in file_in.bytes() {
        let character = byte.unwrap();
        println!("Character {}", character);
        let diff: bool = previous != character;
        let counter_reset: bool = counter == COUNTER_MAX;
        if diff || counter_reset {
            println!("Reset");
            write_compressed(previous, counter, file_out);
            counter = 0;
            previous = character;
        } else {
            previous = character;
            counter += 1;
        }
    }
    write_compressed(previous, counter, file_out);
}


fn main() -> std::io::Result<()> {
    let mut f_in = File::open("file.expanded")?;
    let mut f_out = File::create("file.rle")?;
    compress_iter(&mut f_in, &mut f_out);
    Ok(())
}
