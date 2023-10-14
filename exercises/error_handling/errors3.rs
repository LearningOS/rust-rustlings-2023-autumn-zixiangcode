// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    // ? 会让函数提前结束返回，所以看到所有使用 ? 的地方，返回值都是 Result<> 类型
    // 但是即使是 main 也有返回值，也可以让其返回 Result<> 类型
    // 而且 main 中的 ? 执行成功之后，其实没有任何返回值，所以是 ()
    // 还有，Result 和 Option 一样，匹配的不是结果本身，而需要套 Ok() 或 Some() 来解析结果
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
