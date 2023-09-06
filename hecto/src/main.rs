mod editor;
mod terminal;
mod document;
mod row;
mod highlighting;

use editor::Editor;

fn main() {
    Editor::init().run();
}