use std::env;

#[derive(Debug)]
pub struct EnvShell;

#[derive(Debug)]
pub struct EnvConfig<Tag = EnvShell> {
    pub file_path: String,
    _tag: std::marker::PhantomData<Tag>,
}

impl EnvConfig<EnvShell> {
    fn build_config(
        mut args: impl Iterator<Item = String>,
    ) -> Result<EnvConfig<EnvShell>, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(EnvConfig {
            file_path,
            _tag: std::marker::PhantomData,
        })
    }

    pub fn build() -> Result<EnvConfig<EnvShell>, &'static str> {
        let config = Self::build_config(env::args());

        match config {
            Ok(cf) => Ok(cf),
            Err(e) => Err(e),
        }
    }
}
