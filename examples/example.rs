use globals::unsafe_global;

#[unsafe_global]
static mut MY_GLOBAL: u32 = 0;

#[unsafe_global]
static mut MY_GLOBAL3: u32;

#[unsafe_global]
static mut GLOBAL_STRING: String;




fn main() {
    my_global3_init(|| { 0 });
    global_string_init(|| { String::from("asdf") });
    *my_global3() = 14;
    *my_global3() = 23;
    *global_string() = String::from("asdf");
    (*global_string()).push_str("qwerty");

    println!("Hello, {}, {}, {}", my_global(), my_global3(), global_string());

}
