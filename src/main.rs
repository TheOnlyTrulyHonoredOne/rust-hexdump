use std::fs;
use std::env;
use std::fmt::Write;

fn main() {
    // accepts and reads file
    let filename = env::args().nth(1).expect("Please provide a filename");
    let contents = fs::read(&filename);
    

    // prints hex codes by byte
    match contents {
        Ok(bytes) => {
            let mut hex_output = String::new();

            for byte in bytes {
                write!(&mut hex_output, "{:02x} ", byte).unwrap();
                //println!("{}", hex_output);
            }

            fs::write("output.txt", &hex_output).expect("Failed to write output file");
    
        }
        Err(error) => println!("Error: {}", error),
    }


    
}