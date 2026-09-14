// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::{Component, Path, PathBuf};

// OHOS has target_os="linux", but applications have no desktop home directory.
// Only accept the application's filesDir supplied by UIAbilityContext. Never
// fall back to cwd, /, or a desktop XDG directory when context is unavailable.
pub(super) fn sandbox_dir(base: Option<&str>, directory: &str) -> Option<PathBuf> {
  let base = Path::new(base?);
  if !base.is_absolute()
    || base.parent().is_none()
    || base
      .components()
      .any(|part| matches!(part, Component::ParentDir))
  {
    return None;
  }
  Some(base.join(directory))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn app_directories_stay_in_the_ability_sandbox() {
    let files = "/data/storage/el2/base/haps/entry/files";
    for suffix in ["", "config", "cache", "logs", "tmp"] {
      assert_eq!(
        sandbox_dir(Some(files), suffix),
        Some(Path::new(files).join(suffix))
      );
    }
  }

  #[test]
  fn missing_or_invalid_context_never_grants_a_desktop_directory() {
    for base in [
      None,
      Some(""),
      Some("/"),
      Some("files"),
      Some("/data/../files"),
    ] {
      assert_eq!(sandbox_dir(base, "logs"), None);
    }
  }
}
