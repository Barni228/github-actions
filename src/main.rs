use num_traits::AsPrimitive;

#[allow(dead_code)]
/// Add, which is unnecessarily generic
fn add<Out>(a: impl AsPrimitive<Out>, b: impl AsPrimitive<Out>) -> Out
where
    Out: Copy + 'static + std::ops::Add<Output = Out>,
{
    a.as_() + b.as_()
}

fn main() {
    println!("Hello, world! But better 😏");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add::<usize>(2i32, 2u8), 4);
    }
}
