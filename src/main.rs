mod terminal;
mod render;
mod editor;
mod document;
mod row;

pub use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use render::Window;
pub use document::Document;
pub use row::Row;

fn main()
{
    Editor::default().run();
}
