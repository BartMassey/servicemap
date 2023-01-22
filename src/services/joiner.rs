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

    pub fn invoke<S1, S2>(map: &mut ServiceMap, key: usize, s1: S1, s2: S2) -> ServiceResult<String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let entry = map.0.get_mut(key).ok_or(ServiceMapError::InvalidKey)?;
        let result = entry.call(&(s1.into(), s2.into()))?;
        let result = result
            .downcast::<String>()
            .map_err(|_| ServiceMapError::IncorrectResultType)?;
        Ok(*result)
    }
}

impl ServicePackage for Joiner {
    fn package() -> Box<dyn Service> {
        Box::new(Self)
    }
}

impl Service for Joiner {
    fn call(&mut self, args: &(dyn Any)) -> ServiceResult<Box<dyn Any + 'static>> {
        let args: &(String, String) = args
            .downcast_ref::<(String, String)>()
            .ok_or(ServiceMapError::IncorrectArgumentType)?;
        Ok(Box::new(self.join(&args.0, &args.1)))
    }
}
