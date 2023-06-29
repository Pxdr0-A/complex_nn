/*

 Some basic error handling functionalities.

 */

pub fn input_test(input_len: usize, n_units: usize) -> Option<usize> {
  if (input_len / n_units) * n_units == input_len {
    if input_len / n_units >= 2 {
      let mult = input_len / n_units;
      Some(mult)
    } else {
        None
    }
  } else {
    None
  }
}