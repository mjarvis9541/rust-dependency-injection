trait SomeTrait {
    fn do_something(&self);
}

struct StructA;
struct StructB;

impl SomeTrait for StructA {
    fn do_something(&self) {
        println!("StructA doing something");
    }
}

impl SomeTrait for StructB {
    fn do_something(&self) {
        println!("StructB doing something");
    }
}

struct DependencyInjector<T: SomeTrait> {
    dependency: T,
}

impl<T: SomeTrait> DependencyInjector<T> {
    fn new(dependency: T) -> Self {
        Self { dependency }
    }

    fn execute(&self) {
        self.dependency.do_something();
    }
}

fn main() {
    let a = StructA;
    let injector = DependencyInjector::new(a);
    injector.execute();
}
