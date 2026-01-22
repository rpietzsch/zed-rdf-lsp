use std::fs;
use zed_extension_api::{self as zed, serde_json, Result};

/// RDF extension for Zed providing language server support for SPARQL, Turtle, and TriG.
struct RdfExtension {
    did_find_sparql_server: bool,
    did_find_turtle_server: bool,
    did_find_trig_server: bool,
}

/// Language server IDs (must match extension.toml)
const SPARQL_SERVER_ID: &str = "sparql-language-server";
const TURTLE_SERVER_ID: &str = "turtle-language-server";
const TRIG_SERVER_ID: &str = "trig-language-server";

/// NPM package names for the language servers
const SPARQL_PACKAGE: &str = "sparql-language-server";
const TURTLE_PACKAGE: &str = "turtle-language-server";
const TRIG_PACKAGE: &str = "trig-language-server";

fn language_server_binary_path(
    package_name: &str,
    language_server_id: &zed::LanguageServerId,
    did_find_server: &mut bool,
) -> Result<String> {
    // Get the extension's working directory (where npm packages are installed)
    // This is called during extension initialization, so current_dir is the extension dir
    let extension_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Build absolute path to the server CLI
    let server_path = extension_dir
        .join("node_modules")
        .join(package_name)
        .join("dist")
        .join("cli.js");

    let server_path_str = server_path.to_string_lossy().to_string();

    // Check if the server binary already exists
    if fs::metadata(&server_path).map_or(false, |stat| stat.is_file()) {
        *did_find_server = true;
        return Ok(server_path_str);
    }

    // Install or update the npm package
    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::CheckingForUpdate,
    );

    let version = zed::npm_package_latest_version(package_name)?;

    if !*did_find_server
        || zed::npm_package_installed_version(package_name)?.as_ref() != Some(&version)
    {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::npm_install_package(package_name, &version)?;

        *did_find_server = true;
    }

    Ok(server_path_str)
}

impl zed::Extension for RdfExtension {
    fn new() -> Self {
        Self {
            did_find_sparql_server: false,
            did_find_turtle_server: false,
            did_find_trig_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = match language_server_id.as_ref() {
            SPARQL_SERVER_ID => language_server_binary_path(
                SPARQL_PACKAGE,
                language_server_id,
                &mut self.did_find_sparql_server,
            )?,
            TURTLE_SERVER_ID => language_server_binary_path(
                TURTLE_PACKAGE,
                language_server_id,
                &mut self.did_find_turtle_server,
            )?,
            TRIG_SERVER_ID => language_server_binary_path(
                TRIG_PACKAGE,
                language_server_id,
                &mut self.did_find_trig_server,
            )?,
            _ => return Err(format!("Unknown language server: {}", language_server_id)),
        };

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }
}

zed::register_extension!(RdfExtension);
