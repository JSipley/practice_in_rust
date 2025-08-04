
pub fn generate(num_rows: i32) -> Vec<Vec<i32>>
{
    let mut result: Vec<Vec<i32>> = Vec::new();
    if num_rows == 0 {
        return result;
    }
    result.push(vec![1]);
    for r in 1..num_rows as usize
    {
        let mut row: Vec<i32> = Vec::new();
        row.push(1);
        for c in 1..r
        {
            let value = result[r - 1][c - 1] + result[r - 1][c];
            row.push(value);
        }
        row.push(1);
        result.push(row);
    }
    return result;
}

pub fn run()
{
    let num_rows = 24;
    let triangle = generate(num_rows);
    for row in triangle {
        println!("{:?}", row);
    }
}


// C# impl as reference

// static IList<IList<int>> Generate(int numRows)
// {
//     IList<IList<int>> result = new List<IList<int>>();
//     List<int> firstRow = new List<int> { 1 };
//     result.Add(firstRow);
//     for (int r = 1; r < numRows; r++)
//     {
//         IList<int> row = new List<int>();
//         row.Add(1); // First element is always 1
//         for (int c = 1; c < r; c++)
//         {
//             // Each triangle element is the sum of the two elements above it
//             int value = ((IList<int>)result[r - 1])[c - 1] + ((IList<int>)result[r - 1])[c];
//             row.Add(value);
//         }
//         row.Add(1); // Last element is always 1
//         result.Add(row);
//     }
//     return result;
// }