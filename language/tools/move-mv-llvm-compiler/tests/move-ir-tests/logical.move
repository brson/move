module 0x100::Test {
  fun test_lt(a: u8, b: u8): bool {
    let c = a < b;
    c
  }
  fun test_le(a: u8, b: u8): bool {
    let c = a <= b;
    c
  }
  fun test_gt(a: u8, b: u8): bool {
    let c = a > b;
    c
  }
  fun test_ge(a: u8, b: u8): bool {
    let c = a >= b;
    c
  }
}
