mod submodule;

pub use submodule::inside_sub;

pub fn my_library_function() {
    println!("Inside my_library_function");
    inside_sub();
}
