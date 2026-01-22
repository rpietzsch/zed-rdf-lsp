use std::fs;
use zed_extension_api::{self as zed, serde_json, Result};

/// RDF extension for Zed providing language server support for SPARQL, Turtle, and TriG.
struct RdfExtension {
    /// Cached extension directory path (captured at initialization)
    extension_dir: Option<String>,
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

/// Normalize path for use with Node.js across platforms
/// WASI on Windows might return paths like "/C:/Users/..." or "C:/Users/..."
/// Node.js on Windows needs paths like "C:/Users/..." (forward slashes work)
fn normalize_path_for_node(path: &str) -> String {
    let mut normalized = path.replace('\\', "/");

    // Handle WASI-style Windows paths: "/C:/..." -> "C:/..."
    // This pattern occurs when WASI maps Windows drives
    if normalized.len() >= 4
        && normalized.starts_with('/')
        && normalized.chars().nth(1).map_or(false, |c| c.is_ascii_alphabetic())
        && normalized.chars().nth(2) == Some(':')
    {
        normalized = normalized[1..].to_string();
    }

    normalized
}

/// Get the extension directory, caching it for later use
fn get_extension_dir(cached: &mut Option<String>) -> Result<String> {
    if let Some(ref dir) = cached {
        return Ok(dir.clone());
    }

    // Get current directory - this should be the extension's directory during initialization
    let dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get extension directory: {}", e))?;

    let dir_str = normalize_path_for_node(&dir.to_string_lossy());
    *cached = Some(dir_str.clone());
    Ok(dir_str)
}

fn language_server_binary_path(
    package_name: &str,
    language_server_id: &zed::LanguageServerId,
    extension_dir: &str,
    did_find_server: &mut bool,
) -> Result<String> {
    // Build path to the server CLI
    // WASI uses forward slashes internally, and Node.js accepts forward slashes on all platforms
    let server_path = format!(
        "{}/node_modules/{}/dist/cli.js",
        extension_dir, package_name
    );

    // Check if the server binary already exists
    // WASI filesystem abstraction handles path format conversion
    if fs::metadata(&server_path).map_or(false, |stat| stat.is_file()) {
        *did_find_server = true;
        return Ok(server_path);
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

    Ok(server_path)
}

impl zed::Extension for RdfExtension {
    fn new() -> Self {
        // Capture extension directory at initialization time
        // This is when current_dir reliably points to the extension's directory
        let extension_dir = std::env::current_dir()
            .ok()
            .map(|p| normalize_path_for_node(&p.to_string_lossy()));

        Self {
            extension_dir,
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
        // Get extension directory (cached from initialization, or try to get it now)
        let extension_dir = get_extension_dir(&mut self.extension_dir)?;

        let server_path = match language_server_id.as_ref() {
            SPARQL_SERVER_ID => language_server_binary_path(
                SPARQL_PACKAGE,
                language_server_id,
                &extension_dir,
                &mut self.did_find_sparql_server,
            )?,
            TURTLE_SERVER_ID => language_server_binary_path(
                TURTLE_PACKAGE,
                language_server_id,
                &extension_dir,
                &mut self.did_find_turtle_server,
            )?,
            TRIG_SERVER_ID => language_server_binary_path(
                TRIG_PACKAGE,
                language_server_id,
                &extension_dir,
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
