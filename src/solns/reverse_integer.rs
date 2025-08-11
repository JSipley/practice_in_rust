
pub fn reverse(x: i32) -> i32
{
    let mut result: i32 = 0;
    let mut num = x;
    while num != 0 {
        if let Some(new_result) = result.checked_mul(10)
                                             .and_then(|r| r.checked_add(num % 10)) 
        {
            result = new_result;
            num /= 10;
        } 
        else 
        {
            return 0;
        }
    }
    result
}

pub fn run()
{
    println!("{:?}", reverse(123))
}