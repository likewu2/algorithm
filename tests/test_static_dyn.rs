use std::sync::Mutex;
use lazy_static::lazy_static;

trait FooInterface { 
    fn new (&mut self) -> ();
}

struct DefaultFoo {}
struct SomeNonDefaultFoo {}
struct SomeOtherNonDefaultFoo {}

impl FooInterface for DefaultFoo { 
    fn new (&mut self) -> () {}
}



impl FooInterface for SomeNonDefaultFoo { 
    fn new (&mut self) -> () {}
}

impl FooInterface for SomeOtherNonDefaultFoo { 
    fn new (&mut self) -> () {}
}

/*
lazy_static!{
    static ref DEFAULT_FOO: DefaultFoo = DefaultFoo {};
}
*/
static mut DEFAULT_FOO: DefaultFoo = DefaultFoo {};


lazy_static!{
    static ref GLOBAL_FOO: Mutex<Box<&'static (dyn FooInterface + Sync)>> = Mutex::new(Box::new(&DEFAULT_FOO));
}
//lazy_static!{
//    static ref GLOBAL_FOO: Mutex<&'a dyn FooInterface> = Mutex::new(&DEFAULT_FOO);
//}

//lazy_static! {
//static ref GLOBAL_FOO: Mutex<Box<&'static (dyn FooInterface + Send + 'static)>> = {
//                                                                    let default_foo = DefaultFoo {};
//                                                                    Mutex::new(Box::new(&default_foo))
//                                                                };
//}

fn main() {
   // unsafe{
        //GLOBAL_FOO = &SomeNonDefaultFoo{};
        //GLOBAL_FOO = &SomeOtherNonDefaultFoo{};
        //let my_foo = FooInterface::new(*(*GLOBAL_FOO.lock().unwrap()));
//    }
}
