use crate::CodePage;

pub struct Dos437;
pub struct Dos437Graphical;

impl CodePage for Dos437 {
    fn to_utf8(string: &Vec<u8>) -> String {
        let mut result = String::with_capacity(string.len() * 2);
        for c in string {
            result.push(match c {
                0..=0x7e => *c as char,
                0x7F => '\u{2302}',
                0x80 => '\u{00C7}',
                0x81 => '\u{00FC}',
                0x82 => '\u{00E9}',
                0x83 => '\u{00E2}',
                0x84 => '\u{00E4}',
                0x85 => '\u{00E0}',
                0x86 => '\u{00E5}',
                0x87 => '\u{00E7}',
                0x88 => '\u{00EA}',
                0x89 => '\u{00EB}',
                0x8A => '\u{00E8}',
                0x8B => '\u{00EF}',
                0x8C => '\u{00EE}',
                0x8D => '\u{00EC}',
                0x8E => '\u{00C4}',
                0x8F => '\u{00C5}',
                0x90 => '\u{00C9}',
                0x91 => '\u{00E6}',
                0x92 => '\u{00C6}',
                0x93 => '\u{00F4}',
                0x94 => '\u{00F6}',
                0x95 => '\u{00F2}',
                0x96 => '\u{00FB}',
                0x97 => '\u{00F9}',
                0x98 => '\u{00FF}',
                0x99 => '\u{00D6}',
                0x9A => '\u{00DC}',
                0x9B => '\u{00A2}',
                0x9C => '\u{00A3}',
                0x9D => '\u{00A5}',
                0x9E => '\u{20A7}',
                0x9F => '\u{0192}',
                0xA0 => '\u{00E1}',
                0xA1 => '\u{00ED}',
                0xA2 => '\u{00F3}',
                0xA3 => '\u{00FA}',
                0xA4 => '\u{00F1}',
                0xA5 => '\u{00D1}',
                0xA6 => '\u{00AA}',
                0xA7 => '\u{00BA}',
                0xA8 => '\u{00BF}',
                0xA9 => '\u{2310}',
                0xAA => '\u{00AC}',
                0xAB => '\u{00BD}',
                0xAC => '\u{00BC}',
                0xAD => '\u{00A1}',
                0xAE => '\u{00AB}',
                0xAF => '\u{00BB}',
                0xB0 => '\u{2591}',
                0xB1 => '\u{2592}',
                0xB2 => '\u{2593}',
                0xB3 => '\u{2502}',
                0xB4 => '\u{2524}',
                0xB5 => '\u{2561}',
                0xB6 => '\u{2562}',
                0xB7 => '\u{2556}',
                0xB8 => '\u{2555}',
                0xB9 => '\u{2563}',
                0xBA => '\u{2551}',
                0xBB => '\u{2557}',
                0xBC => '\u{255D}',
                0xBD => '\u{255C}',
                0xBE => '\u{255B}',
                0xBF => '\u{2510}',
                0xC0 => '\u{2514}',
                0xC1 => '\u{2534}',
                0xC2 => '\u{252C}',
                0xC3 => '\u{251C}',
                0xC4 => '\u{2500}',
                0xC5 => '\u{253C}',
                0xC6 => '\u{255E}',
                0xC7 => '\u{255F}',
                0xC8 => '\u{255A}',
                0xC9 => '\u{2554}',
                0xCA => '\u{2569}',
                0xCB => '\u{2566}',
                0xCC => '\u{2560}',
                0xCD => '\u{2550}',
                0xCE => '\u{256C}',
                0xCF => '\u{2567}',
                0xD0 => '\u{2568}',
                0xD1 => '\u{2564}',
                0xD2 => '\u{2565}',
                0xD3 => '\u{2559}',
                0xD4 => '\u{2558}',
                0xD5 => '\u{2552}',
                0xD6 => '\u{2553}',
                0xD7 => '\u{256B}',
                0xD8 => '\u{256A}',
                0xD9 => '\u{2518}',
                0xDA => '\u{250C}',
                0xDB => '\u{2588}',
                0xDC => '\u{2584}',
                0xDD => '\u{258C}',
                0xDE => '\u{2590}',
                0xDF => '\u{2580}',
                0xE0 => '\u{03B1}',
                0xE1 => '\u{00DF}',
                0xE2 => '\u{0393}',
                0xE3 => '\u{03C0}',
                0xE4 => '\u{03A3}',
                0xE5 => '\u{03C3}',
                0xE6 => '\u{00B5}',
                0xE7 => '\u{03C4}',
                0xE8 => '\u{03A6}',
                0xE9 => '\u{0398}',
                0xEA => '\u{03A9}',
                0xEB => '\u{03B4}',
                0xEC => '\u{221E}',
                0xED => '\u{03C6}',
                0xEE => '\u{03B5}',
                0xEF => '\u{2229}',
                0xF0 => '\u{2261}',
                0xF1 => '\u{00B1}',
                0xF2 => '\u{2265}',
                0xF3 => '\u{2264}',
                0xF4 => '\u{2320}',
                0xF5 => '\u{2321}',
                0xF6 => '\u{00F7}',
                0xF7 => '\u{2248}',
                0xF8 => '\u{00B0}',
                0xF9 => '\u{2219}',
                0xFA => '\u{00B7}',
                0xFB => '\u{221A}',
                0xFC => '\u{207F}',
                0xFD => '\u{00B2}',
                0xFE => '\u{25A0}',
                0xFF => '\u{00A0}',
            });
        }

        result
    }

    fn from_utf8(utf8: String) -> Vec<u8> {
        let mut result = Vec::with_capacity(utf8.len());

        for c in utf8.chars() {
            result.push(match c {
                ..='\u{007E}' => c as u8,
                '\u{2302}' => 0x7F,
                '\u{00C7}' => 0x80,
                '\u{00FC}' => 0x81,
                '\u{00E9}' => 0x82,
                '\u{00E2}' => 0x83,
                '\u{00E4}' => 0x84,
                '\u{00E0}' => 0x85,
                '\u{00E5}' => 0x86,
                '\u{00E7}' => 0x87,
                '\u{00EA}' => 0x88,
                '\u{00EB}' => 0x89,
                '\u{00E8}' => 0x8A,
                '\u{00EF}' => 0x8B,
                '\u{00EE}' => 0x8C,
                '\u{00EC}' => 0x8D,
                '\u{00C4}' => 0x8E,
                '\u{00C5}' => 0x8F,
                '\u{00C9}' => 0x90,
                '\u{00E6}' => 0x91,
                '\u{00C6}' => 0x92,
                '\u{00F4}' => 0x93,
                '\u{00F6}' => 0x94,
                '\u{00F2}' => 0x95,
                '\u{00FB}' => 0x96,
                '\u{00F9}' => 0x97,
                '\u{00FF}' => 0x98,
                '\u{00D6}' => 0x99,
                '\u{00DC}' => 0x9A,
                '\u{00A2}' => 0x9B,
                '\u{00A3}' => 0x9C,
                '\u{00A5}' => 0x9D,
                '\u{20A7}' => 0x9E,
                '\u{0192}' => 0x9F,
                '\u{00E1}' => 0xA0,
                '\u{00ED}' => 0xA1,
                '\u{00F3}' => 0xA2,
                '\u{00FA}' => 0xA3,
                '\u{00F1}' => 0xA4,
                '\u{00D1}' => 0xA5,
                '\u{00AA}' => 0xA6,
                '\u{00BA}' => 0xA7,
                '\u{00BF}' => 0xA8,
                '\u{2310}' => 0xA9,
                '\u{00AC}' => 0xAA,
                '\u{00BD}' => 0xAB,
                '\u{00BC}' => 0xAC,
                '\u{00A1}' => 0xAD,
                '\u{00AB}' => 0xAE,
                '\u{00BB}' => 0xAF,
                '\u{2591}' => 0xB0,
                '\u{2592}' => 0xB1,
                '\u{2593}' => 0xB2,
                '\u{2502}' => 0xB3,
                '\u{2524}' => 0xB4,
                '\u{2561}' => 0xB5,
                '\u{2562}' => 0xB6,
                '\u{2556}' => 0xB7,
                '\u{2555}' => 0xB8,
                '\u{2563}' => 0xB9,
                '\u{2551}' => 0xBA,
                '\u{2557}' => 0xBB,
                '\u{255D}' => 0xBC,
                '\u{255C}' => 0xBD,
                '\u{255B}' => 0xBE,
                '\u{2510}' => 0xBF,
                '\u{2514}' => 0xC0,
                '\u{2534}' => 0xC1,
                '\u{252C}' => 0xC2,
                '\u{251C}' => 0xC3,
                '\u{2500}' => 0xC4,
                '\u{253C}' => 0xC5,
                '\u{255E}' => 0xC6,
                '\u{255F}' => 0xC7,
                '\u{255A}' => 0xC8,
                '\u{2554}' => 0xC9,
                '\u{2569}' => 0xCA,
                '\u{2566}' => 0xCB,
                '\u{2560}' => 0xCC,
                '\u{2550}' => 0xCD,
                '\u{256C}' => 0xCE,
                '\u{2567}' => 0xCF,
                '\u{2568}' => 0xD0,
                '\u{2564}' => 0xD1,
                '\u{2565}' => 0xD2,
                '\u{2559}' => 0xD3,
                '\u{2558}' => 0xD4,
                '\u{2552}' => 0xD5,
                '\u{2553}' => 0xD6,
                '\u{256B}' => 0xD7,
                '\u{256A}' => 0xD8,
                '\u{2518}' => 0xD9,
                '\u{250C}' => 0xDA,
                '\u{2588}' => 0xDB,
                '\u{2584}' => 0xDC,
                '\u{258C}' => 0xDD,
                '\u{2590}' => 0xDE,
                '\u{2580}' => 0xDF,
                '\u{03B1}' => 0xE0,
                '\u{00DF}' => 0xE1,
                '\u{0393}' => 0xE2,
                '\u{03C0}' => 0xE3,
                '\u{03A3}' => 0xE4,
                '\u{03C3}' => 0xE5,
                '\u{00B5}' => 0xE6,
                '\u{03C4}' => 0xE7,
                '\u{03A6}' => 0xE8,
                '\u{0398}' => 0xE9,
                '\u{03A9}' => 0xEA,
                '\u{03B4}' => 0xEB,
                '\u{221E}' => 0xEC,
                '\u{03C6}' => 0xED,
                '\u{03B5}' => 0xEE,
                '\u{2229}' => 0xEF,
                '\u{2261}' => 0xF0,
                '\u{00B1}' => 0xF1,
                '\u{2265}' => 0xF2,
                '\u{2264}' => 0xF3,
                '\u{2320}' => 0xF4,
                '\u{2321}' => 0xF5,
                '\u{00F7}' => 0xF6,
                '\u{2248}' => 0xF7,
                '\u{00B0}' => 0xF8,
                '\u{2219}' => 0xF9,
                '\u{00B7}' => 0xFA,
                '\u{221A}' => 0xFB,
                '\u{207F}' => 0xFC,
                '\u{00B2}' => 0xFD,
                '\u{25A0}' => 0xFE,
                '\u{00A0}' => 0xFF,
                _ => 0x3F, // '\u{003F}' == '?'
            });
        }

        result
    }
}

impl CodePage for Dos437Graphical {
    fn to_utf8(string: &Vec<u8>) -> String {
        let mut result = String::with_capacity(string.len() * 2);
        for c in string {
            result.push(match c {
                0x00 => '\0',
                0x01 => '\u{263A}',
                0x02 => '\u{263B}',
                0x03 => '\u{2665}',
                0x04 => '\u{2666}',
                0x05 => '\u{2663}',
                0x06 => '\u{2660}',
                0x07 => '\u{2022}',
                0x08 => '\u{25D8}',
                0x09 => '\u{25CB}',
                0x0A => '\u{25D9}',
                0x0B => '\u{2642}',
                0x0C => '\u{2640}',
                0x0D => '\u{266A}',
                0x0E => '\u{266B}',
                0x0F => '\u{263C}',
                0x10 => '\u{25BA}',
                0x11 => '\u{25C4}',
                0x12 => '\u{2195}',
                0x13 => '\u{203C}',
                0x14 => '\u{00B6}',
                0x15 => '\u{00A7}',
                0x16 => '\u{25AC}',
                0x17 => '\u{21A8}',
                0x18 => '\u{2191}',
                0x19 => '\u{2193}',
                0x1A => '\u{2192}',
                0x1B => '\u{2190}',
                0x1C => '\u{221F}',
                0x1D => '\u{2194}',
                0x1E => '\u{25B2}',
                0x1F => '\u{25BC}',
                0x20..=0x7e => *c as char,
                0x7F => '\u{2302}',
                0x80 => '\u{00C7}',
                0x81 => '\u{00FC}',
                0x82 => '\u{00E9}',
                0x83 => '\u{00E2}',
                0x84 => '\u{00E4}',
                0x85 => '\u{00E0}',
                0x86 => '\u{00E5}',
                0x87 => '\u{00E7}',
                0x88 => '\u{00EA}',
                0x89 => '\u{00EB}',
                0x8A => '\u{00E8}',
                0x8B => '\u{00EF}',
                0x8C => '\u{00EE}',
                0x8D => '\u{00EC}',
                0x8E => '\u{00C4}',
                0x8F => '\u{00C5}',
                0x90 => '\u{00C9}',
                0x91 => '\u{00E6}',
                0x92 => '\u{00C6}',
                0x93 => '\u{00F4}',
                0x94 => '\u{00F6}',
                0x95 => '\u{00F2}',
                0x96 => '\u{00FB}',
                0x97 => '\u{00F9}',
                0x98 => '\u{00FF}',
                0x99 => '\u{00D6}',
                0x9A => '\u{00DC}',
                0x9B => '\u{00A2}',
                0x9C => '\u{00A3}',
                0x9D => '\u{00A5}',
                0x9E => '\u{20A7}',
                0x9F => '\u{0192}',
                0xA0 => '\u{00E1}',
                0xA1 => '\u{00ED}',
                0xA2 => '\u{00F3}',
                0xA3 => '\u{00FA}',
                0xA4 => '\u{00F1}',
                0xA5 => '\u{00D1}',
                0xA6 => '\u{00AA}',
                0xA7 => '\u{00BA}',
                0xA8 => '\u{00BF}',
                0xA9 => '\u{2310}',
                0xAA => '\u{00AC}',
                0xAB => '\u{00BD}',
                0xAC => '\u{00BC}',
                0xAD => '\u{00A1}',
                0xAE => '\u{00AB}',
                0xAF => '\u{00BB}',
                0xB0 => '\u{2591}',
                0xB1 => '\u{2592}',
                0xB2 => '\u{2593}',
                0xB3 => '\u{2502}',
                0xB4 => '\u{2524}',
                0xB5 => '\u{2561}',
                0xB6 => '\u{2562}',
                0xB7 => '\u{2556}',
                0xB8 => '\u{2555}',
                0xB9 => '\u{2563}',
                0xBA => '\u{2551}',
                0xBB => '\u{2557}',
                0xBC => '\u{255D}',
                0xBD => '\u{255C}',
                0xBE => '\u{255B}',
                0xBF => '\u{2510}',
                0xC0 => '\u{2514}',
                0xC1 => '\u{2534}',
                0xC2 => '\u{252C}',
                0xC3 => '\u{251C}',
                0xC4 => '\u{2500}',
                0xC5 => '\u{253C}',
                0xC6 => '\u{255E}',
                0xC7 => '\u{255F}',
                0xC8 => '\u{255A}',
                0xC9 => '\u{2554}',
                0xCA => '\u{2569}',
                0xCB => '\u{2566}',
                0xCC => '\u{2560}',
                0xCD => '\u{2550}',
                0xCE => '\u{256C}',
                0xCF => '\u{2567}',
                0xD0 => '\u{2568}',
                0xD1 => '\u{2564}',
                0xD2 => '\u{2565}',
                0xD3 => '\u{2559}',
                0xD4 => '\u{2558}',
                0xD5 => '\u{2552}',
                0xD6 => '\u{2553}',
                0xD7 => '\u{256B}',
                0xD8 => '\u{256A}',
                0xD9 => '\u{2518}',
                0xDA => '\u{250C}',
                0xDB => '\u{2588}',
                0xDC => '\u{2584}',
                0xDD => '\u{258C}',
                0xDE => '\u{2590}',
                0xDF => '\u{2580}',
                0xE0 => '\u{03B1}',
                0xE1 => '\u{00DF}',
                0xE2 => '\u{0393}',
                0xE3 => '\u{03C0}',
                0xE4 => '\u{03A3}',
                0xE5 => '\u{03C3}',
                0xE6 => '\u{00B5}',
                0xE7 => '\u{03C4}',
                0xE8 => '\u{03A6}',
                0xE9 => '\u{0398}',
                0xEA => '\u{03A9}',
                0xEB => '\u{03B4}',
                0xEC => '\u{221E}',
                0xED => '\u{03C6}',
                0xEE => '\u{03B5}',
                0xEF => '\u{2229}',
                0xF0 => '\u{2261}',
                0xF1 => '\u{00B1}',
                0xF2 => '\u{2265}',
                0xF3 => '\u{2264}',
                0xF4 => '\u{2320}',
                0xF5 => '\u{2321}',
                0xF6 => '\u{00F7}',
                0xF7 => '\u{2248}',
                0xF8 => '\u{00B0}',
                0xF9 => '\u{2219}',
                0xFA => '\u{00B7}',
                0xFB => '\u{221A}',
                0xFC => '\u{207F}',
                0xFD => '\u{00B2}',
                0xFE => '\u{25A0}',
                0xFF => '\u{00A0}',
            });
        }

        result
    }

    fn from_utf8(utf8: String) -> Vec<u8> {
        let mut result = Vec::with_capacity(utf8.len());

        for c in utf8.chars() {
            result.push(match c {
                '\0' => 0x00,
                '\u{263A}' => 0x01,
                '\u{263B}' => 0x02,
                '\u{2665}' => 0x03,
                '\u{2666}' => 0x04,
                '\u{2663}' => 0x05,
                '\u{2660}' => 0x06,
                '\u{2022}' => 0x07,
                '\u{25D8}' => 0x08,
                '\u{25CB}' => 0x09,
                '\u{25D9}' => 0x0A,
                '\u{2642}' => 0x0B,
                '\u{2640}' => 0x0C,
                '\u{266A}' => 0x0D,
                '\u{266B}' => 0x0E,
                '\u{263C}' => 0x0F,
                '\u{25BA}' => 0x10,
                '\u{25C4}' => 0x11,
                '\u{2195}' => 0x12,
                '\u{203C}' => 0x13,
                '\u{00B6}' => 0x14,
                '\u{00A7}' => 0x15,
                '\u{25AC}' => 0x16,
                '\u{21A8}' => 0x17,
                '\u{2191}' => 0x18,
                '\u{2193}' => 0x19,
                '\u{2192}' => 0x1A,
                '\u{2190}' => 0x1B,
                '\u{221F}' => 0x1C,
                '\u{2194}' => 0x1D,
                '\u{25B2}' => 0x1E,
                '\u{25BC}' => 0x1F,
                ' '..='\u{007E}' => c as u8,
                '\u{2302}' => 0x7F,
                '\u{00C7}' => 0x80,
                '\u{00FC}' => 0x81,
                '\u{00E9}' => 0x82,
                '\u{00E2}' => 0x83,
                '\u{00E4}' => 0x84,
                '\u{00E0}' => 0x85,
                '\u{00E5}' => 0x86,
                '\u{00E7}' => 0x87,
                '\u{00EA}' => 0x88,
                '\u{00EB}' => 0x89,
                '\u{00E8}' => 0x8A,
                '\u{00EF}' => 0x8B,
                '\u{00EE}' => 0x8C,
                '\u{00EC}' => 0x8D,
                '\u{00C4}' => 0x8E,
                '\u{00C5}' => 0x8F,
                '\u{00C9}' => 0x90,
                '\u{00E6}' => 0x91,
                '\u{00C6}' => 0x92,
                '\u{00F4}' => 0x93,
                '\u{00F6}' => 0x94,
                '\u{00F2}' => 0x95,
                '\u{00FB}' => 0x96,
                '\u{00F9}' => 0x97,
                '\u{00FF}' => 0x98,
                '\u{00D6}' => 0x99,
                '\u{00DC}' => 0x9A,
                '\u{00A2}' => 0x9B,
                '\u{00A3}' => 0x9C,
                '\u{00A5}' => 0x9D,
                '\u{20A7}' => 0x9E,
                '\u{0192}' => 0x9F,
                '\u{00E1}' => 0xA0,
                '\u{00ED}' => 0xA1,
                '\u{00F3}' => 0xA2,
                '\u{00FA}' => 0xA3,
                '\u{00F1}' => 0xA4,
                '\u{00D1}' => 0xA5,
                '\u{00AA}' => 0xA6,
                '\u{00BA}' => 0xA7,
                '\u{00BF}' => 0xA8,
                '\u{2310}' => 0xA9,
                '\u{00AC}' => 0xAA,
                '\u{00BD}' => 0xAB,
                '\u{00BC}' => 0xAC,
                '\u{00A1}' => 0xAD,
                '\u{00AB}' => 0xAE,
                '\u{00BB}' => 0xAF,
                '\u{2591}' => 0xB0,
                '\u{2592}' => 0xB1,
                '\u{2593}' => 0xB2,
                '\u{2502}' => 0xB3,
                '\u{2524}' => 0xB4,
                '\u{2561}' => 0xB5,
                '\u{2562}' => 0xB6,
                '\u{2556}' => 0xB7,
                '\u{2555}' => 0xB8,
                '\u{2563}' => 0xB9,
                '\u{2551}' => 0xBA,
                '\u{2557}' => 0xBB,
                '\u{255D}' => 0xBC,
                '\u{255C}' => 0xBD,
                '\u{255B}' => 0xBE,
                '\u{2510}' => 0xBF,
                '\u{2514}' => 0xC0,
                '\u{2534}' => 0xC1,
                '\u{252C}' => 0xC2,
                '\u{251C}' => 0xC3,
                '\u{2500}' => 0xC4,
                '\u{253C}' => 0xC5,
                '\u{255E}' => 0xC6,
                '\u{255F}' => 0xC7,
                '\u{255A}' => 0xC8,
                '\u{2554}' => 0xC9,
                '\u{2569}' => 0xCA,
                '\u{2566}' => 0xCB,
                '\u{2560}' => 0xCC,
                '\u{2550}' => 0xCD,
                '\u{256C}' => 0xCE,
                '\u{2567}' => 0xCF,
                '\u{2568}' => 0xD0,
                '\u{2564}' => 0xD1,
                '\u{2565}' => 0xD2,
                '\u{2559}' => 0xD3,
                '\u{2558}' => 0xD4,
                '\u{2552}' => 0xD5,
                '\u{2553}' => 0xD6,
                '\u{256B}' => 0xD7,
                '\u{256A}' => 0xD8,
                '\u{2518}' => 0xD9,
                '\u{250C}' => 0xDA,
                '\u{2588}' => 0xDB,
                '\u{2584}' => 0xDC,
                '\u{258C}' => 0xDD,
                '\u{2590}' => 0xDE,
                '\u{2580}' => 0xDF,
                '\u{03B1}' => 0xE0,
                '\u{00DF}' => 0xE1,
                '\u{0393}' => 0xE2,
                '\u{03C0}' => 0xE3,
                '\u{03A3}' => 0xE4,
                '\u{03C3}' => 0xE5,
                '\u{00B5}' => 0xE6,
                '\u{03C4}' => 0xE7,
                '\u{03A6}' => 0xE8,
                '\u{0398}' => 0xE9,
                '\u{03A9}' => 0xEA,
                '\u{03B4}' => 0xEB,
                '\u{221E}' => 0xEC,
                '\u{03C6}' => 0xED,
                '\u{03B5}' => 0xEE,
                '\u{2229}' => 0xEF,
                '\u{2261}' => 0xF0,
                '\u{00B1}' => 0xF1,
                '\u{2265}' => 0xF2,
                '\u{2264}' => 0xF3,
                '\u{2320}' => 0xF4,
                '\u{2321}' => 0xF5,
                '\u{00F7}' => 0xF6,
                '\u{2248}' => 0xF7,
                '\u{00B0}' => 0xF8,
                '\u{2219}' => 0xF9,
                '\u{00B7}' => 0xFA,
                '\u{221A}' => 0xFB,
                '\u{207F}' => 0xFC,
                '\u{00B2}' => 0xFD,
                '\u{25A0}' => 0xFE,
                '\u{00A0}' => 0xFF,
                _ => 0x3F, // '\u{003F}' == '?'
            });
        }

        result
    }
}
