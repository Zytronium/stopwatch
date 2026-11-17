use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .background((24, 24, 24))
        .child(label().text("Hello, World!"))
        .color((255, 255, 255))
        .center()
}
