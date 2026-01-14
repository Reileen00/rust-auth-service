use tower_governor::GovernorLayer;

pub fn layer() -> GovernorLayer {
    GovernorLayer::default()
}
