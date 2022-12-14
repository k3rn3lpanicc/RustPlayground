use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Result;


fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?; if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}


fn main() -> Result<()>{
    let path = "file.txt";
    let mut output = File::create(path)?;
    write!(output, "Some text here")?;
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));  
    Ok(())
}
/*  SHA256 Hashing : 
 *      1) Working with files : write String into it
 *      2) Reading buffer from file
 *      3) Get the SHA256 hash of file content
 *      4) IMPORTANT : use std::io::Result for one generic input!!!!
 */
