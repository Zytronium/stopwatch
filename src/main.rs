use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new()
            .with_window(
                WindowConfig::new(app)
                    .with_title("Stopwatch")
                    .with_app_id("dev.zytronium.stopwatch")
                    .with_size(480.0, 720.0)
                    .with_background((15, 23, 33))
            )
    )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .child(label().text("Hello, World!"))
        .color((255, 255, 255))
        .center()
}
