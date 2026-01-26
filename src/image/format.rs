/** 用来获取或者读取图片的元信息 判断图片类型

*/

#[derive(Copy, Clone)]
pub enum ImageFormat {
    Png,
    Jpg,
    Gif,
    Unknow,
}

impl ImageFormat {
    //PNG 格式文件头
    const IMAGE_MAGIC: &'static [u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    pub fn detect_format(data: &[u8]) -> Self {
        match data {
            _ if data.starts_with(Self::IMAGE_MAGIC) => ImageFormat::Png,
            _ => ImageFormat::Unknow,
        }
    }

    pub fn extension(&self) -> Option<&'static str> {
        match self {
            ImageFormat::Png => Some("png"),
            ImageFormat::Gif => Some("gif"),
            ImageFormat::Jpg => Some("jpg"),
            ImageFormat::Unknow => None,
        }
    }
}
