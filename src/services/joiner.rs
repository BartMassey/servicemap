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

    pub fn package() -> Box<Joiner> {
        <Box<Joiner>>::default()
    }

    pub fn invoke<S1, S2>(map: &mut ServiceMap, key: usize, s1: S1, s2: S2) -> ServiceResult<String>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let joiner = map.get_service::<Joiner>(key)?;
        Ok(joiner.join(s1.as_ref(), s2.as_ref()))
    }
}
