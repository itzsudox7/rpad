// ui - handles all the editor gui
use eframe::egui;
use crate::file::FileState;
use crate::search::SearchState;
use crate::settings::Settings;
use crate::theme::Theme;

pub struct UiState;

// ui helpers for building the editor
impl UiState {
    // apply theme colors and font settings to the context
    pub fn apply_theme(ctx: &egui::Context, theme: Theme, font_size: f32) {
        let visuals = match theme {
            Theme::Dark => {
                let mut v = egui::Visuals::dark();
                v.override_text_color = Some(egui::Color32::from_rgb(212, 212, 212));
                v.extreme_bg_color = egui::Color32::from_rgb(24, 24, 27);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 34);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(38, 38, 44);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 58);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(82, 139, 255);
                v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(82, 139, 255, 60);
                v.window_fill = egui::Color32::from_rgb(24, 24, 27);
                v.panel_fill = egui::Color32::from_rgb(24, 24, 27);
                v.faint_bg_color = egui::Color32::from_rgb(30, 30, 34);
                v
            }
            Theme::Light => {
                let mut v = egui::Visuals::light();
                v.override_text_color = Some(egui::Color32::from_rgb(30, 30, 40));
                v.extreme_bg_color = egui::Color32::from_rgb(248, 248, 250);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(242, 242, 247);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(235, 235, 240);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(220, 228, 255);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(82, 139, 255);
                v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(82, 139, 255, 50);
                v.window_fill = egui::Color32::from_rgb(248, 248, 250);
                v.panel_fill = egui::Color32::from_rgb(248, 248, 250);
                v.faint_bg_color = egui::Color32::from_rgb(240, 240, 245);
                v
            }
            Theme::Monokai => {
                let mut v = egui::Visuals::dark();
                v.override_text_color = Some(egui::Color32::from_rgb(248, 248, 242));
                v.extreme_bg_color = egui::Color32::from_rgb(39, 40, 34);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(39, 40, 34);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(49, 50, 44);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(73, 74, 60);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(249, 38, 114);
                v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(249, 38, 114, 60);
                v.window_fill = egui::Color32::from_rgb(39, 40, 34);
                v.panel_fill = egui::Color32::from_rgb(39, 40, 34);
                v
            }
            Theme::Solarized => {
                let mut v = egui::Visuals::dark();
                v.override_text_color = Some(egui::Color32::from_rgb(131, 148, 150));
                v.extreme_bg_color = egui::Color32::from_rgb(0, 43, 54);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(0, 43, 54);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(7, 54, 66);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 73, 89);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(38, 139, 210);
                v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(38, 139, 210, 60);
                v.window_fill = egui::Color32::from_rgb(0, 43, 54);
                v.panel_fill = egui::Color32::from_rgb(0, 43, 54);
                v
            }
            Theme::Nord => {
                let mut v = egui::Visuals::dark();
                v.override_text_color = Some(egui::Color32::from_rgb(236, 239, 244));
                v.extreme_bg_color = egui::Color32::from_rgb(46, 52, 64);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(46, 52, 64);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(59, 66, 82);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(67, 76, 94);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(136, 192, 208);
                v.selection.bg_fill = egui::Color32::from_rgba_premultiplied(136, 192, 208, 60);
                v.window_fill = egui::Color32::from_rgb(46, 52, 64);
                v.panel_fill = egui::Color32::from_rgb(46, 52, 64);
                v
            }
        };
        ctx.set_visuals(visuals);

        let mut style = (*ctx.global_style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(font_size, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(11.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        );
ctx.set_global_style(style);
    }

    // shows the top bar with file buttons, menu, and theme selector
    pub fn topbar(
        ui: &mut egui::Ui,
        file: &mut FileState,
        theme: &mut Theme,
        settings: &mut Settings,
        show_search: &mut bool,
        show_minimap: &mut bool,
    ) {
        let bg = theme.topbar_bg();
        let accent = theme.accent_color();
        let text_color = theme.text_color();
        let muted = theme.muted_color();

        egui::Panel::top("topbar")
            .frame(egui::Frame::NONE.fill(bg).inner_margin(egui::Margin::symmetric(12, 0)))
            .show_inside(ui, |ui| {
                ui.set_height(38.0);
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;

                    ui.label(egui::RichText::new("Rpad").color(accent).size(15.0).strong());

                    ui.add_space(16.0);

                    for (label, shortcut) in [
                        ("New", "Ctrl+N"),
                        ("Open", "Ctrl+O"),
                        ("Save", "Ctrl+S"),
                    ] {
                        let btn = egui::Button::new(
                            egui::RichText::new(label).color(text_color).size(13.0),
                        )
                        .frame(false)
                        .min_size(egui::vec2(0.0, 30.0));

                        let resp = ui.add(btn).on_hover_text(shortcut);

                        if resp.clicked() {
                            match label {
                                "New" => file.new_file(),
                                "Open" => {
                                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                                        let status = file.open(path);
                                        let _ = status;
                                    }
                                }
                                "Save" => { let _ = file.save(); }
                                _ => {}
                            }
                        }

                        ui.add_space(6.0);
                    }

                    let sa_btn = egui::Button::new(
                        egui::RichText::new("Save As").color(muted).size(13.0),
                    )
                    .frame(false);
                    if ui.add(sa_btn).on_hover_text("Ctrl+Shift+S").clicked() {
                        let _ = file.save_as();
                    }

                    ui.add_space(8.0);
                    ui.add(egui::Separator::default().vertical().spacing(8.0));
                    ui.add_space(8.0);

                    let search_color = if *show_search { accent } else { muted };
                    let search_btn = egui::Button::new(
                        egui::RichText::new("Find").color(search_color).size(13.0),
                    )
                    .frame(false);
                    if ui.add(search_btn).on_hover_text("Ctrl+F").clicked() {
                        *show_search = !*show_search;
                    }

                    ui.add_space(8.0);

                    ui.menu_button(
                        egui::RichText::new("View").color(muted).size(13.0),
                        |ui| {
                            ui.set_min_width(220.0);
                            ui.spacing_mut().item_spacing.y = 6.0;
                            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(
                                accent.r(),
                                accent.g(),
                                accent.b(),
                                36,
                            );

                            let line_lbl = if settings.show_line_numbers { "* Line Numbers" } else { "  Line Numbers" };
                            if ui.selectable_label(settings.show_line_numbers, egui::RichText::new(line_lbl).size(12.5)).clicked() {
                                settings.show_line_numbers = !settings.show_line_numbers;
                            }

                            let wrap_lbl = if settings.word_wrap { "* Word Wrap" } else { "  Word Wrap" };
                            if ui.selectable_label(settings.word_wrap, egui::RichText::new(wrap_lbl).size(12.5)).clicked() {
                                settings.word_wrap = !settings.word_wrap;
                            }

                            let minimap_lbl = if *show_minimap { "* Minimap" } else { "  Minimap" };
                            if ui.selectable_label(*show_minimap, egui::RichText::new(minimap_lbl).size(12.5)).clicked() {
                                *show_minimap = !*show_minimap;
                            }

                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Font").color(muted).size(11.0));
                                ui.label(egui::RichText::new("Ctrl +/-").color(text_color).size(11.0));
                            });
                        },
                    );

                    ui.add_space(8.0);

                    ui.menu_button(
                        egui::RichText::new("* Theme").color(accent).size(13.0),
                        |ui| {
                            ui.set_min_width(150.0);
                            for t in Theme::all() {
                                let selected = *theme == t;
                                let (dot, color) = t.dot();
                                let lbl = egui::RichText::new(format!("{} {}", dot, t.label()))
                                    .color(if selected { color } else { text_color });
                                if ui.selectable_label(selected, lbl).clicked() {
                                    *theme = t;
                                    ui.close();
                                }
                            }
                        },
                    );

                    if !file.recent_files.is_empty() {
                        ui.add_space(8.0);
                        ui.menu_button(
                            egui::RichText::new("Recent").color(muted).size(13.0),
                            |ui| {
                                let files = file.recent_files.clone();
                                for path in &files {
                                    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                                    if ui.button(&name).on_hover_text(path.display().to_string()).clicked() {
                                        let p = path.clone();
                                        let _ = file.open(p);
                                        ui.close();
                                    }
                                }
                            },
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);

                        if file.is_dirty() {
                            ui.label(egui::RichText::new("*").color(egui::Color32::from_rgb(255, 180, 50)).size(10.0))
                                .on_hover_text("Unsaved changes");
                        }

                        ui.label(egui::RichText::new(file.file_name()).color(text_color).size(12.0));

                        match &file.status {
                            crate::file::FileStatus::Saved => {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("Saved").color(egui::Color32::from_rgb(80, 200, 120)).size(12.0));
                            }
                            crate::file::FileStatus::Opened => {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("Opened").color(egui::Color32::from_rgb(80, 200, 120)).size(12.0));
                            }
                            crate::file::FileStatus::Error(e) => {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new(format!("x {}", e)).color(egui::Color32::from_rgb(255, 80, 80)).size(12.0));
                            }
                            crate::file::FileStatus::None => {}
                        }
                    });
                });
            });
    }

    // shows the bottom status bar with cursor position and text stats
    pub fn statusbar(
        ui: &mut egui::Ui,
        theme: Theme,
        text: &str,
        cursor_line: usize,
        cursor_col: usize,
        tab_size: usize,
    ) {
        let bg = theme.topbar_bg();
        let muted = theme.muted_color();
        let (dot, color) = theme.dot();

        egui::Panel::bottom("statusbar")
            .frame(egui::Frame::NONE.fill(bg).inner_margin(egui::Margin::symmetric(14, 0)))
            .show_inside(ui, |ui| {
                ui.set_height(26.0);
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    let stat = |label: &str| egui::RichText::new(label).color(muted).size(11.0);

                    ui.label(stat(&format!("Ln {}, Col {}", cursor_line, cursor_col)));
                    ui.add_space(14.0);
                    ui.add(egui::Separator::default().vertical().spacing(4.0));
                    ui.add_space(14.0);
                    ui.label(stat(&format!("{} chars", text.len())));
                    ui.add_space(14.0);
                    ui.add(egui::Separator::default().vertical().spacing(4.0));
                    ui.add_space(14.0);
                    ui.label(stat(&format!("{} words", text.split_whitespace().count())));
                    ui.add_space(14.0);
                    ui.add(egui::Separator::default().vertical().spacing(4.0));
                    ui.add_space(14.0);
                    ui.label(stat(&format!("{} lines", text.lines().count().max(1))));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(stat("UTF-8"));
                        ui.add_space(14.0);
                        ui.add(egui::Separator::default().vertical().spacing(4.0));
                        ui.add_space(14.0);
                        let tab_label = format!("Spaces: {}", tab_size);
                        ui.label(stat(&tab_label));
                        ui.add_space(14.0);
                        ui.add(egui::Separator::default().vertical().spacing(4.0));
                        ui.add_space(14.0);
                        ui.label(egui::RichText::new(format!("{} {}", dot, theme.label())).color(color).size(11.0));
                    });
                });
            });
    }

    // shows the search and replace bar
    pub fn searchbar(
        ui: &mut egui::Ui,
        theme: Theme,
        search: &mut SearchState,
        text: &mut String,
        show_search: &mut bool,
    ) {
        let bg = theme.topbar_bg();
        let accent = theme.accent_color();
        let muted = theme.muted_color();

        egui::Panel::top("searchbar")
            .frame(egui::Frame::NONE.fill(bg).inner_margin(egui::Margin::symmetric(14, 6)))
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;

                    ui.label(egui::RichText::new("Find:").color(muted).size(12.0));

                    let search_resp = ui.add(
                        egui::TextEdit::singleline(&mut search.query)
                            .desired_width(200.0)
                            .hint_text("Search...")
                            .font(egui::TextStyle::Body),
                    );

                    if search_resp.changed() {
                        search.update_matches(text);
                    }

                    if !search.query.is_empty() {
                        let total = search.matches.len();
                        let current = if total > 0 { search.current_match + 1 } else { 0 };
                        ui.label(
                            egui::RichText::new(format!("{}/{}", current, total))
                                .color(if total > 0 { accent } else { egui::Color32::from_rgb(255, 100, 100) })
                                .size(11.0),
                        );
                    }

                    if ui.button("^").clicked() {
                        search.prev_match();
                    }
                    if ui.button("v").clicked() {
                        search.next_match();
                    }

                    ui.add_space(8.0);
                    ui.add(egui::Separator::default().vertical().spacing(4.0));
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Replace:").color(muted).size(12.0));
                    ui.add(
                        egui::TextEdit::singleline(&mut search.replace)
                            .desired_width(200.0)
                            .hint_text("Replace with...")
                            .font(egui::TextStyle::Body),
                    );

                    if ui.button("Replace").clicked() {
                        if let Some(&pos) = search.matches.get(search.current_match) {
                            let q = search.query.clone();
                            let r = search.replace.clone();
                            let end = pos + q.len();
                            if end <= text.len() {
                                text.replace_range(pos..end, &r);
                                search.update_matches(text);
                            }
                        }
                    }

                    if ui.button("Replace All").clicked() {
                        let q = search.query.clone();
                        let r = search.replace.clone();
                        if !q.is_empty() {
                            *text = text.replace(&q, &r).to_string();
                            search.update_matches(text);
                        }
                    }

                    ui.add_space(8.0);
                    ui.checkbox(&mut search.case_sensitive, egui::RichText::new("Aa").size(12.0).color(muted))
                        .on_hover_text("Case sensitive");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("x").color(muted).size(13.0)).clicked() {
                            *show_search = false;
                        }
                    });
                });
            });
    }

    // shows the main text editor area with line numbers and minimap
    pub fn editor(
        ui: &mut egui::Ui,
        theme: Theme,
        settings: &Settings,
        text: &mut String,
        show_minimap: bool,
        font_size: f32,
        cursor_pos: &mut (usize, usize),
    ) {
        let bg = theme.bg_color();
        let text_color = theme.text_color();
        let muted = theme.muted_color();

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(bg))
            .show_inside(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    let line_count = text.split('\n').count().max(1);
                    let gutter_w = 52.0;

                    let available = ui.available_size();
                    let editor_width = if show_minimap {
                        available.x - 100.0
                    } else {
                        available.x
                    };

                    if settings.show_line_numbers {
                        egui::Frame::NONE
                            .fill(theme.topbar_bg())
                            .inner_margin(egui::Margin { left: 0, right: 8, top: 8, bottom: 8 })
                            .show(ui, |ui| {
                                ui.set_width(gutter_w);
                                ui.set_min_height(available.y - 16.0);

                                let numbers: String = (1..=line_count)
                                    .map(|n| format!("{:>3}\n", n))
                                    .collect();

                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(gutter_w, available.y - 16.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().text(
                                    rect.left_top() + egui::vec2(0.0, -7.6),
                                    egui::Align2::LEFT_TOP,
                                    numbers,
                                    egui::FontId::new(font_size, egui::FontFamily::Monospace),
                                    muted,
                                );
                            });
                    }

                    let old_text = text.clone();

                    let mut editor = egui::TextEdit::multiline(text)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(editor_width - gutter_w - 28.0)
                        .lock_focus(true)
                        .frame(egui::Frame::NONE)
                        .text_color(text_color);

                    if !settings.word_wrap {
                        editor = editor.code_editor();
                    }

                    let resp = ui.add_sized(
                        [editor_width - gutter_w - 28.0, available.y - 16.0],
                        editor,
                    );

                    if *text != old_text {
                        if let Some(cursor) = egui::TextEdit::load_state(ui.ctx(), resp.id).and_then(|s| s.cursor.char_range()) {
                            let pos = cursor.primary.index;
                            let before = &text[..pos.min(text.len())];
                            cursor_pos.0 = before.chars().filter(|&c| c == '\n').count() + 1;
                            cursor_pos.1 = before.rfind('\n').map(|i| pos - i).unwrap_or(pos + 1);
                        }
                    }

                    if show_minimap {
                        egui::Frame::NONE
                            .fill(theme.topbar_bg())
                            .inner_margin(egui::Margin::same(4))
                            .show(ui, |ui| {
                                ui.set_width(96.0);
                                ui.set_height(available.y);

                                let preview: String = text
                                    .lines()
                                    .take(100)
                                    .map(|l| format!("{}\n", &l[..l.len().min(30)]))
                                    .collect();

                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(preview).monospace().size(3.0).color(muted),
                                    ).wrap(),
                                );
                            });
                    }
                });
            });
    }
}