use eframe::{egui::{self, Context, UserAttentionType, ViewportCommand}, App, Frame};

pub struct Dialogue<'a> {
    pub text: &'a mut String,
}

impl<'a> Dialogue<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
        }
    }
}

impl App for Dialogue<'_> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.send_viewport_cmd(ViewportCommand::RequestUserAttention(UserAttentionType::Informational));
            
            let textbox = ui.text_edit_multiline(self.text);
            ui.memory_mut(|memory| memory.request_focus(textbox.id));

            if self.text.ends_with(" ") || self.text.ends_with("\n") {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        });
    }
}
