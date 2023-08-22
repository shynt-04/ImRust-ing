mod editor;
mod terminal;

use editor::Editor;

fn main() {
    Editor::init().run();
}