// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // fold 方法从迭代器的起始元素开始，按顺序遍历元素，
    // 并在每次遍历时将一个累积值（accumulator）与当前元素进行操作。
    // 它接受一个初始累积值和一个闭包函数，该闭包函数接受两个参数：
    // 当前累积值和迭代器中的元素，然后返回一个新的累积值。
    // rfold 方法与 fold 方法类似，但它是从迭代器的末尾向前遍历元素，执行累积操作。
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
