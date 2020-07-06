use super::model::init::InitContext;

pub struct DbKernel<'a> {
    ctx: InitContext<'a>,
}

impl <'a> DbKernel<'a> {
    pub fn new(dir: &'a str) -> Self {
        let ctx = InitContext::new(dir);
        DbKernel{ctx: ctx}
    }

    pub fn process_query(&mut self, query: &str) {

    }
}