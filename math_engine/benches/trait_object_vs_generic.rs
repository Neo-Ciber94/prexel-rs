#[macro_use]
extern crate bencher;

use bencher::{Bencher, black_box};

trait MyTrait {
    fn bar(&self);
}

const STRUCT_SIZE : usize = 100;
const SAMPLES : u64 = 1000;

#[allow(dead_code)]
struct A { b: [u32; STRUCT_SIZE] }
impl A {
    fn new() -> Self {
        Self{
            b: [u32::default(); STRUCT_SIZE]
        }
    }
}
impl MyTrait for A { fn bar(&self) { } }

#[allow(dead_code)]
struct B { b: [u32; STRUCT_SIZE] }
impl B {
    fn new() -> Self {
        Self{
            b: [u32::default(); STRUCT_SIZE]
        }
    }
}
impl MyTrait for B { fn bar(&self) { } }

fn take_trait_object(value: &impl MyTrait) {
    value.bar()
}

fn take_generic<T : MyTrait>(value: &T) {
    value.bar()
}

fn trait_object_bench(b: &mut Bencher){
    b.bench_n(SAMPLES, |bn|{
        bn.iter(||{
            let a = black_box(A::new());
            let b = black_box(B::new());

            take_trait_object(&a);
            take_trait_object(&b);
        })
    })
}

fn generic_bench(b: &mut Bencher){
    b.bench_n(SAMPLES, |bn|{
        bn.iter(||{
            let a = black_box(A::new());
            let b = black_box(B::new());

            take_generic(&a);
            take_generic(&b);
        })
    })
}

benchmark_group!(benches, trait_object_bench, generic_bench);
benchmark_main!(benches);