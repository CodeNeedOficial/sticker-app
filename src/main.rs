#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storage;

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use storage::{Sticker, Store, parse_tags, render_all_md, render_sticker_md};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode {
    View,
    Edit,
    New,
}

struct StickerApp {
    store: Store,
    selected_id: Option<u32>,
    mode: Mode,
    draft_title: String,
    draft_content: String,
    draft_tags: String,
    filter: String,
    md_cache: CommonMarkCache,
    status: String,
    confirm_delete: Option<u32>,
}

impl StickerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let store = storage::load().unwrap_or_else(|e| {
            eprintln!("aviso: falha ao carregar stickers: {e}");
            Store::empty()
        });
        Self {
            store,
            selected_id: None,
            mode: Mode::View,
            draft_title: String::new(),
            draft_content: String::new(),
            draft_tags: String::new(),
            filter: String::new(),
            md_cache: CommonMarkCache::default(),
            status: String::new(),
            confirm_delete: None,
        }
    }

    fn select(&mut self, id: u32) {
        if let Some(s) = self.store.get(id) {
            self.selected_id = Some(id);
            self.draft_title = s.title.clone();
            self.draft_content = s.content.clone();
            self.draft_tags = s.tags.join(", ");
            self.mode = Mode::View;
            self.confirm_delete = None;
        }
    }

    fn start_new(&mut self) {
        self.selected_id = None;
        self.draft_title.clear();
        self.draft_content.clear();
        self.draft_tags.clear();
        self.mode = Mode::New;
        self.confirm_delete = None;
    }

    fn start_edit(&mut self) {
        if self.selected_id.is_some() {
            self.mode = Mode::Edit;
        }
    }

    fn cancel_edit(&mut self) {
        match self.selected_id {
            Some(id) => self.select(id),
            None => {
                self.draft_title.clear();
                self.draft_content.clear();
                self.draft_tags.clear();
                self.mode = Mode::View;
            }
        }
    }

    fn save_draft(&mut self) {
        let title = self.draft_title.trim().to_string();
        if title.is_empty() {
            self.status = "titulo nao pode ser vazio".to_string();
            return;
        }
        let tags = parse_tags(&self.draft_tags);
        let content = self.draft_content.clone();

        match self.mode {
            Mode::New => {
                let id = {
                    let s = self.store.add(title, content, tags);
                    s.id
                };
                self.persist_with_msg(&format!("criado sticker #{id}"));
                self.selected_id = Some(id);
                self.mode = Mode::View;
            }
            Mode::Edit => {
                if let Some(id) = self.selected_id {
                    if self.store.update(id, title, content, tags) {
                        self.persist_with_msg(&format!("salvo sticker #{id}"));
                        self.mode = Mode::View;
                    }
                }
            }
            Mode::View => {}
        }
    }

    fn delete_selected(&mut self) {
        if let Some(id) = self.selected_id {
            if self.store.remove(id) {
                self.persist_with_msg(&format!("removido sticker #{id}"));
            }
            self.selected_id = None;
            self.draft_title.clear();
            self.draft_content.clear();
            self.draft_tags.clear();
            self.mode = Mode::View;
            self.confirm_delete = None;
        }
    }

    fn persist_with_msg(&mut self, msg: &str) {
        match storage::save(&self.store) {
            Ok(()) => self.status = msg.to_string(),
            Err(e) => self.status = format!("erro ao salvar: {e}"),
        }
    }

    fn export_all(&mut self) {
        let md = render_all_md(&self.store);
        let path = match storage::store_path() {
            Ok(p) => p.with_file_name("stickers.md"),
            Err(e) => {
                self.status = format!("erro: {e}");
                return;
            }
        };
        match std::fs::write(&path, md) {
            Ok(()) => self.status = format!("exportado para {}", path.display()),
            Err(e) => self.status = format!("erro ao exportar: {e}"),
        }
    }

    fn matches_filter(&self, s: &Sticker) -> bool {
        if self.filter.trim().is_empty() {
            return true;
        }
        let q = self.filter.to_lowercase();
        s.title.to_lowercase().contains(&q)
            || s.content.to_lowercase().contains(&q)
            || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
    }
}

impl eframe::App for StickerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("➕ Novo").clicked() {
                    self.start_new();
                }
                ui.separator();
                if ui.button("📤 Exportar .md").clicked() {
                    self.export_all();
                }
                ui.separator();
                ui.label("filtro:");
                ui.text_edit_singleline(&mut self.filter);
            });
        });

        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(if self.status.is_empty() {
                    format!("{} stickers", self.store.stickers.len())
                } else {
                    self.status.clone()
                });
            });
        });

        egui::Panel::left("list")
            .resizable(true)
            .default_size(240.0)
            .show_inside(ui, |ui| {
                ui.heading("Stickers");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let ids: Vec<u32> = self
                        .store
                        .stickers
                        .iter()
                        .filter(|s| self.matches_filter(s))
                        .map(|s| s.id)
                        .collect();

                    if ids.is_empty() {
                        ui.label(if self.filter.trim().is_empty() {
                            "_sem stickers ainda_"
                        } else {
                            "_nenhum resultado_"
                        });
                    }

                    for id in ids {
                        let (title, tags) = {
                            let s = self.store.get(id).unwrap();
                            (s.title.clone(), s.tags.clone())
                        };
                        let selected = self.selected_id == Some(id);
                        let label = if tags.is_empty() {
                            format!("#{id}  {title}")
                        } else {
                            format!("#{id}  {title}  ·  {}", tags.join(", "))
                        };
                        if ui.selectable_label(selected, label).clicked() {
                            self.select(id);
                        }
                    }
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| match self.mode {
            Mode::View => self.render_view(ui),
            Mode::Edit | Mode::New => self.render_editor(ui),
        });
    }
}

impl StickerApp {
    fn render_view(&mut self, ui: &mut egui::Ui) {
        let Some(id) = self.selected_id else {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading("Selecione um sticker à esquerda");
                ui.label("ou clique em ➕ Novo para criar um.");
            });
            return;
        };
        let s = match self.store.get(id) {
            Some(s) => s.clone(),
            None => return,
        };

        ui.horizontal(|ui| {
            if ui.button("✏ Editar").clicked() {
                self.start_edit();
            }
            if ui.button("🗑 Remover").clicked() {
                self.confirm_delete = Some(id);
            }
        });

        if self.confirm_delete == Some(id) {
            ui.horizontal(|ui| {
                ui.colored_label(egui::Color32::LIGHT_RED, "remover este sticker?");
                if ui.button("sim").clicked() {
                    self.delete_selected();
                }
                if ui.button("nao").clicked() {
                    self.confirm_delete = None;
                }
            });
        }

        ui.separator();
        let md = render_sticker_md(&s);
        egui::ScrollArea::vertical().show(ui, |ui| {
            CommonMarkViewer::new().show(ui, &mut self.md_cache, &md);
        });
    }

    fn render_editor(&mut self, ui: &mut egui::Ui) {
        let is_new = matches!(self.mode, Mode::New);
        ui.heading(if is_new {
            "Novo sticker"
        } else {
            "Editando sticker"
        });
        ui.separator();

        ui.label("Título");
        ui.text_edit_singleline(&mut self.draft_title);

        ui.add_space(6.0);
        ui.label("Tags (separadas por vírgula)");
        ui.text_edit_singleline(&mut self.draft_tags);

        ui.add_space(6.0);
        ui.label("Conteúdo (Markdown)");

        let available = ui.available_size();
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(available.x / 2.0 - 8.0, available.y - 60.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("editor_scroll")
                        .show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::multiline(&mut self.draft_content)
                                    .code_editor()
                                    .desired_rows(20),
                            );
                        });
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(available.x / 2.0 - 8.0, available.y - 60.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("preview_scroll")
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Preview").weak());
                            CommonMarkViewer::new().show(
                                ui,
                                &mut self.md_cache,
                                &self.draft_content,
                            );
                        });
                },
            );
        });

        ui.separator();
        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("💾 Salvar").min_size(egui::vec2(100.0, 0.0)))
                .clicked()
            {
                self.save_draft();
            }
            if ui.button("Cancelar").clicked() {
                self.cancel_edit();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([500.0, 350.0])
            .with_title("Sticker"),
        ..Default::default()
    };
    eframe::run_native(
        "Sticker",
        native_options,
        Box::new(|cc| Ok(Box::new(StickerApp::new(cc)))),
    )
}
