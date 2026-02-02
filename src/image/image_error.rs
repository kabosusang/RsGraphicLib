
/// 图像文件错误处理
#[derive(Debug)]
pub enum ImageError {
    Io(std::io::Error, String), //错误路径
    ///文件大小太小
    FileTooSmall(String),
    ///格式不支持
    UnSupportedFormat(String),

	PngError(PNGImageError),
}

impl From<std::io::Error> for ImageError {
    fn from(error: std::io::Error) -> Self {
        ImageError::Io(error, String::from("unknown path")) // 没有路径信息
    }
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::Io(err, path) => write!(f, "IO error for file '{}': {}", path, err),
			ImageError::FileTooSmall(path) => write!(f,"file {} too small",path),
			ImageError::UnSupportedFormat(path) => write!(f,"file {}unsuppored format",path),
			ImageError::PngError(png) => png.fmt(f), 
        }
    }
}

/// PNG文件错误
#[derive(Debug)]
pub enum PNGImageError
{
	InvalidIHDR,
}

impl std::fmt::Display for PNGImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PNGImageError::InvalidIHDR => write!(f,"invalidIHDR"),
        }
    }
}



