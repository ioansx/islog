use crate::error::{Errx, Resultx};

pub fn validate_new_content(new_content: &str) -> Resultx<()> {
    let lines: Vec<&str> = new_content.lines().collect();
    if lines.iter().any(|x| x.contains("# ")) {
        return Err(Errx::validation(
            "cannot have Title (Header 1) in new content",
        ));
    }

    let mut subtitle_count = 0;
    for line in &lines {
        if line.contains("## ") {
            subtitle_count += 1;
        }
    }
    if subtitle_count > 1 {
        return Err(Errx::validation(
            "cannot have more than one Subtitle (Header 2) in new content; currently having {subtitle_count} subtitles",
        ));
    }

    Ok(())
}
