use crate::engine::rpc::RpcManager;
use crate::engine::server::ServerManager;
use crate::i18n;
use crate::ui::theme;

pub fn ui(ui: &mut egui::Ui, server: &ServerManager, rpc: &RpcManager, lang: &i18n::Language) {
    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::SectionLaunchCommands, lang));

        render_command_card(
            ui,
            i18n::t(i18n::Key::LabelServerCommand, lang),
            server.launch_command(),
            lang,
        );

        ui.add_space(12.0);

        render_command_card(
            ui,
            i18n::t(i18n::Key::LabelRpcCommand, lang),
            rpc.launch_command(),
            lang,
        );
    });
}

fn render_command_card(
    ui: &mut egui::Ui,
    title: &str,
    command: Option<String>,
    lang: &i18n::Language,
) {
    theme::section_card(ui, title, |ui| {
        if let Some(command) = command {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(
                        i18n::Key::BtnCopyToClipboard,
                        lang,
                    )))
                    .clicked()
                {
                    ui.ctx().copy_text(command.clone());
                }
            });
            ui.add_space(8.0);
            theme::code_frame().show(ui, |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(command)
                            .monospace()
                            .color(egui::Color32::from_rgb(235, 237, 242)),
                    );
                });
            });
        } else {
            ui.colored_label(theme::TEXT_MUTED, i18n::t(i18n::Key::HintNoCommand, lang));
        }
    });
}
