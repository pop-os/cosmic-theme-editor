use palette::rgb::Srgb;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection {
    pub background: Srgb,
    pub primary_container: Srgb,
    pub secondary_container: Srgb,
    pub accent_color: Srgb,
    pub accent_text_color: Option<Srgb>,
    pub accent_nav_handle_text_color: Option<Srgb>,
    pub destructive_color: Srgb,
}
