pub struct Squares {
    pub squares: Vec<u16>,
    pub winning_positions: Vec<u16>
}



impl Squares {
    pub fn new() -> Self {

        let squares = vec![
            1 << 0,
            1 << 1,
            1 << 2,
            1 << 3,
            1 << 4,
            1 << 5,
            1 << 6,
            1 << 7,
            1 << 8
        ];

        Self {
            squares: squares.to_owned(),
            winning_positions: vec![
                squares[0] + squares[1] + squares[2],
                squares[3] + squares[4] + squares[5],
                squares[6] + squares[7] + squares[8],
                
                squares[0] + squares[3] + squares[6],
                squares[1] + squares[4] + squares[7],
                squares[2] + squares[5] + squares[8],
                
                squares[0] + squares[4] + squares[8],
                squares[6] + squares[4] + squares[2]
            ]
        }
    }
}