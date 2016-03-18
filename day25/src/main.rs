fn main() {
    //let mut value: u32 = 27995004;
    fn find_code (mut max_row: u32, mut row: u32, mut col: u32, mut value: u64, to_row: u32, to_col: u32) -> u64 {
        loop {
            match (row == to_row) & (col == to_col) {
                true  => return value,
                false => {
                    value = value * 252533 % 33554393;
                    if row > 1 {
                        col +=1;
                        row -=1;
                    } else {
                        max_row += 1;
                        row = max_row;
                        col = 1;
                    };
                    //find_code(max_row, row, col, value, to_row, to_col)
                }
            
            }
        }
    
    };

    let code = find_code(11, 6, 6, 27995004, 3010, 3019);
    println!("{}", code);
}
