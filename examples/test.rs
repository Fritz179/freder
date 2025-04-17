struct Y(i32);
impl From<i32> for Y {
    fn from(x: i32) -> Self {
        Y(x)
    }
}

trait A {
    fn a_impl(&self, x: Y) {
        println!("A::a_impl: {}", x.0);
    }
}

struct X;

impl A for X {
}

trait Ext {
    fn aa(&self, x: impl Into<Y>);
}

impl<T: A + ?Sized> Ext for T {
    fn aa(&self, x: impl Into<Y>) {
        self.a_impl(x.into());
    }
}

fn main() {
    let x = X;
    x.aa(4);
    
    let x_ref: &dyn A = &x;
    x_ref.aa(5);
}


