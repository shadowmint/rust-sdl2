#[macro_escape];

#[macro_export]
macro_rules! import_sdl_dependencies(
  () => (
    // Setup linking for all targets.
    #[cfg(target_os="macos")]
    mod mac {
        #[link(kind="framework", name="OpenGL")]
        #[link(kind="framework", name="ForceFeedback")]
        #[link(kind="framework", name="Cocoa")]
        #[link(kind="framework", name="Carbon")]
        #[link(kind="framework", name="IOKit")]
        #[link(kind="framework", name="CoreAudio")]
        #[link(kind="framework", name="AudioToolbox")]
        #[link(kind="framework", name="AudioUnit")]
        #[link(name="objc")]
        #[link(name="iconv")]
        #[link(kind="static", name="SDL2")]
        extern {}
    }

    #[cfg(target_os="linux")]
    #[cfg(target_os="freebsd")]
    mod others {
        #[link(name="m")]
        #[link(name="dl")]
        #[link(name="rt")]
        #[link(kind="static", name="SDL2")]
        extern {}
    }

    #[cfg(target_os="win32")]
    mod win {
        #[link(kind="static", name="SDL2")]
        extern {}
    }
  )
)
