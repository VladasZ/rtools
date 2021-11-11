fn main() {
    let i_32: i32 = -1;
    let i_322: i32 = -2147483648;
    let i_64: i64 = 2147483648;

    println!("{:b}", i_32);
    println!("{:b}", i_322);
    println!("{:b}", i_64);

    println!("{:b}", i32::MIN);
}
