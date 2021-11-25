use orbtk::prelude::*


fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("Lamegui - min")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::new().text("Lame").build(ctx))
                build(ctx)
        })
        .run();
}
