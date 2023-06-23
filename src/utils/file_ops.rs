/* Read and Write file operations:

 Support for:
    CSV,
    JSON?

 */

use std::fs;
use std::error::Error;


pub fn write_csv(path: &str, contents: String) -> std::io::Result<()> {
   
   fs::write(path, contents)?;

   Ok(())
}


pub fn read_csv(path: &str) -> Result<String, Box<dyn Error>> {

   let contents = fs::read_to_string(path)?;

   Ok(contents)
}