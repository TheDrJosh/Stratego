
struct GameState {
    board: [Option<Piece>; 10*10-2*2*3]
}

struct Piece {
    owner: Side,
    piece_type: PieceType,
}

enum Side {
    Red,
    Blue,
}

enum PieceType {
    Bomb,
    Marshal,
    General,
    Colonel,
    Major,
    Captain,
    Lieutenant,
    Sergeant,
    Miner,
    Scout,
    Spy,
    Flag,
}