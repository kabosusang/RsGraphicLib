use graphiclib::{image::image_processor::ImageProcess, *};

fn main() {
    let preview = image::open_preview("fnm.png");
    match preview {
        Ok(pre) => {
            println!("图像格式: {:#?}", pre.extension().unwrap());
            println!("图像宽度: {}", pre.width());
            println!("图像高度: {}", pre.height());
        }
        Err(e) => {
            eprint!("{:}", e)
        }
    }

    if let Ok(config) = glenv::EnvConfig::<glenv::EnvShell>::build() {
        println!("{:#?}", config);
    }
}
