use anyhow::Result;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MonitorIndex(pub usize);

impl std::fmt::Display for MonitorIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    pub width: i32,
    pub height: i32,
}

impl Monitor {
    pub const fn new(width: i32, height: i32) -> Self {
        Monitor { width, height }
    }

    pub fn from_string_dimensions(raw_dimensions: &str) -> Result<Self> {
        let dimensions = raw_dimensions.split('x').collect::<Vec<&str>>();

        if dimensions.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid monitor dimensions: {raw_dimensions}"
            ));
        }

        let width: i32 = dimensions[0].parse()?;
        let height: i32 = dimensions[1].parse()?;

        Ok(Monitor::new(width, height))
    }
}
