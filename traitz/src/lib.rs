pub mod outer_1;
pub mod traits;

//generic fn, these function are materialized when they are called
pub fn overloadable<T: traits::mytrait::MyTrait> (arg: T) -> i32 {
    arg.hello();
    println!("hello was called!");
    arg.hello()
}

