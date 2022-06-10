use std::{fs::File, io::{Read, Error}};

fn open_file_and_read() -> Result<String, Error> {
    let mut buf = String::new();
    File::open("./hello.txt")?.read_to_string(&mut buf)?;
    println!("xxxx {:}", buf);
    Ok(buf)
}

#[cfg(test)]
#[test]
fn test_open_file_aned_read() {
    open_file_and_read();
}
