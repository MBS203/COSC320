trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Corrected to use generic type parameters with trait bounds
fn some_func<T: SomeTrait + OtherTrait>(item: &T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    let some_struct = SomeStruct;
    let other_struct = OtherStruct;
    println!("SomeStruct: {}", some_func(&some_struct));
    println!("OtherStruct: {}", some_func(&other_struct));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(&SomeStruct));
        assert!(some_func(&OtherStruct));
    }
}

