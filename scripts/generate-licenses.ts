#!/usr/bin/env node
import { execSync } from 'child_process';
import { readFileSync, writeFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __DIRNAME = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(__DIRNAME, '..');
const BACKEND_JSON = join(__DIRNAME, '.backend.json');
const FRONTEND_JSON = join(__DIRNAME, '.frontend.json');
const OUTPUT_PATH = join(PROJECT_ROOT, 'LICENSES.md');

interface Package {
  name: string;
  version?: string | null;
  repository?: string | null;
  description?: string | null;
  license?: string | null;
  licenses?: string | null;
}

/**
 * Execute license-checker command
 *
 * https://github.com/davglass/license-checker
 */
function runLicenseChecker() {
  console.log('🔍 Running license-checker for frontend dependencies...');
  try {
    const command = `pnpm dlx license-checker --direct --json --customPath ./scripts/.format.json --out ${FRONTEND_JSON}`;
    execSync(command, {
      cwd: PROJECT_ROOT,
      stdio: 'inherit'
    });
    console.log('✅ Frontend license data generated successfully');
  } catch (error) {
    console.error('❌ Error running license-checker:', error);
    throw error;
  }
}

/**
 * Execute cargo license command
 *
 * https://github.com/onur/cargo-license
 */
function runCargoLicense() {
  console.log('🔍 Running cargo license for backend dependencies...');
  try {
    const command = `cargo license --direct-deps-only -j --manifest-path ./src-tauri/Cargo.toml -o ${BACKEND_JSON}`;
    execSync(command, {
      cwd: PROJECT_ROOT,
      stdio: 'inherit'
    });
    console.log('✅ Backend license data generated successfully');
  } catch (error) {
    console.error('❌ Error running cargo license:', error);
    throw error;
  }
}

/**
 * Parse frontend dependencies JSON file
 *
 * @returns Array of dependency data
 */
function parseFrontendJson(): Package[] {
  console.log('📖 Parsing frontend license data...');
  try {
    const jsonContent = readFileSync(FRONTEND_JSON, 'utf-8');
    const data: { [key: string]: Package } = JSON.parse(jsonContent);
    const packages = new Map<string, Package>();
    for (const pkg of Object.values(data)) {
      // map licenses field to license
      packages.set(pkg.name, { ...pkg, license: pkg.licenses });
    }
    console.log(`✅ Found ${packages.size} frontend packages`);
    return Array.from(packages.values());
  } catch (error) {
    console.error('❌ Error parsing frontend JSON file:', error);
    throw error;
  }
}

/**
 * Parse backend dependencies JSON file
 *
 * @returns Array of dependency data
 */
function parseBackendJson(): Package[] {
  console.log('📖 Parsing backend license data...');
  try {
    const jsonContent = readFileSync(BACKEND_JSON, 'utf-8');
    const data: Package[] = JSON.parse(jsonContent);
    const packages = new Map<string, Package>();
    for (const pkg of data) {
      packages.set(pkg.name, pkg);
    }
    console.log(`✅ Found ${packages.size} backend packages`);
    return Array.from(packages.values());
  } catch (error) {
    console.error('❌ Error parsing backend JSON file:', error);
    throw error;
  }
}

/**
 * Generate Markdown table for dependency data
 *
 * @param packages - Dependency data
 * @param title - Table title
 * @returns Array of table rows
 */
function generateTable(packages: Package[], title: string): string[] {
  const table: string[] = [];
  // filter out the project itself
  const pkgs = packages.filter((pkg) => pkg.name !== 'text-go');
  // add title
  table.push(`## ${title}\n`);
  table.push(`> **${pkgs.length}** packages included\n`);
  // add table header
  table.push('| Package | Version | License | Description |');
  table.push('|---------|---------|---------|-------------|');
  // add table content
  for (const pkg of pkgs) {
    const name = pkg.repository ? `[${pkg.name}](${pkg.repository})` : pkg.name;
    const version = pkg.version || '-';
    const license = pkg.license || 'Unknown';
    // escape pipes in description to avoid breaking table structure
    const description = (pkg.description || '-').replace(/\|/g, '\\|');

    table.push(`| ${name} | ${version} | ${license} | ${description} |`);
  }
  table.push('');
  return table;
}

/**
 * Generate complete Markdown document
 *
 * @param frontendData - Frontend dependency data
 * @param backendData - Backend dependency data
 */
function generateMarkdown(frontendData: Package[], backendData: Package[]) {
  const markdown: string[] = [];
  // add document title
  markdown.push('# Third-Party License Notices\n');
  markdown.push(
    `> This document was automatically generated on ${new Date().toLocaleString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      timeZoneName: 'short'
    })}\n`
  );
  // add frontend dependencies table
  markdown.push(...generateTable(frontendData, 'Frontend Dependencies'));
  // add backend dependencies table
  markdown.push(...generateTable(backendData, 'Backend Dependencies'));

  console.log('📝 Writing markdown file...');
  try {
    writeFileSync(OUTPUT_PATH, markdown.join('\n'), 'utf-8');
    console.log('✅ Markdown file generated:', OUTPUT_PATH);
  } catch (error) {
    console.error('❌ Error writing markdown file:', error);
    throw error;
  }
}

// 1. Run license-checker
runLicenseChecker();
// 2. Run cargo license
runCargoLicense();
// 3. Parse and generate Markdown file
generateMarkdown(parseFrontendJson(), parseBackendJson());
