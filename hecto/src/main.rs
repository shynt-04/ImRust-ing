mod editor;
mod terminal;
mod document;
mod row;


use editor::Editor;

fn main() {
    Editor::init().run();
}