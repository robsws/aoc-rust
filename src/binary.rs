
pub fn bin_to_dec(binary: &[bool]) -> u64 {
  let mut exp = 2u64.pow(binary.len() as u32 - 1);
  let mut total = 0;
  for bit in binary {
      if *bit {
          total += exp;
      }
      exp /= 2;
  }
  total
}