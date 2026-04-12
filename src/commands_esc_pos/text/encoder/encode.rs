use encoding_rs::Encoding;
use serde::{Deserialize, Serialize};

macro_rules! define_encodes {
    ($($variant:ident => ($label:literal, $encoding:ident)),* $(,)?) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
        pub enum Encode {
            $(
                #[serde(rename = $label)]
                $variant,
            )*
            #[default]
            #[serde(rename = "ACCENT_REMOVER")]
            AccentRemover,
        }

        impl Encode {
            pub fn encoding(self) -> Option<&'static Encoding> {
                match self {
                    $(Self::$variant => Some(encoding_rs::$encoding),)*
                    Self::AccentRemover => None,
                }
            }

            pub fn label(self) -> &'static str {
                match self {
                    $(Self::$variant => $label,)*
                    Self::AccentRemover => "ACCENT_REMOVER",
                }
            }
        }
    };
}

define_encodes! {
    Big5 => ("BIG5", BIG5),
    EucJp => ("EUC_JP", EUC_JP),
    EucKr => ("EUC_KR", EUC_KR),
    Gbk => ("GBK", GBK),
    Ibm866 => ("IBM866", IBM866),
    Iso2022Jp => ("ISO_2022_JP", ISO_2022_JP),
    Iso885910 => ("ISO_8859_10", ISO_8859_10),
    Iso885913 => ("ISO_8859_13", ISO_8859_13),
    Iso885914 => ("ISO_8859_14", ISO_8859_14),
    Iso885915 => ("ISO_8859_15", ISO_8859_15),
    Iso885916 => ("ISO_8859_16", ISO_8859_16),
    Iso88592 => ("ISO_8859_2", ISO_8859_2),
    Iso88593 => ("ISO_8859_3", ISO_8859_3),
    Iso88594 => ("ISO_8859_4", ISO_8859_4),
    Iso88595 => ("ISO_8859_5", ISO_8859_5),
    Iso88596 => ("ISO_8859_6", ISO_8859_6),
    Iso88597 => ("ISO_8859_7", ISO_8859_7),
    Iso88598 => ("ISO_8859_8", ISO_8859_8),
    Iso88598I => ("ISO_8859_8_I", ISO_8859_8_I),
    Koi8R => ("KOI8_R", KOI8_R),
    Koi8U => ("KOI8_U", KOI8_U),
    ShiftJis => ("SHIFT_JIS", SHIFT_JIS),
    Utf16Be => ("UTF_16BE", UTF_16BE),
    Utf16Le => ("UTF_16LE", UTF_16LE),
    Utf8 => ("UTF_8", UTF_8),
    Gb18030 => ("GB18030", GB18030),
    Macintosh => ("MACINTOSH", MACINTOSH),
    Replacement => ("REPLACEMENT", REPLACEMENT),
    Windows1250 => ("WINDOWS_1250", WINDOWS_1250),
    Windows1251 => ("WINDOWS_1251", WINDOWS_1251),
    Windows1252 => ("WINDOWS_1252", WINDOWS_1252),
    Windows1253 => ("WINDOWS_1253", WINDOWS_1253),
    Windows1254 => ("WINDOWS_1254", WINDOWS_1254),
    Windows1255 => ("WINDOWS_1255", WINDOWS_1255),
    Windows1256 => ("WINDOWS_1256", WINDOWS_1256),
    Windows1257 => ("WINDOWS_1257", WINDOWS_1257),
    Windows1258 => ("WINDOWS_1258", WINDOWS_1258),
    Windows874 => ("WINDOWS_874", WINDOWS_874),
    XMacCyrillic => ("X_MAC_CYRILLIC", X_MAC_CYRILLIC),
    XUserDefined => ("X_USER_DEFINED", X_USER_DEFINED),
}
