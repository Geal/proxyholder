pub(crate) struct Configuration {}

impl Configuration {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Configuration {})
    }
}
