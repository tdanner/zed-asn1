use zed_extension_api::{self as zed, Command, EnvVars, LanguageServerId, Result};

struct Asn1Extension {
    // ... state
}

impl zed::Extension for Asn1Extension {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Command> {
        Ok(Command {
            command: get_path_to_language_server_executable()?,
            args: get_args_for_language_server()?,
            env: get_env_for_language_server()?,
        })
    }
}

fn get_env_for_language_server() -> Result<EnvVars> {
    Ok(Vec::new())
}

fn get_args_for_language_server() -> Result<Vec<String>> {
    Ok(Vec::new())
}

fn get_path_to_language_server_executable() -> Result<String> {
    Ok(String::from("path/to/language_server_executable"))
}

zed::register_extension!(Asn1Extension);
