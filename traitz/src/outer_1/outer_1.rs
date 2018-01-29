use traits::mytrait;


pub struct OneArg(pub i32);

impl mytrait::MyTrait for OneArg {
     fn hello(&self) -> i32 {
        self.0
     }
}