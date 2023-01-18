use crate::*;

#[derive(Default)]
pub struct Joiner;

impl Joiner {
    fn join(&mut self, s1: &str, s2: &str) -> String {
        let mut result = s1.to_string();
        result.push(' ');
        result.push_str(s2);
        result
    }

    pub fn package(self) -> ServiceEntry {
        ServiceEntry(Box::new(self))
    }

    pub fn result(r: Box<dyn Any>) -> String {
        *r.downcast::<String>().unwrap()
    }
}

impl Service for Joiner {
    fn call(&mut self, args: &dyn Any) -> ServiceResult {
        let args: (&str, &str) = *args.downcast_ref::<(&str, &str)>()
            .ok_or(ServiceMapError::IncorrectArgumentType)?;
        Ok(Box::new(self.join(args.0, args.1)))
    }
}
