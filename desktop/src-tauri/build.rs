use std::fs;
use std::io;
use std::path::Path;

fn ensure_default_icon() -> io::Result<()> {
    let icons_dir = Path::new("icons");
    let ico_path = icons_dir.join("icon.ico");

    if ico_path.exists() {
        // If a previous build created a PNG-in-ICO, tauri may fail to decode it.
        // Detect that case and overwrite with a simple DIB-based ICO.
        if let Ok(existing) = fs::read(&ico_path) {
            // After ICONDIR (6) + ICONDIRENTRY (16) = 22 bytes, PNG images start with 89 50 4E 47.
            if existing.len() >= 26 && existing[22..26] == [0x89, 0x50, 0x4E, 0x47] {
                let _ = fs::remove_file(&ico_path);
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        }
    }

    fs::create_dir_all(icons_dir)?;

    // Minimal ICO using a DIB/BMP payload (more universally accepted than PNG-in-ICO).
    // ICONDIR (6 bytes) + ICONDIRENTRY (16 bytes) + BITMAPINFOHEADER (40 bytes)
    // + pixel data (4 bytes, BGRA) + AND mask (4 bytes, 1 row aligned to 32 bits).
    let bytes_in_res: u32 = 40 + 4 + 4;
    let image_offset: u32 = 6 + 16;

    let mut ico = Vec::with_capacity(image_offset as usize + bytes_in_res as usize);

    // ICONDIR
    ico.extend_from_slice(&0u16.to_le_bytes()); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // type = icon
    ico.extend_from_slice(&1u16.to_le_bytes()); // count

    // ICONDIRENTRY
    ico.push(1); // width
    ico.push(1); // height
    ico.push(0); // color count
    ico.push(0); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // planes
    ico.extend_from_slice(&32u16.to_le_bytes()); // bit count

    ico.extend_from_slice(&bytes_in_res.to_le_bytes());
    ico.extend_from_slice(&image_offset.to_le_bytes());

    // BITMAPINFOHEADER (40 bytes)
    ico.extend_from_slice(&40u32.to_le_bytes()); // biSize
    ico.extend_from_slice(&1i32.to_le_bytes()); // biWidth
    // biHeight includes mask, so it's doubled.
    ico.extend_from_slice(&2i32.to_le_bytes()); // biHeight
    ico.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
    ico.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
    ico.extend_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
    ico.extend_from_slice(&4u32.to_le_bytes()); // biSizeImage
    ico.extend_from_slice(&0i32.to_le_bytes()); // biXPelsPerMeter
    ico.extend_from_slice(&0i32.to_le_bytes()); // biYPelsPerMeter
    ico.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
    ico.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

    // Pixel data for 1x1 BGRA (opaque Kaspa accent-ish teal).
    // BGRA: B=0xCB, G=0xEA, R=0x49, A=0xFF
    ico.extend_from_slice(&[0xCB, 0xEA, 0x49, 0xFF]);

    // AND mask row (1-bit per pixel, padded to 32 bits). 0 = opaque.
    ico.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    fs::write(ico_path, ico)?;
    Ok(())
}

fn main() {
    // Ensure `icons/icon.ico` exists for tauri-build on Windows.
    let _ = ensure_default_icon();
    tauri_build::build()
}
