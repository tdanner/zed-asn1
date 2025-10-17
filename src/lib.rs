mod language_server;

use language_server::Asn1LanguageServer;
use zed_extension_api::{self as zed, Command, LanguageServerId, Result};

struct Asn1Extension {
    asn1_lsp: Option<Asn1LanguageServer>,
}

impl zed::Extension for Asn1Extension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self { asn1_lsp: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Command> {
        let asn1_lsp = self.asn1_lsp.get_or_insert_with(Asn1LanguageServer::new);

        Ok(Command {
            command: asn1_lsp.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Asn1Extension);
