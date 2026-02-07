/// 存储图像的元数据

/// 图像深度
/// 表示表示用多少位来存储单个像素的颜色
/// u8 表示的是深度的具体数据
/// png -> IHDR : Color Type ,Bit Depth
/// png根据读取的ColorType指定位深度

pub enum ImageColorTypeDepth {
    ///索引彩色图像(1,2,4,8)
    IndexedColor(u8),

    ///灰度图像(1:黑白二值 2:4级u灰度 8:256级灰度16:65536(高精度灰度))
    GrayScale(u8),

    ///真彩色图像(8 16)
    TrueColor(u8),

    ///带α通道数据的灰度图像
    AlphaGrayScale(u8),

    ///带α通道数据的真彩色图像
    AlphaTrueColor(u8),

    Unknow,
}

pub trait ImageProcess {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn colot_type(&self) -> ImageColorTypeDepth;
}

///存储图像的元数据
#[derive(Debug)]
pub struct ImageData {
    /// 图像文件的二进制数组
    raw_data: Vec<u8>,
}

impl ImageData {
    pub fn read(&mut self, data: Vec<u8>) -> () {
        self.raw_data = data;
        ()
    }
}
