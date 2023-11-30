pub mod styles {
    pub mod button {
        use bevy::prelude::Color;

        pub const BUTTON_DEFAULT: Color = Color::rgba(0., 0., 0., 0.);
        pub const BUTTON_HOVER: Color =   Color::rgba(0., 0., 0., 0.1);
        pub const BUTTON_ACTIVE: Color =  Color::rgba(0., 0., 0., 0.2);

        pub const SETTINGS_BUTTON_DEFAULT: Color = Color::rgba(1., 1., 1., 1.);
        pub const SETTINGS_BUTTON_HOVER: Color =   Color::rgba(0.9, 0.9, 0.9, 1.);
        pub const SETTINGS_BUTTON_ACTIVE: Color =  Color::rgba(0.8, 0.8, 0.8, 1.);
        // pub const DEFAULT_BORDER: Color = Color::rgb(0.8, 0.2, 0.2);

        pub const TRANSPARENT_WHITE: Color = Color::rgba(1.,1.,1.,0.35);
    }
}

pub mod font_names {

    pub const NOTO_SANS_THIN: &'static str = "internal/fonts/NotoSans-Thin.ttf";
    pub const NOTO_SANS_THIN_I: &'static str = "internal/fonts/NotoSans-ThinItalic.ttf";
    pub const NOTO_SANS_EX_LIGHT: &'static str = "internal/fonts/NotoSans-ExtraLight.ttf";
    pub const NOTO_SANS_EX_LIGHT_I: &'static str = "internal/fonts/NotoSans-ExtraLightItalic.ttf";
    pub const NOTO_SANS_LIGHT: &'static str = "internal/fonts/NotoSans-Light.ttf";
    pub const NOTO_SANS_LIGHT_I: &'static str = "internal/fonts/NotoSans-LightItalic.ttf";
    pub const NOTO_SANS_REGULAR: &'static str = "internal/fonts/NotoSans-Regular.ttf";
    pub const NOTO_SANS_REGULAR_I: &'static str = "internal/fonts/NotoSans-Italic.ttf";
    pub const NOTO_SANS_MEDIUM: &'static str = "internal/fonts/NotoSans-Medium.ttf";
    pub const NOTO_SANS_MEDIUM_I: &'static str = "internal/fonts/NotoSans-MediumItalic.ttf";
    pub const NOTO_SANS_SM_BOLD: &'static str = "internal/fonts/NotoSans-SemiBold.ttf";
    pub const NOTO_SANS_SM_BOLD_I: &'static str = "internal/fonts/NotoSans-SemiBoldItalic.ttf";
    pub const NOTO_SANS_BOLD: &'static str = "internal/fonts/NotoSans-Bold.ttf";
    pub const NOTO_SANS_BOLD_I: &'static str = "internal/fonts/NotoSans-BoldItalic.ttf";
    pub const NOTO_SANS_EX_BOLD: &'static str = "internal/fonts/NotoSans-ExtraBold.ttf";
    pub const NOTO_SANS_EX_BOLD_I: &'static str = "internal/fonts/NotoSans-ExtraBoldItalic.ttf";
    pub const NOTO_SANS_BLACK: &'static str = "internal/fonts/NotoSans-Black.ttf";
    pub const NOTO_SANS_BLACK_I: &'static str = "internal/fonts/NotoSans-BlackItalic.ttf";
}
