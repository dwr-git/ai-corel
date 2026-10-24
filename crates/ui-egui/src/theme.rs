//! Design tokens (four UI brightness levels), fonts and egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    Dark,
    MediumDark,
    MediumLight,
    /// AI-Corel opens light (with the Neumorphic scheme: the soft light-blue look).
    #[default]
    Light,
}

impl Brightness {
    pub const ALL: [Brightness; 4] = [Brightness::Dark, Brightness::MediumDark, Brightness::MediumLight, Brightness::Light];
    pub fn label(self) -> &'static str {
        match self {
            Brightness::Dark => "Dark",
            Brightness::MediumDark => "Medium Dark",
            Brightness::MediumLight => "Medium Light",
            Brightness::Light => "Light",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Brightness::Dark => "dark",
            Brightness::MediumDark => "mediumDark",
            Brightness::MediumLight => "mediumLight",
            Brightness::Light => "light",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }
}

/// User Interface › Accent Color: the one colour for what is on, chosen or focused. Coral, the
/// default, is the app icon's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Accent {
    #[default]
    Coral,
    Blue,
    Violet,
    Green,
    Amber,
    Pink,
}

impl Accent {
    pub const ALL: [Accent; 6] = [Accent::Coral, Accent::Blue, Accent::Violet, Accent::Green, Accent::Amber, Accent::Pink];
    pub fn id(self) -> &'static str {
        match self {
            Accent::Coral => "coral",
            Accent::Blue => "blue",
            Accent::Violet => "violet",
            Accent::Green => "green",
            Accent::Amber => "amber",
            Accent::Pink => "pink",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id().eq_ignore_ascii_case(s))
    }
    /// (accent, accent_strong) on dark panels and on light ones; the strong one fills buttons
    /// with white lettering.
    fn colors(self, dark: bool) -> (u32, u32) {
        match (self, dark) {
            (Accent::Coral, true) => (0xf06a52, 0xe8573f),
            (Accent::Coral, false) => (0xd9472f, 0xd14a33),
            (Accent::Blue, true) => (0x5b96ff, 0x2f74f0),
            (Accent::Blue, false) => (0x2563eb, 0x1f5ad6),
            (Accent::Violet, true) => (0xa98bff, 0x7c55f0),
            (Accent::Violet, false) => (0x7146dc, 0x6a3fd4),
            (Accent::Green, true) => (0x3fcf8e, 0x1f9d63),
            (Accent::Green, false) => (0x16955a, 0x14874f),
            (Accent::Amber, true) => (0xf5b544, 0xc47f0e),
            (Accent::Amber, false) => (0xb06d06, 0xb06d06),
            (Accent::Pink, true) => (0xf36aa3, 0xdb3f7c),
            (Accent::Pink, false) => (0xcc3672, 0xc22e69),
        }
    }
}

/// User Interface › Color Scheme: Neumorphic (AI-Corel's default: one soft, cool surface that
/// panels and controls are raised from or pressed into), Brand (glass panels over the brand
/// gradient, solid black floating windows), Modern (slate greys and the chosen accent) or Classic
/// (the neutral greys and blue accent of earlier versions).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scheme {
    #[default]
    Neumorphic,
    Brand,
    Modern,
    Classic,
}

impl Scheme {
    pub const ALL: [Scheme; 4] = [Scheme::Neumorphic, Scheme::Brand, Scheme::Modern, Scheme::Classic];
    pub fn id(self) -> &'static str {
        match self {
            Scheme::Neumorphic => "neumorphic",
            Scheme::Brand => "brand",
            Scheme::Modern => "modern",
            Scheme::Classic => "classic",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.id().eq_ignore_ascii_case(s))
    }
}

/// All colours and metrics the UI uses. Never hard-code a colour in a widget.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub smart_guide: Color32,
    pub guide: Color32,
    pub measure_bg: Color32,
    pub button: Color32,
    pub radius: u8,
    /// Field values, button labels, link labels.
    pub text_strong: Color32,
    /// Panel tab strips, doc-tab bar, dock header.
    pub tab_strip: Color32,
    /// Outline of secondary (outlined) buttons.
    pub button_border: Color32,
    /// Default Layer 1 selection colour and selected-anchor fill.
    pub selection: Color32,
    pub anchor_selected: Color32,
    /// Active item text in tool flyouts.
    pub flyout_active: Color32,
    /// Hint-bar / tab-bar heights etc. scale with this base font size.
    pub font_size: f32,
    /// Window Close button (custom title bar) when hovered, and its glyph there.
    pub caption_close: Color32,
    pub caption_close_text: Color32,
    /// What preview images mark (Flattener Preview highlights).
    pub highlight: Color32,
    /// The bleed outline around artboards (Document Setup → Bleed).
    pub bleed: Color32,
    /// An artboard's Show Center Mark and Show Cross Hairs (Artboard Options), drawn over its
    /// paper in every brightness.
    pub artboard_mark: Color32,
    /// Error marks (a missing linked file).
    pub error: Color32,
    /// Warning marks (a modified linked file).
    pub warning: Color32,
    /// Rotation gizmo axes, also labelled X, Y and Z.
    pub axis_x: Color32,
    pub axis_y: Color32,
    pub axis_z: Color32,
    /// Neumorphic: panels and controls are raised from or pressed into the surface with paired
    /// soft shadows ([`Tokens::neu_shadows`]) instead of borders.
    pub neu: bool,
    /// Brand: glass panels over the painted backdrop ([`paint_backdrop`]); floating windows are
    /// solid black.
    pub brand: bool,
}

impl Tokens {
    pub fn for_brightness(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        // The Craft look: cool slate greys, flat faces with soft corners, and one accent (the app
        // icon's coral) for what is on, chosen or focused. The four brightness levels share the
        // style and differ only in their greys.
        let base = Tokens {
            dark: true,
            app_bar: hex(0x0b0d12),
            panel: hex(0x181b22),
            panel_darker: hex(0x13161c),
            input: hex(0x0f1116),
            input_border: hex(0x2c323e),
            border: hex(0x06070a),
            divider: hex(0x232832),
            text: hex(0xdfe4ec),
            text_dim: hex(0x969faf),
            text_disabled: hex(0x5a6271),
            icon: hex(0xc9d0dc),
            hover: hex(0x262b35),
            tool_active: hex(0x2f3541),
            accent: hex(0xf06a52),
            accent_strong: hex(0xe8573f),
            row_selected: hex(0x3a2a2c),
            pasteboard: hex(0x101217),
            ruler: hex(0x13161c),
            ruler_tick: hex(0x6f788a),
            smart_guide: hex(0xff3dfc),
            guide: hex(0x4affff),
            measure_bg: Color32::from_rgba_unmultiplied(24, 27, 34, 235),
            button: hex(0x282d37),
            radius: 4,
            text_strong: hex(0xffffff),
            tab_strip: hex(0x0f1116),
            button_border: hex(0x4a5363),
            selection: hex(0x4f80ff),
            anchor_selected: hex(0x3d82ff),
            flyout_active: hex(0xff8a74),
            font_size: 13.0,
            caption_close: hex(0xc42b1c),
            caption_close_text: Color32::WHITE,
            highlight: hex(0xff3030),
            bleed: hex(0xf03030),
            artboard_mark: hex(0x808080),
            error: hex(0xe34850),
            warning: hex(0xf0a330),
            axis_x: hex(0xef6058),
            axis_y: hex(0x45af63),
            axis_z: hex(0x378ef0),
            neu: false,
            brand: false,
        };
        match b {
            Brightness::Dark => base,
            // Charcoal, the default.
            Brightness::MediumDark => Tokens {
                app_bar: hex(0x1b1f27),
                panel: hex(0x272c36),
                panel_darker: hex(0x21262f),
                tab_strip: hex(0x1e222a),
                input: hex(0x1a1e26),
                input_border: hex(0x3a414e),
                border: hex(0x11141a),
                divider: hex(0x323946),
                text: hex(0xe4e9f1),
                text_dim: hex(0x99a2b2),
                text_disabled: hex(0x626b7a),
                icon: hex(0xccd3de),
                hover: hex(0x353c49),
                tool_active: hex(0x3d4553),
                row_selected: hex(0x4a3236),
                pasteboard: hex(0x1e222a),
                ruler: hex(0x21262f),
                ruler_tick: hex(0x7c8597),
                measure_bg: Color32::from_rgba_unmultiplied(39, 44, 54, 235),
                button: hex(0x39404d),
                button_border: hex(0x5a6476),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xc4cad4),
                panel: hex(0xd3d8e0),
                panel_darker: hex(0xc9cfd8),
                tab_strip: hex(0xc4cad4),
                input: hex(0xe9ecf1),
                input_border: hex(0x9aa2b0),
                border: hex(0xa9b0bc),
                divider: hex(0xbcc3ce),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x4a5160),
                text_disabled: hex(0x7d8594),
                icon: hex(0x272c36),
                hover: hex(0xc5ccd6),
                tool_active: hex(0xb6bec9),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xe7c4bd),
                pasteboard: hex(0xb9c0cb),
                ruler: hex(0xccd2db),
                ruler_tick: hex(0x4a5160),
                measure_bg: Color32::from_rgba_unmultiplied(211, 216, 224, 235),
                button: hex(0xc9d0da),
                button_border: hex(0x8a93a2),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe6e9ee),
                panel: hex(0xf3f5f8),
                panel_darker: hex(0xe9ecf1),
                tab_strip: hex(0xe6e9ee),
                input: hex(0xffffff),
                input_border: hex(0xc3c9d3),
                border: hex(0xd0d5dd),
                divider: hex(0xdde1e8),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x58606e),
                text_disabled: hex(0x9aa1ad),
                icon: hex(0x2a303b),
                hover: hex(0xe3e7ed),
                tool_active: hex(0xd5dbe3),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xf8d9d2),
                pasteboard: hex(0xdfe3e9),
                ruler: hex(0xeceff3),
                ruler_tick: hex(0x58606e),
                measure_bg: Color32::from_rgba_unmultiplied(243, 245, 248, 235),
                button: hex(0xe3e7ed),
                button_border: hex(0xa4abb8),
                ..base
            },
        }
    }

    /// The Classic colour scheme at brightness `b`: neutral greys with a blue accent, a dark
    /// pressed tool and blue-grey selected rows, on the Modern metrics (corners, sizes).
    pub fn classic(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let base = Tokens {
            app_bar: hex(0x262626),
            panel: hex(0x323232),
            panel_darker: hex(0x282828),
            input: hex(0x1f1f1f),
            input_border: hex(0x464646),
            border: hex(0x1b1b1b),
            divider: hex(0x3f3f3f),
            text: hex(0xd8d8d8),
            text_dim: hex(0xa8a8a8),
            text_disabled: hex(0x6e6e6e),
            icon: hex(0xcfcfcf),
            hover: hex(0x444444),
            tool_active: hex(0x1d1d1d),
            accent: hex(0x378ef0),
            accent_strong: hex(0x1473e6),
            row_selected: hex(0x484b50),
            pasteboard: hex(0x1e1e1e),
            ruler: hex(0x2f2f2f),
            ruler_tick: hex(0x8a8a8a),
            measure_bg: Color32::from_rgba_unmultiplied(92, 92, 92, 235),
            button: hex(0x444444),
            text_strong: hex(0xf5f5f5),
            tab_strip: hex(0x282828),
            button_border: hex(0x6a6a6a),
            flyout_active: hex(0x54b2f7),
            ..Tokens::for_brightness(Brightness::Dark)
        };
        match b {
            Brightness::Dark => base,
            Brightness::MediumDark => Tokens {
                // A shade below the panels, so the cards stand out from the window behind them.
                app_bar: hex(0x464646),
                panel: hex(0x535353),
                panel_darker: hex(0x424242),
                tab_strip: hex(0x424242),
                input: hex(0x454545),
                input_border: hex(0x5f5f5f),
                border: hex(0x383838),
                divider: hex(0x4b4b4b),
                text: hex(0xd1d1d1),
                text_strong: hex(0xffffff),
                text_dim: hex(0xb0b0b0),
                text_disabled: hex(0x7a7a7a),
                icon: hex(0xc2c2c2),
                hover: hex(0x606060),
                tool_active: hex(0x303030),
                row_selected: hex(0x52677c),
                pasteboard: hex(0x606060),
                ruler: hex(0x333333),
                button: hex(0x535353),
                button_border: hex(0x747474),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xa9a9a9),
                panel: hex(0xb8b8b8),
                panel_darker: hex(0xaaaaaa),
                tab_strip: hex(0xa6a6a6),
                input: hex(0xd6d6d6),
                input_border: hex(0x8c8c8c),
                border: hex(0x969696),
                divider: hex(0x9e9e9e),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x3c3c3c),
                text_disabled: hex(0x7c7c7c),
                icon: hex(0x2a2a2a),
                hover: hex(0xc8c8c8),
                tool_active: hex(0x969696),
                row_selected: hex(0x9fb4d6),
                pasteboard: hex(0xa0a0a0),
                ruler: hex(0xc2c2c2),
                ruler_tick: hex(0x4a4a4a),
                measure_bg: Color32::from_rgba_unmultiplied(184, 184, 184, 235),
                button: hex(0xcacaca),
                button_border: hex(0x7a7a7a),
                flyout_active: hex(0x1473e6),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe4e4e4),
                panel: hex(0xf0f0f0),
                panel_darker: hex(0xe2e2e2),
                tab_strip: hex(0xdddddd),
                input: hex(0xffffff),
                input_border: hex(0xb4b4b4),
                border: hex(0xcfcfcf),
                divider: hex(0xd6d6d6),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x4b4b4b),
                text_disabled: hex(0x9a9a9a),
                icon: hex(0x303030),
                hover: hex(0xdddddd),
                tool_active: hex(0xcdcdcd),
                row_selected: hex(0xc8daf7),
                pasteboard: hex(0xdcdcdc),
                ruler: hex(0xeaeaea),
                ruler_tick: hex(0x5a5a5a),
                measure_bg: Color32::from_rgba_unmultiplied(240, 240, 240, 235),
                button: hex(0xe0e0e0),
                button_border: hex(0x9a9a9a),
                flyout_active: hex(0x1473e6),
                ..base
            },
        }
    }

    /// The Neumorphic scheme: one soft, cool surface (light blue for the light brightnesses, deep
    /// slate for the dark ones) with a calm blue accent.
    pub fn neumorphic(b: Brightness) -> Self {
        let rgb = Color32::from_rgb;
        let dark = matches!(b, Brightness::Dark | Brightness::MediumDark);
        let base = Tokens::for_brightness(if dark { Brightness::Dark } else { Brightness::Light });
        if dark {
            Tokens {
                app_bar: rgb(38, 45, 58),
                panel: rgb(42, 50, 64),
                panel_darker: rgb(35, 42, 54),
                tab_strip: rgb(38, 45, 58),
                input: rgb(31, 37, 48),
                input_border: rgb(62, 72, 90),
                border: rgb(26, 31, 40),
                divider: rgb(52, 61, 77),
                text: rgb(222, 229, 240),
                text_strong: Color32::WHITE,
                text_dim: rgb(150, 162, 182),
                text_disabled: rgb(96, 108, 128),
                icon: rgb(204, 213, 228),
                hover: rgb(50, 59, 75),
                tool_active: rgb(31, 37, 48),
                accent: rgb(92, 168, 255),
                accent_strong: rgb(52, 128, 226),
                flyout_active: rgb(92, 168, 255),
                row_selected: rgb(44, 70, 104),
                pasteboard: rgb(33, 39, 51),
                ruler: rgb(38, 45, 58),
                ruler_tick: rgb(126, 138, 158),
                measure_bg: Color32::from_rgba_unmultiplied(42, 50, 64, 235),
                button: rgb(42, 50, 64),
                button_border: rgb(90, 102, 124),
                radius: 8,
                neu: true,
                ..base
            }
        } else {
            Tokens {
                app_bar: rgb(228, 236, 246),
                panel: rgb(230, 238, 248),
                panel_darker: rgb(221, 230, 242),
                tab_strip: rgb(228, 236, 246),
                input: rgb(219, 228, 241),
                input_border: rgb(190, 203, 221),
                border: rgb(205, 216, 231),
                divider: rgb(210, 220, 234),
                text: rgb(40, 52, 72),
                text_strong: rgb(16, 24, 40),
                text_dim: rgb(90, 104, 126),
                text_disabled: rgb(150, 162, 180),
                icon: rgb(52, 66, 88),
                hover: rgb(238, 244, 252),
                tool_active: rgb(216, 226, 240),
                accent: rgb(52, 128, 226),
                accent_strong: rgb(40, 112, 212),
                flyout_active: rgb(40, 112, 212),
                row_selected: rgb(205, 223, 247),
                pasteboard: rgb(216, 226, 239),
                ruler: rgb(224, 232, 243),
                ruler_tick: rgb(106, 120, 142),
                measure_bg: Color32::from_rgba_unmultiplied(230, 238, 248, 235),
                button: rgb(230, 238, 248),
                button_border: rgb(170, 186, 208),
                radius: 8,
                neu: true,
                ..base
            }
        }
    }

    /// The Brand scheme: glass panels (translucent white) over the brand gradient painted on the
    /// background layer, solid black menus and windows, a soft violet-blue accent. Always dark.
    pub fn brand() -> Self {
        let rgb = Color32::from_rgb;
        let glass = Color32::from_white_alpha;
        Tokens {
            app_bar: Color32::TRANSPARENT,
            panel: glass(26),
            panel_darker: glass(16),
            tab_strip: glass(12),
            input: Color32::from_rgba_unmultiplied(22, 22, 39, 215),
            input_border: glass(44),
            border: glass(24),
            divider: glass(30),
            text: rgb(238, 238, 255),
            text_strong: Color32::WHITE,
            text_dim: rgb(176, 180, 214),
            text_disabled: rgb(112, 116, 150),
            icon: rgb(222, 224, 246),
            hover: glass(34),
            tool_active: Color32::from_rgba_unmultiplied(102, 126, 234, 150),
            accent: rgb(150, 164, 255),
            accent_strong: rgb(108, 105, 204),
            flyout_active: rgb(172, 184, 255),
            row_selected: Color32::from_rgba_unmultiplied(102, 126, 234, 110),
            pasteboard: Color32::TRANSPARENT,
            ruler: glass(12),
            ruler_tick: rgb(170, 174, 210),
            measure_bg: Color32::from_black_alpha(235),
            button: glass(26),
            button_border: glass(96),
            radius: 8,
            brand: true,
            ..Tokens::for_brightness(Brightness::Dark)
        }
    }

    /// The tokens for `b` in `scheme`; Modern takes `accent`, the others have their own.
    pub fn for_scheme(scheme: Scheme, b: Brightness, accent: Accent) -> Self {
        match scheme {
            Scheme::Neumorphic => Self::neumorphic(b),
            Scheme::Brand => Self::brand(),
            Scheme::Modern => Self::for_brightness(b).with_accent(accent),
            Scheme::Classic => Self::classic(b),
        }
    }

    /// The theme's id for the web page (`window.vectorcraft.currentTheme()`): `neuLight`,
    /// `neuDark` and `brand` for AI-Corel's own looks, else `modern-dark`, `classic-light`, ….
    pub fn theme_id(&self) -> &'static str {
        match (self.brand, self.neu, self.dark) {
            (true, _, _) => "brand",
            (_, true, true) => "neuDark",
            (_, true, false) => "neuLight",
            (_, _, true) => "dark",
            (_, _, false) => "light",
        }
    }

    /// The fill of floating windows and popups: solid black in Brand, else the panel colour.
    pub fn float_fill(&self) -> Color32 {
        if self.brand { Color32::BLACK } else { self.panel }
    }

    /// The outline of floating windows: a faint edge on Brand's black, else the border colour.
    pub fn float_border(&self) -> Color32 {
        if self.brand { Color32::from_white_alpha(22) } else { self.border }
    }

    /// The Neumorphic shadow pair `(light, dark)`: the light one falls up and to the left of a
    /// raised shape, the dark one down and to the right (reversed inside a pressed one).
    pub fn neu_shadows(&self) -> (Color32, Color32) {
        if self.dark {
            (Color32::from_rgba_unmultiplied(66, 77, 96, 150), Color32::from_rgba_unmultiplied(10, 13, 20, 210))
        } else {
            (Color32::from_rgba_unmultiplied(255, 255, 255, 240), Color32::from_rgba_unmultiplied(140, 160, 190, 190))
        }
    }

    /// These tokens with `accent`: the accent, buttons filled with it, the active item in flyouts and
    /// selected rows (the panel tinted towards it).
    pub fn with_accent(self, accent: Accent) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let (a, strong) = accent.colors(self.dark);
        let (a, strong) = (hex(a), hex(strong));
        let tint = |k: f32| {
            let mix = |p: u8, q: u8| (f32::from(p) + (f32::from(q) - f32::from(p)) * k).round() as u8;
            Color32::from_rgb(mix(self.panel.r(), a.r()), mix(self.panel.g(), a.g()), mix(self.panel.b(), a.b()))
        };
        Tokens {
            accent: a,
            accent_strong: strong,
            flyout_active: if self.dark { a } else { strong },
            row_selected: tint(if self.dark { 0.24 } else { 0.2 }),
            tool_active: tint(if self.dark { 0.3 } else { 0.24 }),
            ..self
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::default()))
    }
    pub fn cr(&self) -> CornerRadius {
        CornerRadius::same(self.radius)
    }
}

/// The toolbar, the panel dock and the icon rail are cards: `body` drawn on the panel colour with
/// soft corners, `margin` of the window's frame colour around it.
pub fn card<R>(ui: &mut egui::Ui, t: &Tokens, margin: egui::Margin, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::NONE
        .fill(t.app_bar)
        .inner_margin(margin)
        .show(ui, |ui| {
            // Neumorphic cards rise out of the surface: their soft shadows are painted behind them,
            // into the frame's margin (a slot reserved now, filled once the card's size is known).
            let shadow = t.neu.then(|| ui.painter().add(egui::Shape::Noop));
            let inner = egui::Frame::NONE.fill(t.panel).corner_radius(CARD_RADIUS).show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                body(ui)
            });
            if let Some(slot) = shadow {
                let rect = inner.response.rect;
                let painter = ui.painter().clone().with_clip_rect(rect.expand(NEU_CARD_DEPTH * 2.5));
                painter.set(slot, neu_raised_shape(rect, CARD_RADIUS, NEU_CARD_DEPTH, t));
            }
            inner.inner
        })
        .inner
}

/// How far Neumorphic cards rise out of the surface, in points.
pub const NEU_CARD_DEPTH: f32 = 3.5;

/// The paired soft shadows of a Neumorphic shape raised `depth` points out of the surface: the
/// light shadow up and to the left, the dark one down and to the right. Paint it behind the
/// shape's own fill.
pub fn neu_raised_shape(rect: egui::Rect, radius: impl Into<CornerRadius> + Copy, depth: f32, t: &Tokens) -> egui::Shape {
    let (light, dark) = t.neu_shadows();
    let d = depth.round().clamp(1.0, 24.0) as i8;
    let blur = (depth * 2.6).round().clamp(2.0, 60.0) as u8;
    egui::Shape::Vec(vec![
        egui::Shadow { offset: [-d, -d], blur, spread: 0, color: light }.as_shape(rect, radius).into(),
        egui::Shadow { offset: [d, d], blur, spread: 0, color: dark }.as_shape(rect, radius).into(),
    ])
}

/// The inner shading of a Neumorphic shape pressed into the surface (egui has no inset shadow):
/// fading bands of the dark shadow along the inside top and left edges and of the light one along
/// the bottom and right, clipped to the shape.
pub fn neu_pressed(painter: &egui::Painter, rect: egui::Rect, radius: f32, t: &Tokens) {
    let (light, dark) = t.neu_shadows();
    let clip = rect.intersect(painter.clip_rect());
    if !clip.is_positive() {
        return;
    }
    let p = painter.clone().with_clip_rect(clip);
    for (i, strength) in [0.85_f32, 0.5, 0.22].into_iter().enumerate() {
        let o = 1.0 + i as f32;
        let w = Stroke::new(1.6, dark.gamma_multiply(strength));
        p.rect_stroke(rect.translate(egui::vec2(o, o)).expand(1.0), radius, w, egui::StrokeKind::Outside);
        let w = Stroke::new(1.6, light.gamma_multiply(strength * 0.9));
        p.rect_stroke(rect.translate(egui::vec2(-o, -o)).expand(1.0), radius, w, egui::StrokeKind::Outside);
    }
}

/// Where the Brand backdrop is laid out when the window is not the whole page: the page's
/// viewport in the app's screen points. The web shell sets it, so the backdrop lines up with the
/// same CSS gradient around the canvas.
pub const BACKDROP_FRAME: &str = "ai-corel-backdrop-frame";

/// The Brand backdrop, behind every panel: on a 135° linear gradient #0a0a0f → #1a1a2e → #16213e,
/// three soft radial spots (40%/40% cyan, 80%/20% pink, 20%/80% violet, each fading to nothing at
/// half the distance to the farthest corner), laid out over the viewport like CSS
/// `background-attachment: fixed`. Evaluated on a vertex grid; the colours change slowly, so
/// linear interpolation between vertices matches the CSS gradient.
pub fn paint_backdrop(ctx: &egui::Context) {
    if !Tokens::get(ctx).brand {
        return;
    }
    let screen = ctx.content_rect();
    if !screen.is_positive() {
        return;
    }
    let frame = ctx
        .data(|d| d.get_temp::<egui::Rect>(egui::Id::new(BACKDROP_FRAME)))
        .filter(|f| f.is_positive() && f.width().is_finite() && f.height().is_finite())
        .unwrap_or(screen);
    const NX: usize = 48;
    const NY: usize = 30;
    let mut mesh = egui::Mesh::default();
    for j in 0..=NY {
        for i in 0..=NX {
            let p = egui::pos2(
                egui::lerp(screen.left()..=screen.right(), i as f32 / NX as f32),
                egui::lerp(screen.top()..=screen.bottom(), j as f32 / NY as f32),
            );
            mesh.colored_vertex(p, backdrop_color(p - frame.min, frame.size()));
        }
    }
    let row = (NX + 1) as u32;
    for j in 0..NY as u32 {
        for i in 0..NX as u32 {
            let a = j * row + i;
            mesh.add_triangle(a, a + 1, a + row + 1);
            mesh.add_triangle(a, a + row + 1, a + row);
        }
    }
    ctx.layer_painter(egui::LayerId::background()).add(mesh);
}

/// The backdrop's colour at `p`, an offset inside a viewport of `size` (sRGB, like CSS).
pub fn backdrop_color(p: egui::Vec2, size: egui::Vec2) -> Color32 {
    let (w, h) = (size.x.max(1.0), size.y.max(1.0));
    // linear-gradient(135deg, …): towards the bottom right, over |w sin θ| + |h cos θ|.
    let (s, c) = (std::f32::consts::FRAC_1_SQRT_2, -std::f32::consts::FRAC_1_SQRT_2);
    let len = (w * s).abs() + (h * c).abs();
    let t = (((p.x - w / 2.0) * s + (p.y - h / 2.0) * -c) / len + 0.5).clamp(0.0, 1.0);
    let stops = [(0.0, [10.0, 10.0, 15.0]), (0.5, [26.0, 26.0, 46.0]), (1.0, [22.0, 33.0, 62.0])];
    let mut rgb = stops[0].1;
    for pair in stops.windows(2) {
        let ((t0, a), (t1, b)) = (pair[0], pair[1]);
        if t >= t0 && t <= t1 {
            let k = (t - t0) / (t1 - t0);
            rgb = [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k];
        }
    }
    // radial-gradient(circle at x% y%, colour 0%, transparent 50%), bottom layer first.
    for (cx, cy, spot, alpha) in
        [(0.4, 0.4, [120.0, 219.0, 255.0], 0.2), (0.8, 0.2, [255.0, 119.0, 198.0], 0.3), (0.2, 0.8, [120.0, 119.0, 198.0], 0.3)]
    {
        let (ox, oy) = (cx * w, cy * h);
        let far =
            [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)].iter().map(|(x, y)| ((x - ox) * (x - ox) + (y - oy) * (y - oy)).sqrt()).fold(0.0_f32, f32::max);
        let r = ((p.x - ox) * (p.x - ox) + (p.y - oy) * (p.y - oy)).sqrt();
        let a = alpha * (1.0 - r / (0.5 * far.max(1.0))).clamp(0.0, 1.0);
        for k in 0..3 {
            rgb[k] = rgb[k] * (1.0 - a) + spot[k] * a;
        }
    }
    Color32::from_rgb(rgb[0].round() as u8, rgb[1].round() as u8, rgb[2].round() as u8)
}

/// The corner radius of [`card`]s and floating windows.
pub const CARD_RADIUS: u8 = 8;

/// Selected text in the text fields `ui` draws from here on is in the strong text colour. egui
/// paints it in the selection stroke's colour, the accent, which is hard to read on the selection's
/// accent tint (#1465). For fields that draw their own frame: egui's frame takes its focus ring from
/// the same stroke.
pub fn strong_selected_text(ui: &mut egui::Ui) {
    ui.visuals_mut().selection.stroke.color = Tokens::get(ui.ctx()).text_strong;
}

pub const FONT_UI: &str = "ui";
pub const FONT_UI_SEMIBOLD: &str = "ui-semibold";
pub const FONT_MONO: &str = "mono";

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_UI_SEMIBOLD.into()))
}
pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_MONO.into()))
}

/// Installs the interface's fonts and binds the families the panels and dialogs draw in
/// ([`FONT_UI`], [`FONT_UI_SEMIBOLD`], [`FONT_MONO`]). egui panics drawing text in a family no font
/// is bound to, so every context that shows them needs this once: [`crate::VectorcraftApp::logic`]
/// calls it on its first frame, and an embedder that draws the panels or dialogs without it calls
/// it when it makes the context. It replaces the context's font definitions: add other fonts after
/// it, or keep these families in definitions of one's own. Without the `interface-fonts` feature
/// the families are egui's built-in fonts (and craft-fonts' faces in a build with them), which lack
/// some symbols the panels show (→ ▾ ● ✓).
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let (ui, semi, mono) = interface_faces(&mut fonts);
    let stack = |fonts: &FontDefinitions, family: FontFamily| fonts.families.get(&family).cloned().unwrap_or_default();
    let (fallback, monospace) = (stack(&fonts, FontFamily::Proportional), stack(&fonts, FontFamily::Monospace));
    // Without its own monospace face, `mono` is egui's; with it, the face falls back to the text's.
    let mono_fallback = if mono.is_empty() { monospace } else { fallback.clone() };
    let family =
        |faces: Vec<&str>, fallback: &[String]| -> Vec<String> { faces.into_iter().map(str::to_owned).chain(fallback.iter().cloned()).collect() };
    fonts.families.insert(FontFamily::Proportional, family(ui.clone(), &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI_SEMIBOLD.into()), family(semi, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI.into()), family(ui, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_MONO.into()), family(mono, &mono_fallback));
    add_craft_fonts(&mut fonts);
    ctx.set_fonts(fonts);
}

/// The interface's own faces, added to `fonts`: the names of the regular, semibold and monospace
/// faces that head their families, in order.
#[cfg(feature = "interface-fonts")]
fn interface_faces(fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    let add = |f: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        f.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    };
    add(fonts, "SourceSans3", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    // Inter is drawn a little smaller than its size: its letters are wider than Source Sans's, and
    // the panels are laid out for those.
    let inter =
        |bytes: &'static [u8]| Arc::new(FontData::from_static(bytes).tweak(egui::epaint::text::FontTweak { scale: 0.92, ..Default::default() }));
    fonts.font_data.insert("Inter".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-Regular.ttf")));
    fonts.font_data.insert("Inter-SemiBold".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf")));
    add(fonts, "SourceSans3-Semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(fonts, "JetBrainsMono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    (vec!["Inter", "SourceSans3"], vec!["Inter-SemiBold", "SourceSans3-Semibold"], vec!["JetBrainsMono"])
}

/// Without the `interface-fonts` feature the interface has no faces of its own: every family is egui's.
#[cfg(not(feature = "interface-fonts"))]
fn interface_faces(_fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    (Vec::new(), Vec::new(), Vec::new())
}

/// The egui name of a craft-fonts face.
pub(crate) fn craft_font_name(f: &vectorcraft_text::CraftFont) -> String {
    format!("craft-fonts {} {}", f.family, f.style)
}

/// The Japanese and Georgian craft-fonts faces (when built with `CRAFT_FONTS_DIR`) as fallbacks at
/// the end of every family, after the app's own fonts and before the installed fonts `ui_fonts`
/// adds: BIZ UDPGothic first (bold first in the semibold family), then the Mincho faces, then Noto
/// Sans Georgian.
fn add_craft_fonts(fonts: &mut FontDefinitions) {
    for f in vectorcraft_text::CRAFT_FONTS.iter().filter(|f| f.is_japanese() || f.is_georgian()) {
        fonts.font_data.insert(craft_font_name(f), Arc::new(FontData::from_static(f.bytes)));
    }
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name(FONT_UI_SEMIBOLD.into());
        stack.extend(vectorcraft_text::craft_fonts::japanese_ui_fonts(bold).into_iter().map(craft_font_name));
        stack.extend(vectorcraft_text::craft_fonts::georgian_ui_fonts().into_iter().map(craft_font_name));
    }
}

/// Apply tokens to egui's global style.
pub fn apply(ctx: &egui::Context, b: Brightness) {
    apply_with(ctx, b, Accent::default(), Scheme::default());
}

/// [`apply`] with an accent colour and colour scheme.
pub fn apply_with(ctx: &egui::Context, b: Brightness, accent: Accent, scheme: Scheme) {
    let t = Tokens::for_scheme(scheme, b, accent);
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, t));
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, t.border);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.selection.bg_fill = t.accent.gamma_multiply(0.55);
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(if t.dark { 120 } else { 60 }) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(if t.dark { 140 } else { 70 }) };
    if t.brand {
        // Brand: menus, popups and windows are solid black blocks with a strong black shadow.
        v.window_fill = Color32::BLACK;
        v.panel_fill = Color32::TRANSPARENT;
        v.extreme_bg_color = t.input;
        v.window_stroke = Stroke::new(1.0, t.float_border());
        let shadow = egui::epaint::Shadow { offset: [0, 8], blur: 32, spread: 0, color: Color32::BLACK };
        v.window_shadow = shadow;
        v.popup_shadow = shadow;
    } else if t.neu {
        // Neumorphic: soft, borderless windows lifted by a cool shadow.
        let (_, dark) = t.neu_shadows();
        v.window_stroke = Stroke::NONE;
        v.window_shadow = egui::epaint::Shadow { offset: [6, 8], blur: 26, spread: 0, color: dark };
        v.popup_shadow = egui::epaint::Shadow { offset: [4, 6], blur: 18, spread: 0, color: dark };
        v.window_corner_radius = CornerRadius::same(14);
        v.menu_corner_radius = CornerRadius::same(10);
    }
    let w = &mut v.widgets;
    craft_ui::theme::WidgetPalette { noninteractive: t.panel, inactive: t.button, hovered: t.hover, active: t.tool_active, open: t.hover }.apply_to(
        w,
        t.cr(),
        Stroke::new(1.0, t.text),
    );
    w.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
    w.inactive.bg_stroke = Stroke::NONE;
    w.hovered.bg_stroke = Stroke::new(1.0, t.input_border);
    w.active.bg_stroke = Stroke::new(1.0, t.accent);
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(24.0, 24.0);
        s.spacing.menu_margin = egui::Margin::same(6);
        s.spacing.slider_width = 120.0;
        s.text_styles = [
            (TextStyle::Small, FontId::proportional(11.0)),
            (TextStyle::Body, FontId::proportional(13.0)),
            (TextStyle::Button, FontId::proportional(13.0)),
            (TextStyle::Heading, FontId::new(15.0, FontFamily::Name(FONT_UI_SEMIBOLD.into()))),
            (TextStyle::Monospace, FontId::new(11.5, FontFamily::Name(FONT_MONO.into()))),
        ]
        .into();
        s.animation_time = 0.08;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_roundtrip() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        assert!(!Tokens::for_brightness(Brightness::Light).dark);
    }

    /// #1662: a slider's rail (`button_border`, see `widgets::slider`) stands out from the panel
    /// in every brightness.
    #[test]
    fn slider_rails_show_on_every_panel() {
        for b in Brightness::ALL {
            for scheme in Scheme::ALL {
                let t = Tokens::for_scheme(scheme, b, Accent::default());
                let gap = (i32::from(t.button_border.r()) - i32::from(t.panel.r())).abs();
                assert!(gap >= 24, "{scheme:?} {b:?}: rail {:?} on panel {:?}", t.button_border, t.panel);
            }
        }
    }

    /// Classic keeps the Modern corners and the cards' frame, and has its own blue accent
    /// whatever accent is chosen.
    #[test]
    fn classic_scheme_keeps_the_corners_and_its_blue() {
        for b in Brightness::ALL {
            let (modern, classic) = (Tokens::for_brightness(b), Tokens::for_scheme(Scheme::Classic, b, Accent::Pink));
            assert_eq!(classic.radius, modern.radius);
            assert_eq!(classic.dark, modern.dark, "{b:?}");
            assert_ne!(classic.app_bar, classic.panel, "{b:?}: the cards need a frame");
            assert_eq!(classic.accent, Color32::from_rgb(0x37, 0x8e, 0xf0));
            assert_eq!(Scheme::parse(Scheme::Classic.id()), Some(Scheme::Classic));
        }
    }
}
