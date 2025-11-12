#!/usr/bin/env node
import { readdirSync, readFileSync, writeFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __DIRNAME = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__DIRNAME, '..');
const PACKAGE_JSON = join(PROJECT_ROOT, 'package.json');
const CARGO_TOML = join(PROJECT_ROOT, 'src-tauri', 'Cargo.toml');

/**
 * Get Svelte version number.
 *
 * @return version string
 */
function getSvelteVersion(): string {
  const packageJson = JSON.parse(readFileSync(PACKAGE_JSON, 'utf-8'));
  const svelteVersion = packageJson.devDependencies?.svelte || packageJson.dependencies?.svelte;
  if (!svelteVersion) {
    throw new Error('Svelte version not found in package.json');
  }
  return svelteVersion.replace(/^[\^~]/, '');
}

/**
 * Get Tauri version number.
 *
 * @return version string
 */
function getTauriVersion(): string {
  const cargoToml = readFileSync(CARGO_TOML, 'utf-8');
  const match = cargoToml.match(/tauri\s*=\s*\{\s*version\s*=\s*"([^"]+)"/);
  if (!match) {
    throw new Error('Tauri version not found in Cargo.toml');
  }
  return match[1];
}

/**
 * Update all README files in project root directory.
 */
function updateReadme() {
  const svelteVersion = getSvelteVersion();
  console.log(`📋 Svelte version: ${svelteVersion}`);

  const tauriVersion = getTauriVersion();
  console.log(`📋 Tauri version: ${tauriVersion}`);

  const rootFiles = readdirSync(PROJECT_ROOT);
  const readmeFiles = rootFiles.filter((f) => f.startsWith('README') && f.endsWith('.md'));
  console.log(`📖 Found ${readmeFiles.length} README files:`, readmeFiles);

  for (const filename of readmeFiles) {
    const readmeFile = join(PROJECT_ROOT, filename);
    let readme = readFileSync(readmeFile, 'utf-8');
    // use regex to replace version numbers
    readme = readme.replace(/Tauri-v[\d.]+/g, `Tauri-v${tauriVersion}`);
    readme = readme.replace(/Svelte-v[\d.]+/g, `Svelte-v${svelteVersion}`);
    writeFileSync(readmeFile, readme, 'utf-8');
    console.log(`✅ Updated ${filename}`);
  }
  console.log('🎉 All README files updated successfully');
}

updateReadme();
