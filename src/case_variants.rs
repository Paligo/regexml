use ahash::{HashMap, HashMapExt};

pub(crate) const ROMAN_VARIANTS: [char; 4] = ['\u{0130}', '\u{0131}', '\u{212A}', '\u{017F}'];

pub(crate) struct CaseVariants {
    // Use one hashmap for characters with a single case variant, another for
    // characters with multiple case variants, to reduce the number of objects
    // that need to be allocated
    mono_variants: HashMap<char, char>,
    poly_variants: HashMap<char, Vec<char>>,
}

impl CaseVariants {
    pub(crate) fn empty() -> Self {
        Self {
            mono_variants: HashMap::new(),
            poly_variants: HashMap::new(),
        }
    }

    pub(crate) fn get_case_variants(&self, code: char) -> Vec<char> {
        let mono = self.mono_variants.get(&code);
        if let Some(mono) = mono {
            vec![*mono]
        } else {
            let poly = self.poly_variants.get(&code);
            if let Some(poly) = poly {
                poly.clone()
            } else {
                vec![]
            }
        }
    }
}
