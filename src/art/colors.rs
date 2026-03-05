/// Semantic color tokens used to mark ASCII art segments.
/// Each token is resolved to a concrete ANSI 256 color by the active Theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtColor {
    SunCore,
    SunRay,
    CloudLight,
    CloudDark,
    RainDrop,
    SnowFlake,
    Lightning,
    FogMist,
    MoonBody,
    Star,
    #[allow(dead_code)]
    Ground,
}
