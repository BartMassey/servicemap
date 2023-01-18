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
}

impl ServicePackage for Joiner {
    type CallArgs = (&'static str, &'static str);
    type CallResult = String;

    fn package() -> ServiceEntry {
        ServiceEntry(Box::new(Self))
    }
}

impl Service for Joiner {
    fn call(&mut self, args: &dyn Any) -> ServiceResult<Box<dyn Any>> {
        let args: (&str, &str) = *args.downcast_ref::<(&str, &str)>()
            .ok_or(ServiceMapError::IncorrectArgumentType)?;
        Ok(Box::new(self.join(args.0, args.1)))
    }
}
