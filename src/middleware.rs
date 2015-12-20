use std::error::Error;
use std::fmt;
use std::error;
use std::io::Error as IOError;
use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;

#[derive(Debug, Clone)]
pub struct RenderError {
    pub desc: String
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.desc)
    }
}

impl error::Error for RenderError {
    fn description(&self) -> &str {
        &self.desc[..]
    }
}

impl From<IOError> for RenderError {
    fn from(_: IOError) -> RenderError {
        RenderError::new("IO Error")
    }
}

impl RenderError {
    pub fn new<T: AsRef<str>>(desc: T) -> RenderError {
        RenderError {
            desc: desc.as_ref().to_owned()
        }
    }
}

#[derive(Clone)]
pub struct Template{
	value: String
}

impl Template {
	pub fn new<T: ToString>(value: T) -> Template {
		Template {
			value: value.to_string()
		}
	}
}

pub struct MaudEngine;

impl MaudEngine{
	
	pub fn new() -> MaudEngine {
			MaudEngine
			
	}
	
	pub fn render(value: &str) -> Result<String, RenderError> {
		Ok( value.to_string())
	}
	
}

impl typemap::Key for MaudEngine {
    type Value = Template;
}

impl Modifier<Response> for Template {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<MaudEngine>(self);
    }
}

impl PluginFor<Response> for MaudEngine {
    type Error = ();

    fn eval(resp: &mut Response) -> Result<Template, ()> {
        match resp.extensions.get::<MaudEngine>(){
            Some(t) => Ok(t.clone()),
            None => Err(())
        }
    }
}


impl AfterMiddleware for MaudEngine {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let page_wrapper = resp.extensions.get::<MaudEngine>().as_ref()
            .and_then(|t| {
                Some(MaudEngine::render(&t.value))
            });

        match page_wrapper {
            Some(page_result) => {
                match page_result {
                    Ok(page) => {
                        if !resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType::html());
                        }
                        resp.set_mut(page );
                        Ok(resp)
                    }
                    Err(e) => {
                        info!("{}", e.description());
                        Err(IronError::new(e, status::InternalServerError))
                    }
                }
            }
            None => {
                Ok(resp)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use iron::prelude::*;
    use middleware::*;

    fn hello_world() -> IronResult<Response> {
        let resp = Response::new();

        Ok(resp.set(Template::new("Maud on Iron")))
    }

    #[test]
    fn test_resp_set() {
        let mut resp = hello_world().ok().expect("response expected");

        match resp.get::<MaudEngine>() {
            Ok(h) => {
               assert_eq!(
               						h.value.to_string(),
                           "Maud on Iron");
            },
            _ => panic!("template expected")
        }
    }

}
