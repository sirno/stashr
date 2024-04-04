use std::fs;
use std::path::Path;
pub type TransferOp = fn(&Path, &Path) -> anyhow::Result<()>;

pub fn move_file(origin: &Path, target: &Path) -> anyhow::Result<()> {
    if !origin.is_file() && !origin.is_dir() {
        let e = fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::Other,
            "Not a directory or file",
        );
        return Err(anyhow::anyhow!(e));
    }

    fs::rename(origin, target)
        .and_then(|_| Ok(()))
        .or_else(|_| {
            if origin.is_dir() {
                let options = fs_extra::dir::CopyOptions::new().content_only(true);
                fs::create_dir_all(target)?;
                fs_extra::dir::copy(origin, target, &options)?;
                fs_extra::dir::remove(origin)
            } else {
                let options = fs_extra::file::CopyOptions::new();
                fs_extra::file::copy(origin, target, &options)?;
                fs_extra::file::remove(origin)
            }
        })
        .map_err(|e| anyhow::anyhow!(e))
}

pub fn copy_file(origin: &Path, target: &Path) -> anyhow::Result<()> {
    if !origin.is_file() && !origin.is_dir() {
        let e = fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::Other,
            "Not a directory or file",
        );
        return Err(anyhow::anyhow!(e));
    }

    if origin.is_dir() {
        let options = fs_extra::dir::CopyOptions::new().content_only(true);
        fs::create_dir_all(target)?;
        fs_extra::dir::copy(origin, target, &options)?;
        Ok(())
    } else {
        let options = fs_extra::file::CopyOptions::new();
        fs_extra::file::copy(origin, target, &options)?;
        Ok(())
    }
}
