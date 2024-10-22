use figment::{
    error::Result,
    providers::{Env, Format, Serialized, Toml},
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {}

impl Config {
    pub fn figment() -> Figment {
        Figment::from(Config::default())
            .merge(Toml::file(Env::var_or("OSU_MAESTRO_CONFIG", "Maestro.toml")).nested())
    }

    pub fn try_from<T: Provider>(provider: T) -> Result<Self> {
        let figment = Figment::from(provider);
        let config = figment.extract::<Self>()?;
        Ok(config)
    }

    pub fn from<T: Provider>(provider: T) -> Self {
        Self::try_from(provider)
            .unwrap_or_else(|_e| panic!("aborting due to configuration error(s)"))
    }
}

impl Provider for Config {
    #[track_caller]
    fn metadata(&self) -> Metadata {
        if self == &Config::default() {
            Metadata::named("maestro::Config::default()")
        } else {
            Metadata::named("maestro::Config")
        }
    }

    #[track_caller]
    fn data(&self) -> Result<Map<Profile, Dict>> {
        let map: Map<Profile, Dict> = Serialized::defaults(self).data()?;

        Ok(map)
    }
}
