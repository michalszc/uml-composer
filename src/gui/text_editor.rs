use druid::widget::TextBox;
use druid::{
    Env, Event, EventCtx,
    Widget, WidgetPod, Selector,
    LifeCycleCtx, LifeCycle,
    UpdateCtx, PaintCtx, LayoutCtx,
    BoxConstraints, Size
};

use super::tabs::DynamicTabsData;

const INSERT_TAB: Selector = Selector::new("text_editor.insert_tab");

pub struct TextEditor {
    text: String,
    text_box: WidgetPod<String, TextBox<String>>,
    cursor_pos: usize,
    line: usize
}

impl TextEditor {
    pub fn new(text: String) -> Self {
        TextEditor {
            text,
            text_box: WidgetPod::new(TextBox::multiline()),
            cursor_pos: 0,
            line: 0
        }
    }
}

impl Widget<DynamicTabsData> for TextEditor {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut DynamicTabsData, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(INSERT_TAB) => {
                // Insert a tab character when the INSERT_TAB command is received
                let lines: Vec<&str> = self.text.split("\n").collect();
                let mut index = self.cursor_pos.clone();
                for i in 0..self.line.clone() {
                    index += lines.get(i).unwrap_or(&"").len() + 1;
                }
                self.text.insert(index, '\t');
                ctx.request_update();
                ctx.set_handled();
            }
            Event::KeyDown(key_event) => {
                match key_event {
                    k if k.key == druid::keyboard_types::Key::Tab => {
                        // Intercept the Tab key and send the INSERT_TAB command
                        ctx.submit_command(INSERT_TAB);
                        ctx.set_handled();
                    }
                    k if k.key == druid::keyboard_types::Key::Backspace => {
                        // Move the cursor left
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        } else {
                            if self.line > 0 {
                                self.line -= 1;
                                let lines: Vec<&str> = self.text.split("\n").collect();
                                self.cursor_pos = lines.get(self.line).unwrap_or(&"").len();
                            }
                        }
                        ctx.request_update();
                    }
                    k if k.key == druid::keyboard_types::Key::ArrowLeft => {
                        // Move the cursor left
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        } else {
                            if self.line > 0 {
                                self.line -= 1;
                                let lines: Vec<&str> = self.text.split("\n").collect();
                                self.cursor_pos = lines.get(self.line).unwrap_or(&"").len();
                            }
                        }
                        ctx.request_update();
                    }
                    k if k.key == druid::keyboard_types::Key::ArrowRight => {
                        // Move the cursor right
                        let lines: Vec<&str> = self.text.split("\n").collect();
                        if self.cursor_pos < lines.get(self.line).unwrap_or(&"").replace("\n", "").len() {
                            self.cursor_pos += 1;
                        } else {
                            if lines.len() - 1 > self.line {
                                self.line += 1;
                                self.cursor_pos = 0;
                            }
                        }
                        ctx.request_update();
                    }
                    k if k.key == druid::keyboard_types::Key::ArrowDown => {
                        // Move the cursor down
                        self.line += 1;
                        self.cursor_pos = 0;
                        ctx.request_update();
                        // ctx.set_handled();
                    }
                    k if k.key == druid::keyboard_types::Key::ArrowUp => {
                        // Move the cursor up
                        if self.line > 0 {
                            self.line -= 1;
                        }
                        self.cursor_pos = 0;
                        ctx.request_update();
                    }
                    k if k.key == druid::keyboard_types::Key::Enter => {
                        // Move the cursor down
                        self.line += 1;
                        self.cursor_pos = 0;
                        ctx.request_update();
                    }
                    _ => {
                        self.cursor_pos += 1;

                        self.text_box.event(ctx, event, &mut self.text, _env);
                    }
                }
            }
            _ => {
                self.text_box.event(ctx, event, &mut self.text, _env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &DynamicTabsData, _env: &Env) {
        self.text_box.lifecycle(ctx, event, &self.text, _env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &DynamicTabsData, _data: &DynamicTabsData, _env: &Env) {
        self.text_box.update(ctx, &self.text, _env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &DynamicTabsData, env: &Env) -> Size {
        self.text_box.layout(ctx, bc, &self.text, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &DynamicTabsData, env: &Env) {
        self.text_box.paint(ctx, &self.text, env);
    }
}
