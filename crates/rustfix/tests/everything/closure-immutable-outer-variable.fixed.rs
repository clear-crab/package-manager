// Point at the captured immutable outer variable

fn foo(mut f: Box<dyn FnMut()>) {
    f();
}

fn main() {
    let mut y = true;
    foo(Box::new(move || y = false) as Box<_>); //~ ERROR cannot assign to captured outer variable
}
