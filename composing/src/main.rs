fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_and_multiply = compose(|x| x + 2, |x| x * 2);
    let divide_and_subtract = compose(|x| x / 2, |x| x - 2);

    // let finally = compose(add_and_multiply, divide_and_subtract);
    let finally = compose(divide_and_subtract, add_and_multiply);
    println!("Result is {}", finally(5));
}
