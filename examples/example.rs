use globals::unsafe_global;

#[unsafe_global]
static mut MY_GLOBAL: u32 = 0;

#[unsafe_global]
static mut MY_GLOBAL3: u32;




fn main() {
    my_global3_init(|| { 10 });
    *my_global3() = 14;
    println!("Hello, {}, {}", my_global(), my_global3());

}
