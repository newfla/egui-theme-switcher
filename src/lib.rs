use std::sync::RwLock;

use egui::{
    lerp, pos2, vec2, Align2, Color32, FontId, Response, Sense, ThemePreference, Ui, Widget,
    WidgetInfo, WidgetType,
};

static TOGGLE_STORAGE: RwLock<ThemePreference> = RwLock::new(ThemePreference::System);

/// Widget Size. Default to S
#[non_exhaustive]
#[derive(Default)]
pub enum Dimension {
    #[default]
    S,
    M,
    L,
    XL,
    Custom(f32),
}

impl Dimension {
    fn multiplier(&self) -> f32 {
        match self {
            Dimension::S => 1.,
            Dimension::M => 3.,
            Dimension::L => 5.,
            Dimension::XL => 7.,
            Dimension::Custom(mul) => *mul,
        }
    }
}

/// Paint the switcher to the [Ui] specifying the [Dimension]
pub fn theme_switcher_ui(ui: &mut Ui, dim: Dimension) -> Response {
    // Widget and font size
    let desired_size =
        ui.spacing().interact_size.y * vec2(5. * dim.multiplier(), 1. * dim.multiplier());
    let mut font = FontId::default();
    font.size *= dim.multiplier();

    // Allocating space
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    // Attach some meta-data to the response which can be used by screen readers
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::RadioButton,
            ui.is_enabled(),
            true,
            "theme switcher",
        )
    });

    let theme = TOGGLE_STORAGE
        .read()
        .map(|v| *v)
        .unwrap_or(ThemePreference::System);

    let how_on = match theme {
        ThemePreference::Dark => 1.,
        ThemePreference::Light => 0.,
        ThemePreference::System => 0.5,
    };

    ui.ctx().set_theme(theme);

    // Paint!
    if ui.is_rect_visible(rect) {
        egui_material_icons::initialize(ui.ctx());

        let rect_visuals = ui.style().interact_selectable(&response, false);
        let circle_visuals = ui.style().interact_selectable(&response, true);

        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(rect_visuals.expansion);
        let radius = 0.5 * rect.height();
        let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let system_x = rect.width() / 2. + rect.left();
        let system_position = pos2(system_x, rect.center().y);
        let light_position = pos2(rect.left() + 1.1 * radius, rect.center().y - radius / 10.);
        let dark_position = pos2(rect.right() - 1.1 * radius, rect.center().y - radius / 10.);
        let circle_position = pos2(circle_x, rect.center().y);

        // Paint background rect
        ui.painter()
            .rect(rect, radius, rect_visuals.bg_fill, rect_visuals.bg_stroke);

        // Paint icons
        let light_rect = ui.painter().text(
            light_position,
            Align2::CENTER_CENTER,
            egui_material_icons::icons::ICON_LIGHT_MODE,
            font.clone(),
            Color32::WHITE,
        );
        let system_rect = ui.painter().text(
            system_position,
            Align2::CENTER_CENTER,
            egui_material_icons::icons::ICON_SETTINGS,
            font.clone(),
            Color32::WHITE,
        );
        let dark_rect = ui.painter().text(
            dark_position,
            Align2::CENTER_CENTER,
            egui_material_icons::icons::ICON_DARK_MODE,
            font,
            Color32::WHITE,
        );

        // Check for clicks
        if response.clicked() {
            response.mark_changed(); // report back that the value changed
            let interaction = response.interact_pointer_pos().unwrap();
            if light_rect.contains(interaction) {
                *TOGGLE_STORAGE.write().unwrap() = ThemePreference::Light;
            } else if dark_rect.contains(interaction) {
                *TOGGLE_STORAGE.write().unwrap() = ThemePreference::Dark;
            } else if system_rect.contains(interaction) {
                *TOGGLE_STORAGE.write().unwrap() = ThemePreference::System;
            }
        }

        // Paint the circle, animating it from left to right with `how_on`:
        ui.painter().circle(
            circle_position,
            1. * radius,
            circle_visuals.bg_fill,
            circle_visuals.fg_stroke,
        );
    }
    response
}

/// Add the switcher to the [Ui] specifying a [Dimension]
pub fn theme_switcher_with_dimension(dim: Dimension) -> impl Widget {
    move |ui: &mut Ui| theme_switcher_ui(ui, dim)
}

/// Add the switcher to the [Ui] with [Dimension::S]
pub fn theme_switcher() -> impl Widget {
    move |ui: &mut Ui| theme_switcher_ui(ui, Dimension::default())
}
