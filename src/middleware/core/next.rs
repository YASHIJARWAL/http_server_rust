use crate::{http::{request::Request, response::Response}, middleware::Middleware,};

pub struct Next<'a>{
    pub middlewares: &'a[Box<dyn Middleware>],
    pub handler: fn (&Request)->Response,
}
impl <'a>Next<'a>{
    pub fn run(self , req: &Request)->Response{
        if let Some((first,rest))=self.middlewares.split_first(){
            first.handle(req,Next{middlewares:rest,handler:self.handler},)
        }
        else {
            (self.handler)(req)
        }
    }
}