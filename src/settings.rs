use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub(crate) struct Settings {
    pub evil: bool,
}

impl kubewarden::settings::Validatable for Settings {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::settings::Validatable;

    #[test]
    fn validate_settings() -> Result<(), ()> {
        let settings: Settings = Default::default();

        assert!(settings.validate().is_ok());
        Ok(())
    }
}
