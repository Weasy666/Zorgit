use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::collections::BTreeMap;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    convert_icon_svg_to_css()
}

fn convert_icon_svg_to_css() -> Result<(), std::io::Error> {
    let icons_path = Path::new("assets/ui_icons");
    let css_path = Path::new("static/css/");
    fs::create_dir_all(css_path)?;
    let mut f = File::create(css_path.join("zorgit-icons.css"))?;

    let dirs = walk_dirs(icons_path)?;
    for (dir, entries) in dirs {
        f.write_all(format!("/* Icons from {} */", dir).as_bytes())?;
        f.write_all(b"\n")?;
        for icon in entries {
            if icon.file_type()?.is_file() && icon.path().extension().unwrap() == "svg" {
                let sanitized = sanitize(fs::read_to_string(icon.path())?);
                let escaped = escape(sanitized);
                let name = icon.file_name().into_string().unwrap().replace(".svg", "");
                let icon_content = format!(".zg-{} {{ mask-image: url(\"data:image/svg+xml,{svg}\"); -webkit-mask-image: url(\"data:image/svg+xml,{svg}\"); }}", name, svg=escaped);
                f.write_all(&icon_content.into_bytes())?;
                f.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}

fn walk_dirs(dir: &Path) -> Result<BTreeMap<String, Vec<DirEntry>>, std::io::Error> {
    let mut entries: BTreeMap<String, Vec<DirEntry>> = BTreeMap::new();
    if dir.is_dir() {
        let key = dir.file_name().unwrap().to_str().unwrap().to_string();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let e = walk_dirs(&path)?;
                entries.extend(e);
            } else {
                if let Some(content) = entries.get_mut(&key) {
                    content.push(entry);
                }
                else {
                    let mut content = Vec::new();
                    content.push(entry);
                    entries.insert(key.clone(), content);
                }
            }
        }
    }
    Ok(entries)
}

fn sanitize(text: String) -> String {
    text.replace("\n", "").replace("\r", "").replace("\t", "").replace("    ", "").trim().to_string()
}

fn escape(text: String) -> String {
    text.replace("\"", "'").replace("#", "%23")
}
