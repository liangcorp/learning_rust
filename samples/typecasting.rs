fn main() {
    let a: i32 = 10;

    let b: i64 = a;     // Won't compile (mismatch types)

    let b: i64 = a as i64;  // Works
    let b: i64 = a.into();  // Works

    // Addition for typecasting
    let b: i64 = a as i64 + 10;
    let b: i64 = (a + 10).into();

    let a: i64 = a as i64 + 10; // a is not recreated until the end
}