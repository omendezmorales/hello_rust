/// Composes two functions together.
///
/// Given two functions, `first_function` and `second_function`, where
/// `first_function` takes an argument of type `A` and returns a value of type `B`,
/// and `second_function` takes an argument of type `B` and returns a value of type `C`,
/// this function returns a new function that takes an argument of type `A` and returns a value of type `C`.
///
/// The returned function applies `first_function` to its input, and then applies `second_function` to the result.
///
/// # Examples
///
/// ```
/// let add_then_multiply = compose(&|x| x + 2, &|x| x * 2);
/// assert_eq!(add_then_multiply(3), 10); // (3 + 2) * 2 = 10
/// ```
fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_then_multiply = compose(|x| x + 2, |x| x * 2);
    let divide_then_subtract = compose(|x| x / 2, |x| x - 2);

    let finally = compose(add_then_multiply, divide_then_subtract);
    println!(
        "Result of add  ∘ multiply, followed by divide ∘ subtract is {}",
        finally(10)
    );
}
