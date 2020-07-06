use super::model::init::InitContext;

pub struct DbKernel<'a> {
    ctx: InitContext<'a>,
}

impl <'a> DbKernel<'a> {
    pub fn new(dir: &'a str) -> DbKernel {
        let ctx = InitContext::new(dir);
        DbKernel{ctx: ctx}
    }
}