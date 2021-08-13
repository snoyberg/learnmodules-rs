mod submodule;

fn main() {
    println!("Hello, world!");
    learnmodules::my_library_function();
    crate::submodule::inside_sub();
    learnmodules::inside_sub();
}
