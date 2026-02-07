/** 用来获取或者读取图片的元信息 判断图片类型
    也存储图像的数据
*/
use super::{
    image_error::ImageError,
    image_processor::ImageProcess,
    png::{PNGImage, PNGImagePreview, PngSignature},
};

#[derive(Debug, Clone)]
pub enum ImageFormatPreview {
    Png(PNGImagePreview),
    Jpg,
    Gif,
    Unknow,
}

impl ImageFormatPreview {
    pub fn detect_format(data: &[u8]) -> Result<Self, ImageError> {
        match data {
            _ if data.starts_with(PngSignature::IMAGE_HEAD) => {
                const head_len: usize = PngSignature::IMAGE_HEAD.len();
                assert!(
                    data.len() >= head_len,
                    "Png data too shot: {} bytes",
                    data.len()
                );

                let ihdr_data_start = head_len + 8;
                let body: &[u8] = &data[ihdr_data_start..ihdr_data_start + 13];
                match PNGImagePreview::build(body) {
                    Ok(preview) => Ok(ImageFormatPreview::Png(preview)),
                    Err(e) => Err(e), // 传播错误
                }
            }
            _ => Ok(ImageFormatPreview::Unknow),
        }
    }

    pub fn extension(&self) -> Option<&'static str> {
        match self {
            ImageFormatPreview::Png(_) => Some("png"),
            ImageFormatPreview::Gif => Some("gif"),
            ImageFormatPreview::Jpg => Some("jpg"),
            ImageFormatPreview::Unknow => None,
        }
    }
}

impl ImageProcess for ImageFormatPreview {
    fn width(&self) -> u32 {
        match self {
            ImageFormatPreview::Png(pv) => pv.width(),
            ImageFormatPreview::Jpg => todo!(),
            ImageFormatPreview::Gif => todo!(),
            ImageFormatPreview::Unknow => todo!(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            ImageFormatPreview::Png(pv) => pv.height(),
            ImageFormatPreview::Jpg => todo!(),
            ImageFormatPreview::Gif => todo!(),
            ImageFormatPreview::Unknow => todo!(),
        }
    }

    fn colot_type(&self) -> super::image_processor::ImageColorTypeDepth {
        match self {
            ImageFormatPreview::Png(pv) => pv.colot_type(),
            ImageFormatPreview::Jpg => todo!(),
            ImageFormatPreview::Gif => todo!(),
            ImageFormatPreview::Unknow => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum ImageFormat {
    Png(PNGImage),
}
