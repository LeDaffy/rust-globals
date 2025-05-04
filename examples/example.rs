use globals::{checked_get_mut_or_init, unchecked_global, checked_global, CheckedGlobal};

#[unchecked_global]
static mut MY_GLOBAL: u32 = 0;

#[unchecked_global]
static mut MY_GLOBAL3: u32;

#[unchecked_global]
static mut GLOBAL_STRING: String;


#[checked_global]
static mut GLOBAL_STRING3: String;



static mut C_GLOBAL: CheckedGlobal<String> = CheckedGlobal::uninit();

fn prints(string: &String) {
    println!("Hello, {}, {}, {}, {}", my_global(), my_global3(), string, global_string3().unwrap());
}

fn main() {
    checked_get_mut_or_init(&raw mut C_GLOBAL, || { String::from("unchecked") });
    


    _ = global_string3_or_init(|| { String::from("Checked for initialization") } );

    my_global3_init(0);
    global_string_init(String::from("asdf"));
    *my_global3() = 14;
    *my_global3() = 23;
    *global_string() = String::from("asdf123");
    (*global_string()).push_str("qwerty");

    prints(global_string());


}
