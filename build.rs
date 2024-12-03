extern crate cc;



// C:/01_work/00_Tools/compiler/gcc-arm-9/bin/arm-none-eabi-gcc.exe

fn main() {
    let path  = std::env::var("PATH").unwrap();
    println!("PATH: {}", path.clone());
    let changed_path =  "C:/01_work/00_Tools/compiler/gcc-arm-9/bin;".to_owned() + &path;
    std::env::set_var("PATH", &changed_path);
    println!("PATH: after update{}", &changed_path);


    cc::Build::new().file("src/multiply.c")
    .define("CC", "arm-none-eabi-gcc.exe")
    .define("CC_ENABLE_DEBUG_OUTPUT", "1")
    .compile("multiply");
}
