// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.


// collect() 可以获取任何可迭代的内容，并将其转换为相关集合


#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b == 0 {
        Ok(a / b)
    } else {
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    
    // into_iter() 返回的是可消费的迭代器，这类迭代器一旦使用就无法再次使用
    // 不使用 iter()，因为它是对 numbers 的一个引用，但是不转移所有权
    // 所以下面的 into_iter 是转移所有权的，find 本身操作的还是 numbers，而 division_results 只是 numbers 的一个迭代器。
    // 如果使用的是 into_iter() 后而不使用 clone 则会把 numbers 的所有权转移，
    // 而且 numbers 会在 match 那一行执行完 find 后 drop，这样后续的操作就会对一个已经 drop 的 vector 操作。
    // 总的来说，无论是 iter() 还是 into_iter() 如果不 clone ，在 find 后都会对空迭代器操作
    let mut division_results = numbers.iter().map(|n| divide(*n, 27));
    
    // clone() 使后续的操作不影响原始迭代器
    // find() 在可消费迭代器上会消耗迭代器中的元素，一旦找到满足条件的元素就会返回，并且后续迭代器不可用
    // 在不可消费迭代器上，不会消耗元素
    // 最主要的是 division_results 并不是 mutable，所以不能在后续对其进行 find 操作
    // find 操作后把迭代器 drop了，None 那里就不能再对迭代器操作，所以 clone 一下才能不影响 None 那里
    match division_results.clone().find(|x| x.is_err()) {
        Some(x) => Err(x.unwrap_err()),
        None => Ok(division_results.map(|x| x.unwrap()).collect()), // 如果不 clone，这里就在对空迭代器操作
    }

    // 下面是一种容易理解的写法
    // let mut ans = Vec::new();
    // for i in division_results {
    //     match i {
    //         Ok(val) => ans.push(val),
    //         Err(e) => return Err(e),
    //     }
    // }
    // Ok(ans)
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
