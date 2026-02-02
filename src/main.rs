use graphiclib::*;

fn main() {
	let preview = image::open_preview("fnm.png").unwrap();

	let png_name = preview.extension().unwrap();

	println!("图像宽度: {:#?}", png_name);


	

    if let Ok(config) = glenv::EnvConfig::<glenv::EnvShell>::build() {
        println!("{:#?}", config);
    }
}
