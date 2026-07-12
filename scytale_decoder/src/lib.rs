//scytale_decoder level:5
pub fn scytale_decoder(s: String, letters_per_turn: usize) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let  chars :Vec<char>=s.chars().collect();
    let len = chars.len();

    let mut columns =len / letters_per_turn;
    if len % letters_per_turn>0{
        columns+=1;
    }
    let mut index =0;
    let mut grid =vec![vec!['\0';columns];letters_per_turn];

    // ملء المصفوفة عمودًا عمودًا
    for col in 0..columns{
        for row in 0..letters_per_turn{
            if index<len{
                grid[row][col]=chars[index];
                index+=1;
            }
        }
    }
   
    let mut result = String::new();
    for row in 0..letters_per_turn {
        for col in 0..columns {
            if grid[row][col] != '\0' {
                result.push(grid[row][col]);
            }
        }
    }

    Some(result)
}