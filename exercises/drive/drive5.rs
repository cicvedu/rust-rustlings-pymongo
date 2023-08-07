// drive5.rs
// Your task is to make the testcase be able to call the `my_demo_function` in module Foo.
// the `my_demo_function_alias` is an alias for `my_demo_function_alias`, so the two line of
// code in the testcase should call the same function.
// You should not modify any existing code. All you need to do is add two line of attributes.





extern  {
    // #[link_name = "my_demo_function"]
    #[no_mangle]
    fn my_demo_function(a:u32) -> u32;
    // #[link(name = "my_demo_function")]
    #[no_mangle]
    // link(name) means link of dylib
    // link_name  means link of function name
    // #[link(name = "my_demo_function")]
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a:u32) -> u32;
}


mod Foo{
    #[no_mangle]
    fn my_demo_function(a:u32) -> u32 {a}
}

#[cfg(test)]
mod tests {
    // extern crate super::Foo;
    use super::*;

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
