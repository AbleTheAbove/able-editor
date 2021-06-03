use fltk::{
    app, dialog,
    enums::Event,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    text, window,
};
use std::{error, panic, path};
mod editor;
mod menu;
#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Cut,
    Copy,
    Paste,
    Config,
}

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

pub struct App {
    app: app::App,
    saved: bool,
    filename: String,
    r: app::Receiver<Message>,
    buf: text::TextBuffer,
    editor: editor::Editor,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(0, 0, 0);

        app::foreground(255, 255, 255);

        app::set_frame_type(fltk::enums::FrameType::NoBox);

        let (s, r) = app::channel::<Message>();
        let mut buf = text::TextBuffer::default();
        buf.set_tab_distance(4);
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("AblEditor");
        let ico = include_bytes!("../assets/icon.png");
        let image = fltk::image::PngImage::from_data(ico).unwrap();
        main_win.set_icon(Some(image));

        let _menu = menu::Menu::new(&s);

        let mut frame = fltk::frame::Frame::default();
        frame.set_label_size(30);
        frame.set_label(&"hi");
        let mut editor = editor::Editor::new(buf.clone());
        editor.emit(s, Message::Changed);
        main_win.make_resizable(true);
        // only resize editor, not the menu bar
        main_win.resizable(&*editor);
        main_win.end();
        main_win.show();
        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                s.send(Message::Quit);
            }
        });
        let filename = if args.len() > 1 {
            let file = path::Path::new(&args[1]);
            assert!(
                file.exists() && file.is_file(),
                "An error occured while opening the file!"
            );
            buf.load_file(&args[1]).unwrap();
            args[1].clone()
        } else {
            String::new()
        };
        // Handle drag and drop
        let mut dnd = false;
        let mut released = false;

        editor.handle({
            let mut buf = buf.clone();
            move |_, ev| match ev {
                Event::DndEnter => {
                    dnd = true;
                    true
                }
                Event::DndDrag => true,
                Event::DndRelease => {
                    released = true;
                    true
                }
                Event::Paste => {
                    if dnd && released {
                        let path = app::event_text();
                        let path = std::path::Path::new(&path);
                        assert!(path.exists());
                        buf.load_file(&path).unwrap();
                        dnd = false;
                        released = false;
                        true
                    } else {
                        false
                    }
                }
                Event::DndLeave => {
                    dnd = false;
                    released = false;
                    true
                }
                _ => false,
            }
        });

        Self {
            app,
            saved: true,
            filename,
            r,
            buf,
            editor,
        }
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mut filename = self.filename.clone();
        if self.saved {
            if filename.is_empty() {
                let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
                dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
                dlg.show();
                filename = dlg.filename().to_string_lossy().to_string();
                if !filename.is_empty() {
                    self.buf.save_file(&filename).unwrap_or_else(|_| {
                        dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                    });
                    self.saved = true;
                }
            } else if path::Path::new(&filename).exists() {
                self.buf.save_file(&filename)?;
                self.saved = true;
            } else {
                dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
            }
        } else {
            let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
            dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if !filename.is_empty() {
                self.buf.save_file(&filename).unwrap_or_else(|_| {
                    dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!")
                });
                self.saved = true;
            }
        }
        Ok(())
    }
    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => self.saved = false,
                    New => {
                        if self.buf.text() != "" {
                            let x = dialog::choice(
                                center().0 - 200,
                                center().1 - 100,
                                "File unsaved, Do you wish to continue?",
                                "Yes",
                                "No!",
                                "",
                            );
                            if x == 0 {
                                self.buf.set_text("");
                            }
                        }
                    }
                    Open => {
                        let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                        dlg.set_option(dialog::FileDialogOptions::NoOptions);
                        dlg.show();
                        let filename = dlg.filename().to_string_lossy().to_string();
                        if !filename.is_empty() {
                            if path::Path::new(&filename).exists() {
                                self.buf.load_file(&filename).unwrap();
                                self.filename = filename;
                            } else {
                                dialog::alert(
                                    center().0 - 200,
                                    center().1 - 100,
                                    "File does not exist!",
                                )
                            }
                        }
                    }
                    Save | SaveAs => self.save_file().unwrap(),

                    Quit => {
                        if self.saved {
                            self.app.quit();
                        } else {
                            let x = dialog::choice(
                                center().0 - 200,
                                center().1 - 100,
                                "Would you like to save your work?",
                                "Yes",
                                "No",
                                "",
                            );

                            if x == 0 {
                                self.save_file().unwrap();
                                self.app.quit();
                            } else {
                                self.app.quit();
                            }
                        }
                    }
                    Cut => self.editor.cut(),
                    Copy => self.editor.copy(),
                    Paste => self.editor.paste(),
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            dialog::message(center().0 - 200, center().1 - 100, s);
        } else {
            dialog::message(center().0 - 200, center().1 - 100, &info.to_string());
        }
    }));
    let args: Vec<_> = std::env::args().collect();
    let mut app = App::new(args);
    app.launch();
}
