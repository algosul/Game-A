use freetype::{Face, Library, face::LoadFlag};
#[cfg(target_os = "linux")]
#[test]
fn test() {
    use std::rc::Rc;
    let library = Library::init().unwrap();
    let face: Face = library
        .new_memory_face(
            Rc::new(include_bytes!("../rc/fonts/SEGUIEMJ.TTF").to_vec()),
            0,
        )
        .unwrap();
    face.set_char_size(48 * 64, 0, 96, 0).unwrap();
    for (i, c) in "😊😂🤣".chars().enumerate() {
        if let Some(index) = face.get_char_index(c as usize) {
            if index != 0 {
                face.load_glyph(index, LoadFlag::RENDER | LoadFlag::COLOR)
                    .unwrap();
                let glyph = face.glyph();
                use image::*;
                let bitmap = glyph.bitmap();
                let width = bitmap.width();
                let rows = bitmap.rows();
                let pitch = bitmap.pitch();
                assert_eq!(pitch, width * 4);
                if width == 0 || rows == 0 {
                    continue;
                }
                let mut buffer = bitmap.buffer().to_vec();
                // BGRA to RGBA
                for chunk in buffer.chunks_exact_mut(4) {
                    chunk.swap(0, 2);
                }
                let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
                    ImageBuffer::from_raw(width as u32, rows as u32, buffer)
                        .unwrap();
                img.save(format!("{i}.png")).unwrap();
                println!("Character: {c}, {i}.png");
            }
        }
    }
}
