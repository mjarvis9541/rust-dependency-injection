use std::sync::Arc;

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

struct DependencyInjector {
    dependency: Arc<dyn SomeTrait + Send + Sync>,
}

impl DependencyInjector {
    fn new(dependency: Arc<dyn SomeTrait + Send + Sync>) -> Self {
        Self { dependency }
    }

    fn execute(&self) {
        self.dependency.do_something();
    }
}

fn main() {
    let a = Arc::new(StructA) as Arc<dyn SomeTrait + Send + Sync>;
    let injector = DependencyInjector::new(a);
    injector.execute();
}
