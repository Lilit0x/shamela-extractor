use crate::arabic::*;

pub fn strip_diacritics(text: &str) -> String {
    text.chars().filter(|c| !is_diacritic(*c)).collect()
}

pub fn normalize_hamza(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            ALEF_MADDA | ALEF_HAMZA_ABOVE | ALEF_HAMZA_BELOW => ALEF,
            WAW_HAMZA => WAW,
            YEH_HAMZA => YEH,
            _ => c,
        })
        .collect()
}

pub fn strip_tatweel(text: &str) -> String {
    text.chars().filter(|c| !is_tatweel(*c)).collect()
}

pub fn normalize_ligatures(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            LAM_ALEF => {
                result.push(LAM);
                result.push(ALEF);
            }
            LAM_ALEF_HAMZA_ABOVE => {
                result.push(LAM);
                result.push(ALEF);
            }
            LAM_ALEF_HAMZA_BELOW => {
                result.push(LAM);
                result.push(ALEF);
            }
            LAM_ALEF_MADDA_ABOVE => {
                result.push(LAM);
                result.push(ALEF);
            }
            _ => result.push(c),
        }
    }
    result
}

pub fn normalize_alif_maqsurah(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            ALEF_MAKSURA => YEH,
            _ => c,
        })
        .collect()
}

pub fn strip_html(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut inside_tag = false;
    for c in text.chars() {
        match c {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => result.push(c),
            _ => {}
        }
    }
    result
}

pub fn normalize(text: &str) -> String {
    let text = strip_html(text);
    let text = normalize_ligatures(&text);
    let text = strip_diacritics(&text);
    let text = normalize_hamza(&text);
    let text = normalize_alif_maqsurah(&text);
    let text = strip_tatweel(&text);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_diacritics() {
        assert_eq!(strip_diacritics("أَبَأَ"), "أبأ");
        assert_eq!(strip_diacritics("الْكِتَابُ"), "الكتاب");
    }

    #[test]
    fn test_normalize_hamza() {
        assert_eq!(normalize_hamza("إسلام"), "اسلام");
        assert_eq!(normalize_hamza("أمّة"), "امّة");
        assert_eq!(normalize_hamza("آمن"), "امن");
    }

    #[test]
    fn test_normalize_ligatures() {
        let lam_alef = String::from(LAM_ALEF);
        assert_eq!(normalize_ligatures(&lam_alef), "لا");
    }

    #[test]
    fn test_strip_html() {
        assert_eq!(
            strip_html("<span data-type=\"title\">فصل الهمزة</span>"),
            "فصل الهمزة"
        );
        assert_eq!(strip_html("no tags here"), "no tags here");
        assert_eq!(
            strip_html("<b>bold</b> and <i>italic</i>"),
            "bold and italic"
        );
    }

    #[test]
    fn test_normalize_alif_maqsurah() {
        assert_eq!(normalize_alif_maqsurah("موسى"), "موسي");
        assert_eq!(normalize_alif_maqsurah("على"), "علي");
    }

    #[test]
    fn test_full_normalize() {
        assert_eq!(normalize("أَبَأَ"), "ابا");
        assert_eq!(normalize("الْكِتَابُ"), "الكتاب");
    }
}
