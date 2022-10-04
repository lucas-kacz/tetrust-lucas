
#[derive(PartialEq)]

pub enum TetriminoType {O, I, T, L, J, S, Z}

pub struct Tetrimino{
    pub style : TetriminoType,
    pub color : [f32;4],
    pub shape : [[u8;4];4]
}

impl Tetrimino{

    pub const fn new(style : TetriminoType) -> Self
    {
        match style
        {
            TetriminoType::O => Tetrimino
            {
                style: TetriminoType :: O,
                color: [0.0,1.0,0.0,1.0],
                shape: [[0,0,0,0],
                        [0,1,1,0],
                        [0,1,1,0],
                        [0,0,0,0]]
            },

            TetriminoType::I => Tetrimino
            {
                style: TetriminoType :: I,
                color: [1.0,1.0,1.0,1.0],
                shape: [[0,0,0,0],
                        [1,1,1,1],
                        [0,0,0,0],
                        [0,0,0,0]]
            },

            TetriminoType::T => Tetrimino
            {
                style: TetriminoType :: T,
                color: [1.0,1.0,0.0,1.0],
                shape: [[0,1,0,0],
                        [1,1,1,0],
                        [0,0,0,0],
                        [0,0,0,0]]
            },

            TetriminoType::L => Tetrimino
            {
                style: TetriminoType :: L,
                color: [0.0,1.0,1.0,1.0],
                shape: [[0,0,1,0],
                        [1,1,1,0],
                        [0,0,0,0],
                        [0,0,0,0]]
            },

            TetriminoType::J => Tetrimino
            {
                style: TetriminoType :: J,
                color: [0.0,0.0,1.0,1.0],
                shape: [[1,0,0,0],
                        [1,1,1,0],
                        [0,0,0,0],
                        [0,0,0,0]]
            },

            TetriminoType::S => Tetrimino
            {
                style: TetriminoType :: S,
                color: [1.0,0.0,1.0,1.0],
                shape: [[0,1,1,0],
                        [1,1,0,0],
                        [0,0,0,0],
                        [0,0,0,0]]
            },

            TetriminoType::Z => Tetrimino
            {
                style: TetriminoType :: Z,
                color: [1.0,0.0,0.0,1.0],
                shape: [[1,1,0,0],
                        [0,1,1,0],
                        [0,0,0,0],
                        [0,0,0,0]]
            },
        }
    }

}