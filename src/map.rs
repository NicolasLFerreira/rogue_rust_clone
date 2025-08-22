pub struct Map {
    pub width: u16,
    pub height: u16,
    pub matrix: Vec<Vec<char>>,
}

impl Map {
    pub fn create_new(width: u16, height: u16) -> Self {
        let mut temp_matrix: Vec<Vec<char>> = Vec::new();

        for i in 0..width {
            let mut temp_row: Vec<char> = Vec::new();

            for j in 0..height {
                temp_row.push('.');
            }

            temp_matrix.push(temp_row);
        }

        Map {
            height,
            width,
            matrix: temp_matrix,
        }
    }
}
