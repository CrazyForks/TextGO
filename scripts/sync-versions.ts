#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __DIRNAME = path.dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = path.resolve(__DIRNAME, '..');
const TAURI_CONF_PATH = path.join(PROJECT_ROOT, 'src-tauri', 'tauri.conf.json');
const CARGO_TOML_PATH = path.join(PROJECT_ROOT, 'src-tauri', 'Cargo.toml');
const PACKAGE_JSON_PATH = path.join(PROJECT_ROOT, 'package.json');

/**
 * Sync version to package.json and Cargo.toml.
 */
function syncVersions() {
  // read tauri.conf.json
  const tauriConf = JSON.parse(fs.readFileSync(TAURI_CONF_PATH, 'utf-8'));
  const targetVersion = tauriConf.version;
  console.log(`📋 Target version from tauri.conf.json: ${targetVersion}`);

  // update package.json
  const packageJson = JSON.parse(fs.readFileSync(PACKAGE_JSON_PATH, 'utf-8'));
  if (packageJson.version !== targetVersion) {
    packageJson.version = targetVersion;
    fs.writeFileSync(PACKAGE_JSON_PATH, JSON.stringify(packageJson, null, 2) + '\n', 'utf-8');
    console.log(`✅ Updated package.json version to: ${targetVersion}`);
  }

  // update Cargo.toml
  let cargoToml = fs.readFileSync(CARGO_TOML_PATH, 'utf-8');
  const versionRegex = /^version\s*=\s*"[^"]*"/m;
  const match = cargoToml.match(versionRegex);
  if (match && match[0].match(/"([^"]*)"/)?.[1] !== targetVersion) {
    cargoToml = cargoToml.replace(versionRegex, `version = "${targetVersion}"`);
    fs.writeFileSync(CARGO_TOML_PATH, cargoToml, 'utf-8');
    console.log(`✅ Updated Cargo.toml version to: ${targetVersion}`);
  }

  console.log('🎉 Version sync completed successfully');
}

syncVersions();
