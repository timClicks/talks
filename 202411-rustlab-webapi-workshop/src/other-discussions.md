# Other Discussions

## Compiler lints

https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html


## Error



    

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<u8> = fs::read("image.jpg")?;
    assert_eq!(data[0..3], [0xFF, 0xD8, 0xFF]);
    
        
    
    Ok(())
}



// use thiserror; // 1.0.64

// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum OurError {
//     #[error("file system")]
//     Io(#[from] std::io::Error),
    
//     #[error("formatting")]
//     Fmt(#[from] std::fmt::Error),
    
//     #[error("unable to proceed because {0}")]
//     Custom(String),
// }

// fn main() -> std::result::Result<(), OurError> {
//     let result = std::fs::write("\\", "Hello")?; // std::io::Error !=> String

//     // match result {
//     //     Ok(_) => (),
//     //     Err(err) => eprintln!("error: help needed: {err}"), 
//     // };
    
//     Ok(())
//     // Err(OurError::Custom(String::from("danger")))
// }