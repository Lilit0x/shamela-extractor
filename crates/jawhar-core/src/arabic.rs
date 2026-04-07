#![allow(dead_code, unused)]
/// Arabic character constants and classifications
/// Ported from PyArabic (https://github.com/linuxscout/pyarabic)
/// Reference: Unicode Arabic block U+0600–U+06FF
/// https://www.unicode.org/charts/PDF/U0600.pdf

// ===== Punctuation =====
pub const COMMA: char = '\u{060C}';
pub const SEMICOLON: char = '\u{061B}';
pub const QUESTION: char = '\u{061F}';

// ===== Hamza forms =====
pub const HAMZA: char = '\u{0621}';
pub const ALEF_MADDA: char = '\u{0622}';
pub const ALEF_HAMZA_ABOVE: char = '\u{0623}';
pub const WAW_HAMZA: char = '\u{0624}';
pub const ALEF_HAMZA_BELOW: char = '\u{0625}';
pub const YEH_HAMZA: char = '\u{0626}';

// ===== Core letters =====
pub const ALEF: char = '\u{0627}';
pub const BEH: char = '\u{0628}';
pub const TEH_MARBUTA: char = '\u{0629}';
pub const TEH: char = '\u{062A}';
pub const THEH: char = '\u{062B}';
pub const JEEM: char = '\u{062C}';
pub const HAH: char = '\u{062D}';
pub const KHAH: char = '\u{062E}';
pub const DAL: char = '\u{062F}';
pub const THAL: char = '\u{0630}';
pub const REH: char = '\u{0631}';
pub const ZAIN: char = '\u{0632}';
pub const SEEN: char = '\u{0633}';
pub const SHEEN: char = '\u{0634}';
pub const SAD: char = '\u{0635}';
pub const DAD: char = '\u{0636}';
pub const TAH: char = '\u{0637}';
pub const ZAH: char = '\u{0638}';
pub const AIN: char = '\u{0639}';
pub const GHAIN: char = '\u{063A}';
pub const TATWEEL: char = '\u{0640}';
pub const FEH: char = '\u{0641}';
pub const QAF: char = '\u{0642}';
pub const KAF: char = '\u{0643}';
pub const LAM: char = '\u{0644}';
pub const MEEM: char = '\u{0645}';
pub const NOON: char = '\u{0646}';
pub const HEH: char = '\u{0647}';
pub const WAW: char = '\u{0648}';
pub const ALEF_MAKSURA: char = '\u{0649}';
pub const YEH: char = '\u{064A}';

// ===== Diacritics (Tashkeel / Harakat) =====
pub const FATHATAN: char = '\u{064B}';
pub const DAMMATAN: char = '\u{064C}';
pub const KASRATAN: char = '\u{064D}';
pub const FATHA: char = '\u{064E}';
pub const DAMMA: char = '\u{064F}';
pub const KASRA: char = '\u{0650}';
pub const SHADDA: char = '\u{0651}';
pub const SUKUN: char = '\u{0652}';

// ===== Hamza modifiers =====
pub const MADDA_ABOVE: char = '\u{0653}';
pub const HAMZA_ABOVE: char = '\u{0654}';
pub const HAMZA_BELOW: char = '\u{0655}';

// ===== Small letters =====
pub const SMALL_ALEF: char = '\u{0670}';
pub const ALEF_WASLA: char = '\u{0671}';
pub const SMALL_WAW: char = '\u{06E5}';
pub const SMALL_YEH: char = '\u{06E6}';

// ===== Misc =====
pub const FULL_STOP: char = '\u{06D4}';
pub const BYTE_ORDER_MARK: char = '\u{FEFF}';

// ===== Eastern Arabic-Indic digits =====
pub const ZERO: char = '\u{0660}';
pub const ONE: char = '\u{0661}';
pub const TWO: char = '\u{0662}';
pub const THREE: char = '\u{0663}';
pub const FOUR: char = '\u{0664}';
pub const FIVE: char = '\u{0665}';
pub const SIX: char = '\u{0666}';
pub const SEVEN: char = '\u{0667}';
pub const EIGHT: char = '\u{0668}';
pub const NINE: char = '\u{0669}';

// ===== Extended Arabic-Indic digits (Persian/Urdu) =====
pub const ZERO_P: char = '\u{06F0}';
pub const ONE_P: char = '\u{06F1}';
pub const TWO_P: char = '\u{06F2}';
pub const THREE_P: char = '\u{06F3}';
pub const FOUR_P: char = '\u{06F4}';
pub const FIVE_P: char = '\u{06F5}';
pub const SIX_P: char = '\u{06F6}';
pub const SEVEN_P: char = '\u{06F7}';
pub const EIGHT_P: char = '\u{06F8}';
pub const NINE_P: char = '\u{06F9}';

// ===== Arabic punctuation =====
pub const PERCENT: char = '\u{066A}';
pub const DECIMAL: char = '\u{066B}';
pub const THOUSANDS: char = '\u{066C}';
pub const STAR: char = '\u{066D}';

// ===== Ligatures (presentation forms) =====
pub const LAM_ALEF: char = '\u{FEFB}';
pub const LAM_ALEF_HAMZA_ABOVE: char = '\u{FEF7}';
pub const LAM_ALEF_HAMZA_BELOW: char = '\u{FEF9}';
pub const LAM_ALEF_MADDA_ABOVE: char = '\u{FEF5}';

// ===== Character groups =====

/// All hamza forms
pub const HAMZAT: &[char] = &[
    HAMZA,
    WAW_HAMZA,
    YEH_HAMZA,
    HAMZA_ABOVE,
    HAMZA_BELOW,
    ALEF_HAMZA_BELOW,
    ALEF_HAMZA_ABOVE,
];

/// All alef-like letters
pub const ALEFAT: &[char] = &[
    ALEF,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    ALEF_HAMZA_BELOW,
    ALEF_WASLA,
    ALEF_MAKSURA,
    SMALL_ALEF,
];

/// Short vowel marks (without shadda)
pub const HARAKAT: &[char] = &[FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SUKUN];

/// Short harakat only (no tanwin)
pub const SHORT_HARAKAT: &[char] = &[FATHA, DAMMA, KASRA, SUKUN];

/// Tanwin marks
pub const TANWIN: &[char] = &[FATHATAN, DAMMATAN, KASRATAN];

/// All tashkeel marks (harakat + shadda)
pub const TASHKEEL: &[char] = &[
    FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SUKUN, SHADDA,
];

/// Small letter marks
pub const SMALL: &[char] = &[SMALL_ALEF, SMALL_WAW, SMALL_YEH];

/// Ligature forms (presentation forms that should be decomposed)
pub const LIGATURES: &[char] = &[
    LAM_ALEF,
    LAM_ALEF_HAMZA_ABOVE,
    LAM_ALEF_HAMZA_BELOW,
    LAM_ALEF_MADDA_ABOVE,
];

/// Weak letters (can change form)
pub const WEAK: &[char] = &[ALEF, WAW, YEH, ALEF_MAKSURA];

/// Yeh-like letters
pub const YEH_LIKE: &[char] = &[YEH, YEH_HAMZA, ALEF_MAKSURA, SMALL_YEH];

/// Waw-like letters
pub const WAW_LIKE: &[char] = &[WAW, WAW_HAMZA, SMALL_WAW];

/// Teh-like letters
pub const TEH_LIKE: &[char] = &[TEH, TEH_MARBUTA];

/// Moon letters (letters that don't assimilate with lam of the definite article)
pub const MOON: &[char] = &[
    HAMZA,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    ALEF_HAMZA_BELOW,
    ALEF,
    BEH,
    JEEM,
    HAH,
    KHAH,
    AIN,
    GHAIN,
    FEH,
    QAF,
    KAF,
    MEEM,
    HEH,
    WAW,
    YEH,
];

/// Sun letters (letters that assimilate with lam of the definite article)
pub const SUN: &[char] = &[
    TEH, THEH, DAL, THAL, REH, ZAIN, SEEN, SHEEN, SAD, DAD, TAH, ZAH, LAM, NOON,
];

/// Eastern Arabic-Indic digits
pub const EASTERN_DIGITS: &[char] = &[ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

/// Extended Arabic-Indic digits (Persian)
pub const PERSIAN_DIGITS: &[char] = &[
    ZERO_P, ONE_P, TWO_P, THREE_P, FOUR_P, FIVE_P, SIX_P, SEVEN_P, EIGHT_P, NINE_P,
];

/// All Arabic letters (no diacritics, no digits)
pub const LETTERS: &[char] = &[
    ALEF,
    BEH,
    TEH,
    TEH_MARBUTA,
    THEH,
    JEEM,
    HAH,
    KHAH,
    DAL,
    THAL,
    REH,
    ZAIN,
    SEEN,
    SHEEN,
    SAD,
    DAD,
    TAH,
    ZAH,
    AIN,
    GHAIN,
    FEH,
    QAF,
    KAF,
    LAM,
    MEEM,
    NOON,
    HEH,
    WAW,
    ALEF_MAKSURA,
    YEH,
    HAMZA,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    WAW_HAMZA,
    ALEF_HAMZA_BELOW,
    YEH_HAMZA,
];

// ===== Classification functions =====

/// Check if a character is an Arabic diacritic (tashkeel)
pub fn is_tashkeel(c: char) -> bool {
    TASHKEEL.contains(&c)
}

/// Check if a character is a haraka (vowel mark, no shadda)
pub fn is_haraka(c: char) -> bool {
    HARAKAT.contains(&c)
}

/// Check if a character is a tanwin mark
pub fn is_tanwin(c: char) -> bool {
    TANWIN.contains(&c)
}

/// Check if a character is shadda
pub fn is_shadda(c: char) -> bool {
    c == SHADDA
}

/// Check if a character is sukun
pub fn is_sukun(c: char) -> bool {
    c == SUKUN
}

/// Check if a character is tatweel (kashida)
pub fn is_tatweel(c: char) -> bool {
    c == TATWEEL
}

/// Check if a character is a hamza form
pub fn is_hamza(c: char) -> bool {
    HAMZAT.contains(&c)
}

/// Check if a character is an alef form
pub fn is_alef(c: char) -> bool {
    ALEFAT.contains(&c)
}

/// Check if a character is a yeh-like letter
pub fn is_yeh_like(c: char) -> bool {
    YEH_LIKE.contains(&c)
}

/// Check if a character is a waw-like letter
pub fn is_waw_like(c: char) -> bool {
    WAW_LIKE.contains(&c)
}

/// Check if a character is a weak letter
pub fn is_weak(c: char) -> bool {
    WEAK.contains(&c)
}

/// Check if a character is a sun letter
pub fn is_sun(c: char) -> bool {
    SUN.contains(&c)
}

/// Check if a character is a moon letter
pub fn is_moon(c: char) -> bool {
    MOON.contains(&c)
}

/// Check if a character is a ligature (presentation form)
pub fn is_ligature(c: char) -> bool {
    LIGATURES.contains(&c)
}

/// Check if a character is a small letter mark
pub fn is_small(c: char) -> bool {
    SMALL.contains(&c)
}

/// Check if a character is an Arabic letter (no diacritics, no digits)
pub fn is_letter(c: char) -> bool {
    LETTERS.contains(&c)
}

/// Check if a character is any Arabic diacritic mark
/// Covers the full range including Quranic marks
pub fn is_diacritic(c: char) -> bool {
    matches!(c,
        '\u{0610}'..='\u{061A}'  // Quranic marks (sign sallallahou etc.)
        | '\u{064B}'..='\u{065F}' // Core diacritics (fatha through wavy hamza below)
        | '\u{0670}'              // Small/superscript alef (dagger alef)
        | '\u{06D6}'..='\u{06DC}' // Quranic annotation marks
        | '\u{06DF}'..='\u{06E8}' // More Quranic marks
        | '\u{06EA}'..='\u{06ED}' // Additional Quranic marks
    )
}

/// Check if a character is an Eastern Arabic-Indic digit
pub fn is_eastern_digit(c: char) -> bool {
    EASTERN_DIGITS.contains(&c)
}

/// Check if a character is within the Arabic Unicode block
pub fn is_arabic(c: char) -> bool {
    matches!(c,
        '\u{0600}'..='\u{06FF}'   // Arabic
        | '\u{0750}'..='\u{077F}' // Arabic Supplement
        | '\u{08A0}'..='\u{08FF}' // Arabic Extended-A
        | '\u{FB50}'..='\u{FDFF}' // Arabic Presentation Forms-A
        | '\u{FE70}'..='\u{FEFF}' // Arabic Presentation Forms-B
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tashkeel() {
        assert!(is_tashkeel(FATHA));
        assert!(is_tashkeel(DAMMA));
        assert!(is_tashkeel(KASRA));
        assert!(is_tashkeel(SHADDA));
        assert!(is_tashkeel(SUKUN));
        assert!(is_tashkeel(FATHATAN));
        assert!(!is_tashkeel(ALEF));
        assert!(!is_tashkeel(BEH));
    }

    #[test]
    fn test_is_diacritic() {
        assert!(is_diacritic(FATHA));
        assert!(is_diacritic(SHADDA));
        assert!(is_diacritic(SMALL_ALEF));
        assert!(is_diacritic('\u{06D6}')); // Quranic mark
        assert!(!is_diacritic(ALEF));
        assert!(!is_diacritic('a'));
    }

    #[test]
    fn test_is_hamza() {
        assert!(is_hamza(HAMZA));
        assert!(is_hamza(WAW_HAMZA));
        assert!(is_hamza(YEH_HAMZA));
        assert!(is_hamza(ALEF_HAMZA_ABOVE));
        assert!(!is_hamza(ALEF));
        assert!(!is_hamza(WAW));
    }

    #[test]
    fn test_is_alef() {
        assert!(is_alef(ALEF));
        assert!(is_alef(ALEF_MADDA));
        assert!(is_alef(ALEF_HAMZA_ABOVE));
        assert!(is_alef(ALEF_HAMZA_BELOW));
        assert!(is_alef(ALEF_MAKSURA));
        assert!(!is_alef(BEH));
    }

    #[test]
    fn test_is_arabic() {
        assert!(is_arabic(ALEF));
        assert!(is_arabic(FATHA));
        assert!(is_arabic(LAM_ALEF)); // presentation form
        assert!(!is_arabic('a'));
        assert!(!is_arabic('1'));
    }

    #[test]
    fn test_is_letter() {
        assert!(is_letter(ALEF));
        assert!(is_letter(BEH));
        assert!(is_letter(YEH));
        assert!(is_letter(HAMZA));
        assert!(!is_letter(FATHA));
        assert!(!is_letter(SHADDA));
        assert!(!is_letter(TATWEEL));
    }

    #[test]
    fn test_sun_moon() {
        assert!(is_sun(SHEEN)); // الشَّمس
        assert!(is_sun(LAM));
        assert!(is_moon(QAF)); // القَمَر
        assert!(is_moon(KAF));
        assert!(!is_sun(ALEF));
        assert!(!is_moon(SHEEN));
    }
}
