// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        // first 是字符串的第一个字符
        // as_str() 是将一个 String 转为切片 &str 引用
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]

// 在迭代器上可以调用 map() 方法
// 它会返回一个新的迭代器，不会修改原始迭代器，
// 可以用 .collect() 方法将迭代器中内容收集起来
// .map() 方法接受一个闭包作为参数。这个闭包定义了对每个元素的转换操作。
// 闭包中的代码将应用于迭代器中的每个元素，并返回转换后的值。

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&w| capitalize_first(w)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // reduce() 用于对迭代器中的元素执行归约操作，将它们合并为一个单一的值
    // 提供一个闭包，该闭包接受两个参数，表示累积值和当前元素，然后返回一个新的累积值。
    // 这个闭包操作将在迭代器中的每个元素上逐个应用，将它们逐步合并为一个结果值。
    // reduce() 方法返回一个 Option 值，可能包含最终的归约结果。如果输入迭代器为空，它将返回 None。

    // capitalize_words_vector(words).into_iter().reduce(|x, y| x + &y).unwrap_or(String::new())

    
    words.iter().map(|&word| capitalize_first(word)).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
