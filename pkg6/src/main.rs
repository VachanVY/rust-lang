use std::{fs::File, io::{self, Read}};
use std::error::Error;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut buf = String::new();
    let mut fcontent = File::open(path)?;

    // that ? is a shorthand for match
    // match File::open("src/hello.txt") {
    //     Ok(file) => file,
    //     Err(err) => return Err(err.into()),
    // };
    fcontent.read_to_string(&mut buf)?;
    Ok(buf)

    // fs::read_to_string(path) // this is a shorthand for the above code
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file("src/hello.txt")?;
    println!("{}\n", content);
    println!("f-strings:\n{content} ");

    let integer: u8 = "127".parse().unwrap();
    assert_eq!(integer, 127);

    Ok(())
}
