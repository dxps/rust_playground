use derive_more::Constructor;
use serde::Serialize;

use crate::{Clip, ShortCode};

/// This trait is used to get general information about every single page
/// context, plus global information such as title.
pub trait PageCtx {
    /// `title` refers to page's title.
    fn title(&self) -> &str;

    /// `template_path` refers to the path to the actual template itself.
    fn template_path(&self) -> &str;

    /// `parent` refers to the base template.
    /// This includes header, foother and such common across pages parts.
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageCtx for Home {
    fn title(&self) -> &str {
        "Stash Your Clipboard!"
    }

    fn template_path(&self) -> &str {
        "home"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: Clip,
}

impl PageCtx for ViewClip {
    fn title(&self) -> &str {
        "View Clip"
    }

    fn template_path(&self) -> &str {
        "clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: ShortCode,
}

impl PageCtx for PasswordRequired {
    fn title(&self) -> &str {
        "Password Required"
    }

    fn template_path(&self) -> &str {
        "clip_need_password"
    }

    fn parent(&self) -> &str {
        "base"
    }
}
