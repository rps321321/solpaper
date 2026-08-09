//! Prepare a Solpaper-owned cache copy of a local wallpaper source (pack #5 / #20).
//!
//! Validates extension + compressed size + header dimensions, then copies into
//! the cache directory. Does not call `IDesktopWallpaper`.

use std::fs;
use std::path::{Path, PathBuf};

use solpaper_core::{
    check_decoded_pixels, check_local_file_size, validate_source_path_shape, WallpaperErrorKind,
};

use super::WallpaperError;

/// Copy `source` into `cache_dir` as an owned file after policy checks.
///
/// On any validation failure, returns a typed error and does not write.
pub fn prepare_owned_wallpaper(source: &Path, cache_dir: &Path) -> Result<PathBuf, WallpaperError> {
    validate_source_path_shape(source).map_err(WallpaperError::from)?;
    if !source.is_file() {
        return Err(WallpaperError::new(WallpaperErrorKind::PathInvalid));
    }
    let meta = fs::metadata(source)
        .map_err(|e| WallpaperError::with_detail(WallpaperErrorKind::PathInvalid, e.to_string()))?;
    check_local_file_size(meta.len()).map_err(WallpaperError::from)?;

    // Header-only dimensions when the decoder supports the format.
    match image::image_dimensions(source) {
        Ok((w, h)) => {
            check_decoded_pixels(w, h).map_err(WallpaperError::from)?;
        }
        Err(e) => {
            return Err(WallpaperError::with_detail(
                WallpaperErrorKind::PathInvalid,
                format!("image dimensions: {e}"),
            ));
        }
    }

    fs::create_dir_all(cache_dir)
        .map_err(|e| WallpaperError::with_detail(WallpaperErrorKind::Platform, e.to_string()))?;

    let file_name = source
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("wallpaper.bin");
    // Stable owned name: hash of full path + original basename for uniqueness.
    let key = simple_path_key(source);
    let owned_name = format!("wp-{key}-{file_name}");
    let dest = cache_dir.join(owned_name);

    fs::copy(source, &dest)
        .map_err(|e| WallpaperError::with_detail(WallpaperErrorKind::Platform, e.to_string()))?;

    // Ensure position consumers see an absolute path when possible.
    Ok(fs::canonicalize(&dest).unwrap_or(dest))
}

fn simple_path_key(path: &Path) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    path.hash(&mut h);
    format!("{:016x}", h.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("solpaper-wp-prep-{n}"))
    }

    #[test]
    fn prepare_rejects_bad_extension() {
        let root = temp_dir();
        fs::create_dir_all(&root).unwrap();
        let src = root.join("x.gif");
        fs::write(&src, b"GIF89a").unwrap();
        let err = prepare_owned_wallpaper(&src, &root.join("cache")).unwrap_err();
        assert_eq!(err.kind, WallpaperErrorKind::FormatUnsupported);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn prepare_copies_valid_png() {
        let root = temp_dir();
        let cache = root.join("cache");
        fs::create_dir_all(&root).unwrap();
        // Minimal 1×1 PNG
        let png: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE,
            0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        let src = root.join("solid.png");
        fs::write(&src, png).unwrap();
        let owned = prepare_owned_wallpaper(&src, &cache).unwrap();
        assert!(owned.exists());
        assert!(owned.starts_with(&cache) || owned.to_string_lossy().contains("wp-"));
        let _ = fs::remove_dir_all(root);
    }
}
