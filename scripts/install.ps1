# RDF Extension Installer for Zed (Windows PowerShell)
# Usage: .\scripts\install.ps1
#
# This script installs the prebuilt RDF extension for Zed on Windows.
# No Rust toolchain required.

param(
    [switch]$Uninstall,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# Extension info
$ExtensionName = "rdf"
$ExtensionDir = "$env:APPDATA\Zed\extensions\installed\$ExtensionName"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoDir = Split-Path -Parent $ScriptDir

function Show-Help {
    Write-Host @"
RDF Extension Installer for Zed

Usage:
    .\scripts\install.ps1           Install the extension
    .\scripts\install.ps1 -Uninstall    Uninstall the extension
    .\scripts\install.ps1 -Help         Show this help

The extension will be installed to:
    $env:APPDATA\Zed\extensions\installed\rdf

Requirements:
    - Download the release from GitHub first, or build with 'task release:bundle'
    - The dist/ folder must contain extension.wasm
"@
}

function Install-Extension {
    Write-Host "Installing RDF extension for Zed..." -ForegroundColor Cyan
    Write-Host "Extension directory: $ExtensionDir"

    # Check for prebuilt files
    $DistDir = Join-Path $RepoDir "dist"
    $WasmFile = Join-Path $DistDir "extension.wasm"

    if (-not (Test-Path $WasmFile)) {
        Write-Host @"

ERROR: Prebuilt files not found in dist/

Please either:
  1. Download a release from: https://github.com/rpietzsch/zed-rdf-lsp/releases
     Extract it to the 'dist' folder in this repository.

  2. Build from source (requires Rust):
     task build
     task release:bundle

"@ -ForegroundColor Red
        exit 1
    }

    # Create extension directory
    if (-not (Test-Path $ExtensionDir)) {
        New-Item -ItemType Directory -Path $ExtensionDir -Force | Out-Null
    }

    # Copy files
    Write-Host "Copying extension files..."

    # Copy extension.wasm
    Copy-Item $WasmFile -Destination $ExtensionDir -Force

    # Copy extension.toml
    Copy-Item (Join-Path $RepoDir "extension.toml") -Destination $ExtensionDir -Force

    # Copy languages directory
    $LangSrc = Join-Path $RepoDir "languages"
    $LangDst = Join-Path $ExtensionDir "languages"
    if (Test-Path $LangDst) {
        Remove-Item $LangDst -Recurse -Force
    }
    Copy-Item $LangSrc -Destination $ExtensionDir -Recurse -Force

    # Copy grammars if present
    $GrammarsSrc = Join-Path $DistDir "grammars"
    if (Test-Path $GrammarsSrc) {
        $GrammarsDst = Join-Path $ExtensionDir "grammars"
        if (Test-Path $GrammarsDst) {
            Remove-Item $GrammarsDst -Recurse -Force
        }
        Copy-Item $GrammarsSrc -Destination $ExtensionDir -Recurse -Force
    }

    Write-Host ""
    Write-Host "Extension installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Restart Zed to activate the extension."
    Write-Host "The extension will be available for .rq, .sparql, .ttl, and .trig files."
}

function Uninstall-Extension {
    Write-Host "Uninstalling RDF extension..." -ForegroundColor Cyan

    if (Test-Path $ExtensionDir) {
        Remove-Item $ExtensionDir -Recurse -Force
        Write-Host "Extension uninstalled from: $ExtensionDir" -ForegroundColor Green
    } else {
        Write-Host "Extension not found at: $ExtensionDir" -ForegroundColor Yellow
    }
}

# Main
if ($Help) {
    Show-Help
} elseif ($Uninstall) {
    Uninstall-Extension
} else {
    Install-Extension
}
