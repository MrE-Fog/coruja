use anyhow::{anyhow, Context, Result};

#[derive(Copy, Clone, Debug)]
pub struct Spec<'a, T> {
    pub key: &'a str,
    pub rule: Rule<T>,
}

#[derive(Copy, Clone, Debug)]
pub enum Rule<T> {
    Required,
    Optional { default: T },
}

// TODO Tentar reduzir a complexidade destas trait restrictions tentando usar um From ao invés
// de um parsing? O que fazer em casos de parsing próprios?
pub fn env_value_from_spec<T>(spec: Spec<T>) -> Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    match std::env::var(spec.key) {
        Ok(value) => {
            let parsed_value = value
                .parse::<T>()
                .with_context(|| format!("failed parsing {}", spec.key))?;
            Ok(parsed_value)
        }
        Err(std::env::VarError::NotPresent) => match spec.rule {
            Rule::Required => Err(anyhow!("missing required variable {}", spec.key)),
            Rule::Optional { default } => Ok(default),
        },
        Err(err) => Err(err)?,
    }
}
