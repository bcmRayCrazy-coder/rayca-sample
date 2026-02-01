use std::path::Path;

use crate::core::sample::SampleCategory;

pub fn is_supported_type(path: &Path) -> bool {
    path.extension()
        .map(|ext| {
            ext.eq_ignore_ascii_case("flac")
                || ext.eq_ignore_ascii_case("ogg")
                || ext.eq_ignore_ascii_case("wav")
        })
        .unwrap_or(false)
}

pub fn load_all_sample_from_dir(
    category: &mut SampleCategory,
    dir: &Path,
) -> Result<(), anyhow::Error> {
    if !dir.is_dir() {
        return Err(anyhow::Error::msg("{dir} is not a directory"));
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && is_supported_type(&path) {
            if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(id) = id.parse::<usize>() {
                    println!(
                        "Load sample {} from {}",
                        id,
                        path.to_str().unwrap_or("Unknown")
                    );
                    let file = audrey::open(path);

                    match file {
                        Ok(mut buf) => {
                            // TODO: Resample to 44100Hz
                            let buf = buf
                                .frames::<[f32; 2]>()
                                .map(|f| f.unwrap())
                                .flat_map(|f| vec![f, f])
                                .collect::<Vec<_>>();

                            let _ = category.set_sample_raw(id, buf).inspect_err(|err| {
                                println!("Failed to load sample to category: {}", err);
                                let _ = category.set_sample_fallback(id, 0);
                                println!("Using default sample if possible");
                            });
                        }
                        Err(err) => {
                            println!("Error opening file: {}", err);
                            let _ = category.set_sample_fallback(id, 0);
                            println!("Using default sample if possible");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
