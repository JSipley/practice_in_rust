
pub fn generate(num_rows: i32) -> Vec<Vec<i32>>
{
    let mut triangle = Vec::with_capacity(num_rows as usize);
    for r in 0..num_rows as usize 
    {
        let mut row = Vec::with_capacity(r + 1);
        row.push(1);
        if r > 0 
        {
            let prev:&Vec<i32> = &triangle[r - 1];
            row.extend(prev.windows(2).map(|w| w[0] + w[1]));
            row.push(1);
        }
        triangle.push(row);
    }
    triangle
}

pub fn run()
{
    let num_rows = 24;
    let triangle = generate(num_rows);
    for row in triangle {
        println!("{:?}", row);
    }
}