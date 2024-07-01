use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Home {}
    }
}

impl PageContext for Home {
    fn parent(&self) -> &str {
        "base"
    }
    fn template_path(&self) -> &str {
        "home"
    }
    fn title(&self) -> &str {
        "Stash Your Clipboard"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    clip: crate::domain::Clip,
}

impl PageContext for ViewClip {
    fn parent(&self) -> &str {
        "base"
    }
    fn template_path(&self) -> &str {
        "clip"
    }
    fn title(&self) -> &str {
        "View Clip"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::Shortcode,
}

impl PageContext for PasswordRequired {
    fn parent(&self) -> &str {
        "base"
    }
    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn title(&self) -> &str {
        "Password Required"
    }
}
