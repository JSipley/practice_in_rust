
pub fn reverse(x: i32) -> i32
{
    let mut result: i32 = 0;
    let mut num = x;
    while num != 0 {
        let digit = num % 10;
        if let Some(new_result) = result.checked_mul(10)
                                             .and_then(|r| r.checked_add(digit)) {
            result = new_result;
        } else {
            return 0;
        }
        num /= 10;
    }
    result
}

pub fn run()
{
    println!("{:?}", reverse(123))
}