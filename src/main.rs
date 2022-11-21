mod terminal;
mod editor;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
