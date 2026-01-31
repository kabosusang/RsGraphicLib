/** 用来获取或者读取图片的元信息 判断图片类型
    也存储图像的数据
*/
use super::png::PngSignature;

#[derive(Copy, Clone)]
pub enum ImageFormat {
    Png,
    Jpg,
    Gif,
    Unknow,
}

impl ImageFormat {
    pub fn detect_format(data: &[u8]) -> Self {
        match data {
            _ if data.starts_with(PngSignature::IMAGE_HEAD) => ImageFormat::Png,
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

