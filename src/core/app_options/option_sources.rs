use std::{borrow::Cow, str::FromStr};

pub fn use_env(env_name: &str) -> Option<String> {
    std::env::var(env_name).ok()
}
pub fn get_from_cli_env<'a, T>(
    from_cli: Option<&'a T>,
    env_name: &str,
    on_get_env: impl Fn(&str) -> Option<String>,
) -> Option<Cow<'a, T>>
where
    T: Clone + FromStr,
{
    if let Some(cli_val) = from_cli {
        return Some(Cow::Borrowed(cli_val));
    } else if let Some(env_val) = on_get_env(env_name) {
        match env_val.parse() {
            Ok(parsed) => Some(Cow::Owned(parsed)),
            Err(_) => None,
        }
    } else {
        None
    }
}

// TODO: dead_code
#[allow(dead_code)]
fn get_from_cli_env_conf<'a, T>(
    from_cli: Option<&'a T>,
    env_name: &str,
    conf: Option<&'a T>,
    on_get_env: impl Fn(&str) -> Option<String>,
) -> Option<Cow<'a, T>>
where
    T: Clone + FromStr,
{
    let cli_env = get_from_cli_env(from_cli, env_name, on_get_env);
    if cli_env.is_some() {
        cli_env
    } else {
        conf.map(Cow::Borrowed)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;
    const ENV_NAME_DOES_NOT_MATTER: &str = "";

    macro_rules! cli_val {
        () => {{
            "cli_val".to_owned()
        }};
    }
    macro_rules! cli_val_opt {
        () => {{
            Some(cli_val!())
        }};
    }
    macro_rules! env_val {
        () => {{
            "env_val".to_owned()
        }};
    }
    macro_rules! env_val_opt {
        () => {{
            Some(env_val!())
        }};
    }
    macro_rules! conf_val {
        () => {{
            "conf_val".to_owned()
        }};
    }
    macro_rules! conf_val_opt {
        () => {{
            Some(conf_val!())
        }};
    }

    #[derive(PartialEq, Eq, Debug)]
    enum ExpectedCow {
        IsNone,
        IsBorrowed(String),
        IsOwned(String),
    }

    #[test_case(cli_val_opt!(), env_val_opt!() => ExpectedCow::IsBorrowed(cli_val!())  ; "Should return cli value")]
    #[test_case(None, env_val_opt!() => ExpectedCow::IsOwned(env_val!())  ; "Should return env value")]
    #[test_case(cli_val_opt!(), None => ExpectedCow::IsBorrowed(cli_val!())  ; "Should return cli value without env value")]
    #[test_case(None, None => ExpectedCow::IsNone  ; "Should return no cli and env value")]
    fn should_return_cli_env_or_none(
        cli: Option<String>,
        env_actual_val: Option<String>,
    ) -> ExpectedCow {
        let actual = get_from_cli_env(cli.as_ref(), ENV_NAME_DOES_NOT_MATTER, |_| {
            env_actual_val.clone()
        });

        if let Some(actual_cow) = actual {
            match actual_cow {
                Cow::Borrowed(borrowed) => ExpectedCow::IsBorrowed(borrowed.to_owned()),
                Cow::Owned(owned) => ExpectedCow::IsOwned(owned),
            }
        } else {
            ExpectedCow::IsNone
        }
    }

    #[test_case(cli_val_opt!(), env_val_opt!(), conf_val_opt!() => ExpectedCow::IsBorrowed(cli_val!())  ; "Should return cli value")]
    #[test_case(None, None, None => ExpectedCow::IsNone  ; "Should return no cli, env or conf value")]
    #[test_case(cli_val_opt!(), None, None => ExpectedCow::IsBorrowed(cli_val!())  ; "Should return cli value as the only value")]
    #[test_case(None, env_val_opt!(), None => ExpectedCow::IsOwned(env_val!())  ; "Should return env value as the only value")]
    #[test_case(None, None, conf_val_opt!() => ExpectedCow::IsBorrowed(conf_val!())  ; "Should return conf reference")]
    fn should_return_cli_env_conf_none(
        cli: Option<String>,
        env_actual_val: Option<String>,
        conf_actual_val: Option<String>,
    ) -> ExpectedCow {
        let actual = get_from_cli_env_conf(
            cli.as_ref(),
            ENV_NAME_DOES_NOT_MATTER,
            conf_actual_val.as_ref(),
            |_| env_actual_val.clone(),
        );

        if let Some(actual_cow) = actual {
            match actual_cow {
                Cow::Borrowed(borrowed) => ExpectedCow::IsBorrowed(borrowed.to_owned()),
                Cow::Owned(owned) => ExpectedCow::IsOwned(owned),
            }
        } else {
            ExpectedCow::IsNone
        }
    }
}
