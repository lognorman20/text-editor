mod editor;

use editor::Editor;

fn main() {
    let mut editor: Editor = Editor::default();
    editor.run();
}
