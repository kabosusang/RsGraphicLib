use std::mem;

use super::{
    image_error::{ImageError, PNGImageError},
    image_processor::{self, *},
};

/// PNG图片格式解析
///
///PTLE : 调色板数据块PLTE(palette chunk)包含有与索引彩色图像(indexed-color image)相关的彩色变换数据
///IDAT : 图像数据块IDAT(image data chunk)：它存储实际的数据，在数据流中可包含多个连续顺序的图像数据块。
///IEND : 图像结束数据IEND(image trailer chunk)：它用来标记PNG文件或者数据流已经结束，并且必须要放在文件的尾部。
/// png文件中的固定签名
#[derive(Debug)]
pub struct PngSignature {}

impl PngSignature {
    //PNG 格式文件头
    pub const IMAGE_HEAD: &'static [u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    //IEND PNG格式结尾 ：它用来标记PNG文件或者数据流已经结束
    pub const IMAGE_END: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
}

/// PNG Chuck
///
/// PNG分为关键数据块(critical chuck)
/// 和辅助数据块(ancillary chunks)
struct PngChuck {
    ///数据块的长度
    length: [u8; 4],

    ///数据块的类型
    chuck_type: [u8; 4],

    ///Chuch Data 数据块数据
    // chuck_data : [u32]

    //CRC校验
    crc_check: [u8; 4],
}

/// IHDR数据块 -> PNG格式的第一个数据块
///
/// width : 图像宽度
/// height : 图像高度
/// -----------------------------------
/// bit_depth :
/// 1,2,4,8    -> 索引彩色图像
/// 1,2,4,6,16 -> 灰度图像  
/// 8,16       -> 真彩色图像
/// ------------------------------------
/// color_type -> 颜色类型
/// 0 : 灰度图像  1,2,4,8
/// 2 : 真彩色图像 8,16
/// 3 : 索引彩色图像 1,2,4,8
/// 4 : 带α通道数据的灰度图像 8,16
/// 6 : 带α通道数据的真彩色图像 8,16

#[derive(Debug, Clone)]
pub struct IHDRChuck {
    ///图像宽度
    ///注意: PNG格式是大端格式不变
    pub width: u32,

    ///图像高度
    pub height: u32,

    ///图像深度
    pub bit_depth: u8,

    ///颜色类型
    pub color_type: u8,

    ///压缩方法(LZ77派生方法)
    pub compression_method: u8,

    ///滤波器方法
    pub filter_method: u8,

    ///隔行扫描方法
    pub interlace_method: u8,
}

impl IHDRChuck {
    /// parse
    pub fn from_bytes<T: AsRef<[u8]>>(data: T) -> Option<Self> {
        let data: &[u8] = data.as_ref();

        // 检查IHDR固定数据块
        if data.len() != 13 {
            return None;
        }

        Some(Self {
            // 直接索引，更高效
            width: u32::from_be_bytes(data[0..4].try_into().ok()?),
            height: u32::from_be_bytes(data[4..8].try_into().ok()?),
            bit_depth: data[8],
            color_type: data[9],
            compression_method: data[10],
            filter_method: data[11],
            interlace_method: data[12],
        })
    }
}

/// 快速文件读取存储
#[derive(Debug, Clone)]
pub struct PNGImagePreview {
    /// IHDR元数据
    ihdr_chuck: IHDRChuck,
}

impl PNGImagePreview {
    pub fn build<T: AsRef<[u8]>>(data: T) -> Result<Self, ImageError> {
        let bytes = data.as_ref();

        match IHDRChuck::from_bytes(bytes) {
            Some(chuck) => Ok(Self { ihdr_chuck: chuck }),
            None => Err(ImageError::PngError(PNGImageError::InvalidIHDR)),
        }
    }
}

impl ImageProcess for PNGImagePreview {
    fn width(&self) -> u32 {
        self.ihdr_chuck.width
    }

    fn height(&self) -> u32 {
        self.ihdr_chuck.height
    }

    fn colot_type(&self) -> ImageColorTypeDepth {
        match self.ihdr_chuck.color_type {
            0 => ImageColorTypeDepth::GrayScale(self.ihdr_chuck.bit_depth),
            2 => ImageColorTypeDepth::TrueColor(self.ihdr_chuck.bit_depth),
            3 => ImageColorTypeDepth::IndexedColor(self.ihdr_chuck.bit_depth),
            4 => ImageColorTypeDepth::AlphaGrayScale(self.ihdr_chuck.bit_depth),
            6 => ImageColorTypeDepth::AlphaTrueColor(self.ihdr_chuck.bit_depth),
            _ => ImageColorTypeDepth::Unknow,
        }
    }
}

/// 完整文件存储
#[derive(Debug)]
pub struct PNGImage {
    /// PNG文件格式固定签名
    ///
    /// 包含必须的头签名和尾签名
    png_signature: PngSignature,

    /// 二进制文件的源数据
    raw_data: ImageData,

    /// IHDR数据块
    ihdr_chuck: IHDRChuck,
}

impl PNGImage {
    pub fn build(data: &[u8]) {}
}

impl ImageProcess for PNGImage {
    fn width(&self) -> u32 {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn colot_type(&self) -> ImageColorTypeDepth {
        todo!()
    }
}
