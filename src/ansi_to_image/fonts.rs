use std::fmt::{self, Display, Formatter};

use include_flate::flate;

#[derive(Debug)]
pub enum FontFamily {
    #[cfg(feature = "font-iosevka_term")]
    IosevkaTerm,
    #[cfg(feature = "font-anonymous_pro")]
    AnonymousPro,
    #[cfg(feature = "font-source_code_pro")]
    SourceCodePro,
    #[cfg(feature = "font-ubuntu")]
    Ubuntu,
    Custom(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>),
}

impl FontFamily {
    pub fn from_name(name: String) -> Option<Self> {
        for item in Self::list().into_iter() {
            if item.to_string() == name {
                return Some(item);
            }
        }
        None
    }
    pub fn list() -> Vec<Self> {
        let mut result = vec![];
        #[cfg(feature = "font-iosevka_term")]
        result.push(Self::IosevkaTerm);
        #[cfg(feature = "font-anonymous_pro")]
        result.push(Self::AnonymousPro);
        #[cfg(feature = "font-source_code_pro")]
        result.push(Self::SourceCodePro);
        #[cfg(feature = "font-ubuntu")]
        result.push(Self::Ubuntu);
        result
    }

    pub fn regular(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Medium.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro.ttf");
                &DATA
            }
            #[cfg(feature = "font-source_code_pro")]
            Self::SourceCodePro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/SourceCodePro/SourceCodePro-Regular.otf");
                &DATA
            }
            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-R.ttf");
                &DATA
            }
            Self::Custom(regular, _bold, _italic, _bold_italic) => regular,
        }
    }
    pub fn bold(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Bold.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Bold.ttf");
                &DATA
            }
            #[cfg(feature = "font-source_code_pro")]
            Self::SourceCodePro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/SourceCodePro/SourceCodePro-Bold.otf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-B.ttf");
                &DATA
            }
            Self::Custom(_regular, bold, _italic, _bold_italic) => bold,
        }
    }
    pub fn italic(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Italic.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Italic.ttf");
                &DATA
            }

            #[cfg(feature = "font-source_code_pro")]
            Self::SourceCodePro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/SourceCodePro/SourceCodePro-It.otf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-RI.ttf");
                &DATA
            }
            Self::Custom(_regular, _bold, italic, _bold_italic) => italic,
        }
    }
    pub fn bold_italic(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-BoldItalic.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Bold_Italic.ttf");
                &DATA
            }
            #[cfg(feature = "font-source_code_pro")]
            Self::SourceCodePro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/SourceCodePro/SourceCodePro-BoldIt.otf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-BI.ttf");
                &DATA
            }
            Self::Custom(_regular, _bold, _italic, bold_italic) => bold_italic,
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        #[cfg(feature = "font-iosevka_term")]
        return Self::IosevkaTerm;
        #[cfg(feature = "font-anonymous_pro")]
        return Self::AnonymousPro;
        #[cfg(feature = "font-camingo_code")]
        return Self::CamingoCode;
        #[cfg(feature = "font-ubuntu")]
        return Self::Ubuntu;
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => "IosevkaTerm",
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => "AnonymousPro",
            #[cfg(feature = "font-source_code_pro")]
            Self::SourceCodePro => "SourceCodePro",
            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => "Ubuntu",
            _ => "Custom",
        };
        write!(f, "{}", name)
    }
}

// fn load_file(path: String) -> Option<Vec<u8>> {
//     let data = &mut File::open(path);
//     if let Ok(file) = data {
//         let buffer = &mut Vec::new();
//         file.read_to_end(buffer);
//         return Some(buffer.to_owned());
//     } else if let Err(err) = data {
//         eprintln!("{}, cannot open file: {}", path, err);
//     }

//     None
// }
