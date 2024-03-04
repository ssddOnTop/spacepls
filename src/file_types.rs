use std::fmt::{Display, Formatter, Write};

/// High-level Types of files that can be uploaded and queried.
pub enum Types {
    Any,
    Documents(Docs),
    Spreadsheets(Sheets),
    Presentations(Presentations),
    Images(Images),
    Videos(Videos),
    Audios(Audios),
    Archives(Archives),
    Forms,
    Folders,
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Any => f.write_str("any"),
            Types::Documents(d) => d.fmt(f),
            Types::Spreadsheets(s) => s.fmt(f),
            Types::Presentations(p) => p.fmt(f),
            Types::Images(i) => i.fmt(f),
            Types::Videos(v) => v.fmt(f),
            Types::Audios(a) => a.fmt(f),
            Types::Archives(a) => a.fmt(f),
            Types::Forms => f.write_str("forms"),
            Types::Folders => f.write_str("folders"),
        }
    }
}

pub enum Images {
    Any,
    JPG,
    JPEG,
    PNG,
    GIF,
    TIFF,
    BMP,
    SVG,
}

impl Display for Images {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Images::JPG => f.write_str("jpg"),
            Images::JPEG => f.write_str("jpeg"),
            Images::PNG => f.write_str("png"),
            Images::GIF => f.write_str("gif"),
            Images::TIFF => f.write_str("tiff"),
            Images::BMP => f.write_str("bmp"),
            Images::SVG => f.write_str("svg"),
            Images::Any => f.write_str("any"),
        }
    }
}

// For Videos
pub enum Videos {
    Any,
    MP4,
    AVI,
    MOV,
    WMV,
    FLV,
    MKV,
}

impl Display for Videos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Videos::MP4 => f.write_str("mp4"),
            Videos::AVI => f.write_str("avi"),
            Videos::MOV => f.write_str("mov"),
            Videos::WMV => f.write_str("wmv"),
            Videos::FLV => f.write_str("flv"),
            Videos::MKV => f.write_str("mkv"),
            Videos::Any => f.write_str("any"),
        }
    }
}

// For Audios
pub enum Audios {
    Any,
    MP3,
    WAV,
    AAC,
    FLAC,
    OGG,
}

impl Display for Audios {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Audios::MP3 => f.write_str("mp3"),
            Audios::WAV => f.write_str("wav"),
            Audios::AAC => f.write_str("aac"),
            Audios::FLAC => f.write_str("flac"),
            Audios::OGG => f.write_str("ogg"),
            Audios::Any => f.write_str("any"),
        }
    }
}

// For Archives
pub enum Archives {
    Any,
    ZIP,
    RAR,
    TAR,
    GZ,
    _7Z,
}

impl Display for Archives {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Archives::ZIP => f.write_str("zip"),
            Archives::RAR => f.write_str("rar"),
            Archives::TAR => f.write_str("tar"),
            Archives::GZ => f.write_str("gz"),
            Archives::_7Z => f.write_str("7z"),
            Archives::Any => f.write_str("any"),
        }
    }
}

pub enum Presentations {
    Any,
    Gslide,
    Ppt,
    Pptx,
    ODP,
    PDF,
}

impl Display for Presentations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Presentations::Gslide => f.write_str("gslide"),
            Presentations::Ppt => f.write_str("ppt"),
            Presentations::Pptx => f.write_str("pptx"),
            Presentations::ODP => f.write_str("odp"),
            Presentations::PDF => f.write_str("pdf"),
            Presentations::Any => f.write_str("any"),
        }
    }
}

pub enum Sheets {
    Any,
    Gsheet,
    Xls,
    Xlsx,
    ODS,
    CSV,
    TSV,
    PDF,
}

impl Display for Sheets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sheets::Gsheet => f.write_str("gsheet"),
            Sheets::Xls => f.write_str("xls"),
            Sheets::Xlsx => f.write_str("xlsx"),
            Sheets::ODS => f.write_str("ods"),
            Sheets::CSV => f.write_str("csv"),
            Sheets::TSV => f.write_str("tsv"),
            Sheets::PDF => f.write_str("pdf"),
            Sheets::Any => f.write_str("any"),
        }
    }
}

pub enum Docs {
    Any,
    Gdoc,
    Doc,
    Docx,
    RTF,
    TXT,
    ODT,
    PDF,
    HTML,
    HTM,
}
impl Display for Docs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Docs::Gdoc => f.write_str("gdoc"),
            Docs::Doc => f.write_str("doc"),
            Docs::Docx => f.write_str("docx"),
            Docs::RTF => f.write_str("rtf"),
            Docs::TXT => f.write_str("txt"),
            Docs::ODT => f.write_str("odt"),
            Docs::PDF => f.write_str("pdf"),
            Docs::HTML => f.write_str("html"),
            Docs::HTM => f.write_str("htm"),
            Docs::Any => f.write_str("any"),
        }
    }
}
