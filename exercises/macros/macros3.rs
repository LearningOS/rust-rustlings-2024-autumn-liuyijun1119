// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    
    pub fn call_my_macro() {
    use crate::my_macro;
        my_macro!();
    }

    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
   macros::call_my_macro();
}
