use fltk::{
    enums::{CallbackTrigger, Color, Font},
    prelude::{DisplayExt, WidgetBase, WidgetExt},
    text,
};
use std::ops::{Deref, DerefMut};

pub struct Editor {
    editor: text::TextEditor,
}

impl Editor {
    pub fn new(buf: text::TextBuffer) -> Self {
        let mut editor = text::TextEditor::new(5, 35, 790, 560, "");
        editor.set_buffer(Some(buf));

        editor.set_text_color(Color::White);
        editor.set_color(Color::Black);
        editor.set_cursor_style(fltk::text::Cursor::Simple);

        #[cfg(target_os = "macos")]
        editor.resize(5, 5, 790, 590);

        editor.set_scrollbar_size(15);
        editor.set_text_font(Font::Screen);
        editor.set_linenumber_width(32);
        editor.set_linenumber_fgcolor(Color::White);
        editor.set_linenumber_bgcolor(Color::Black);
        editor.set_trigger(CallbackTrigger::Changed);
        editor.set_cursor_color(Color::White);
        Self { editor }
    }
}

impl Deref for Editor {
    type Target = text::TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for Editor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}
