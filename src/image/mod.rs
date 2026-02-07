/*! 图片格式读取模块
    图片处理模块
*/

use format::{ImageFormat, ImageFormatPreview};
use image_error::ImageError;
use std::{fs::File, io::Read, path::Path};

pub mod format;
mod png;

mod image_error;
pub mod image_processor;

///快速预览读取
///默认读取文件的前40个字节
pub fn open_preview<P: AsRef<Path>>(path: P) -> Result<ImageFormatPreview, ImageError> {
    let path_ref = path.as_ref();
    let path_str = path_ref.display().to_string();
    //打开文件
    let mut file = File::open(path_ref).map_err(|e| ImageError::Io(e, path_str))?;

    //读取头文件 读取文件元数据
    const READ_BYTES: usize = 40;
    let mut buffer_preview = [0u8; READ_BYTES];
    let bytes_read_num = file
        .read(&mut buffer_preview)
        .map_err(|e| ImageError::Io(e, path_ref.display().to_string()))?;

    //读取文件后的处理
    if bytes_read_num < 8 {
        return Err(ImageError::FileTooSmall(path_ref.display().to_string()));
    }

    let detect_format = ImageFormatPreview::detect_format(&buffer_preview);
    match detect_format {
        Ok(preview) => Ok(preview),
        Err(e) => Err(e),
    }
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<ImageFormat, ImageError> {
    let path_ref = path.as_ref(); // 转换为 &Path
    let path_str = path_ref.display().to_string(); // 先获取路径字符串
    let bytes = std::fs::read(path_ref).map_err(|e| ImageError::Io(e, path_str.clone()))?; // 手动转换并包含径路

    todo!()
}
