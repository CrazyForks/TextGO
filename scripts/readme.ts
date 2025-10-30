import { readdirSync, readFileSync, writeFileSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __DIRNAME = dirname(fileURLToPath(import.meta.url));
const ROOT_DIR = join(__DIRNAME, '..');
const PACKAGE_JSON_PATH = join(ROOT_DIR, 'package.json');
const CARGO_TOML_PATH = join(ROOT_DIR, 'src-tauri', 'Cargo.toml');

function getSvelteVersion(): string {
  const packageJson = JSON.parse(readFileSync(PACKAGE_JSON_PATH, 'utf-8'));
  const svelteVersion = packageJson.devDependencies?.svelte || packageJson.dependencies?.svelte;
  if (!svelteVersion) {
    throw new Error('Svelte version not found in package.json');
  }
  return svelteVersion.replace(/^[\^~]/, '');
}

function getTauriVersion(): string {
  const cargoToml = readFileSync(CARGO_TOML_PATH, 'utf-8');
  const match = cargoToml.match(/tauri\s*=\s*\{\s*version\s*=\s*"([^"]+)"/);
  if (!match) {
    throw new Error('Tauri version not found in Cargo.toml');
  }
  return match[1];
}

function getReadmeFiles(): string[] {
  const files = readdirSync(ROOT_DIR);
  return files.filter((file) => file.startsWith('README') && file.endsWith('.md'));
}

function updateReadme(): void {
  const svelteVersion = getSvelteVersion();
  const tauriVersion = getTauriVersion();

  console.log(`Svelte version: ${svelteVersion}`);
  console.log(`Tauri version: ${tauriVersion}`);

  const readmeFiles = getReadmeFiles();
  console.log(`Found ${readmeFiles.length} README files:`, readmeFiles);

  for (const file of readmeFiles) {
    const filePath = join(ROOT_DIR, file);
    let readme = readFileSync(filePath, 'utf-8');

    // 使用正则替换版本号
    readme = readme.replace(/Tauri-v[\d.]+/g, `Tauri-v${tauriVersion}`);
    readme = readme.replace(/Svelte-v[\d.]+/g, `Svelte-v${svelteVersion}`);

    writeFileSync(filePath, readme, 'utf-8');
    console.log(`Updated ${file}`);
  }

  console.log('All README files updated successfully!');
}

try {
  updateReadme();
} catch (error) {
  console.error('Error updating README:', error);
  process.exit(1);
}
