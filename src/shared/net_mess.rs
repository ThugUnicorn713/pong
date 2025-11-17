pub enum NetMessage {
    Snapshot(GameState),
    Paddle(PaddleEvent),
}