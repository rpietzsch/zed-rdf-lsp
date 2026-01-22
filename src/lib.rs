use zed_extension_api::{self as zed, serde_json, Result};

/// RDF extension for Zed providing language server support for SPARQL, Turtle, and TriG.
struct RdfExtension {
    cached_sparql_server_path: Option<String>,
    cached_turtle_server_path: Option<String>,
    cached_trig_server_path: Option<String>,
}

/// Language server IDs (must match extension.toml)
const SPARQL_SERVER_ID: &str = "sparql-language-server";
const TURTLE_SERVER_ID: &str = "turtle-language-server";
const TRIG_SERVER_ID: &str = "trig-language-server";

/// NPM package names
const SPARQL_NPM_PACKAGE: &str = "sparql-language-server";
const TURTLE_NPM_PACKAGE: &str = "turtle-language-server";
const TRIG_NPM_PACKAGE: &str = "trig-language-server";

impl RdfExtension {
    /// Install or update an npm-based language server and return the path to its binary.
    fn install_npm_language_server(
        &self,
        package_name: &str,
        server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        // Check if already installed
        let installed_version = zed::npm_package_installed_version(package_name)?;
        let latest_version = zed::npm_package_latest_version(package_name)?;

        // Install or update if needed
        if installed_version.as_ref() != Some(&latest_version) {
            zed::set_language_server_installation_status(
                server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(package_name, &latest_version);

            if result.is_err() {
                zed::set_language_server_installation_status(
                    server_id,
                    &zed::LanguageServerInstallationStatus::Failed(format!(
                        "Failed to install {} v{}",
                        package_name, latest_version
                    )),
                );
                return Err(format!(
                    "Failed to install npm package: {} v{}",
                    package_name, latest_version
                ));
            }
        }

        // Get the path to the installed package binary
        zed::npm_package_installed_version(package_name)?
            .ok_or_else(|| format!("Package {} not found after installation", package_name))?;

        Ok(zed::node_binary_path()?)
    }

    /// Get the language server command for an npm-based server.
    fn npm_server_command(
        &mut self,
        server_id: &zed::LanguageServerId,
        package_name: &str,
        cached_path: &mut Option<String>,
    ) -> Result<zed::Command> {
        let node_path = match cached_path {
            Some(path) => path.clone(),
            None => {
                let path = self.install_npm_language_server(package_name, server_id)?;
                *cached_path = Some(path.clone());
                path
            }
        };

        // The npm package provides a CLI that can be run via node
        // The entry point is typically in node_modules/<package>/dist/cli.js
        Ok(zed::Command {
            command: node_path,
            args: vec![
                zed::npm_package_path(package_name)?,
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

impl zed::Extension for RdfExtension {
    fn new() -> Self {
        Self {
            cached_sparql_server_path: None,
            cached_turtle_server_path: None,
            cached_trig_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            SPARQL_SERVER_ID => self.npm_server_command(
                language_server_id,
                SPARQL_NPM_PACKAGE,
                &mut self.cached_sparql_server_path,
            ),
            TURTLE_SERVER_ID => self.npm_server_command(
                language_server_id,
                TURTLE_NPM_PACKAGE,
                &mut self.cached_turtle_server_path,
            ),
            TRIG_SERVER_ID => self.npm_server_command(
                language_server_id,
                TRIG_NPM_PACKAGE,
                &mut self.cached_trig_server_path,
            ),
            _ => Err(format!("Unknown language server: {}", language_server_id)),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // The Stardog language servers don't require special workspace configuration
        // but this can be extended to support custom settings
        Ok(None)
    }
}

zed::register_extension!(RdfExtension);
