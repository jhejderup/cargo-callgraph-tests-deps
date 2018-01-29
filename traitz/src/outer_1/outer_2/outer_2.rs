
use traits::mytrait;

pub struct Outer2;

impl mytrait::MyTrait for Outer2 {
     fn hello(&self) -> i32 {
        0
     }
}

