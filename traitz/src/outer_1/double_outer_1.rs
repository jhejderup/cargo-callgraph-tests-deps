use traits::mytrait;


pub struct TwoArgs(pub i32, pub i32);

impl mytrait::MyTrait for TwoArgs {
    fn hello(&self) -> i32 {
        self.0 + self.1 //the panic function comes because we have an addition!!
     }
}