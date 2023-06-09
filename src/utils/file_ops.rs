/* Read and Write file operations:

 Support for:
    CSV,
    JSON?

 */

use std::fs;

pub fn write_csv(path: &str, contents: String) -> std::io::Result<()> {
   fs::write(path, contents)?;

   Ok(())
}