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
 * 执行 license-checker 命令
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
 * 执行 cargo license 命令
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
 * 解析前端依赖 JSON 文件
 *
 * @returns 依赖数据数组
 */
function parseFrontendJson(): Package[] {
  console.log('📖 Parsing frontend license data...');
  try {
    const jsonContent = readFileSync(FRONTEND_JSON, 'utf-8');
    const data: { [key: string]: Package } = JSON.parse(jsonContent);
    const packages: Package[] = Object.values(data).map((pkg) => ({
      ...pkg,
      // 把 licenses 字段映射到 license
      license: pkg.licenses
    }));
    console.log(`✅ Found ${packages.length} frontend packages`);
    return packages;
  } catch (error) {
    console.error('❌ Error parsing frontend JSON file:', error);
    throw error;
  }
}

/**
 * 解析后端依赖 JSON 文件
 *
 * @returns 依赖数据数组
 */
function parseBackendJson(): Package[] {
  console.log('📖 Parsing backend license data...');
  try {
    const jsonContent = readFileSync(BACKEND_JSON, 'utf-8');
    const data: Package[] = JSON.parse(jsonContent);
    console.log(`✅ Found ${data.length} backend packages`);
    return data;
  } catch (error) {
    console.error('❌ Error parsing backend JSON file:', error);
    throw error;
  }
}

/**
 * 生成依赖数据 Markdown 表格
 *
 * @param packages - 依赖数据
 * @param title - 表格标题
 * @returns 表格行数组
 */
function generateTable(packages: Package[], title: string): string[] {
  const table: string[] = [];
  // 过滤掉项目本身
  const pkgs = packages.filter((pkg) => pkg.name !== 'text-go');
  // 添加标题
  table.push(`## ${title}\n`);
  table.push(`> **${pkgs.length}** packages included\n`);
  // 添加表格头
  table.push('| Package | Version | License | Description |');
  table.push('|---------|---------|---------|-------------|');
  // 添加表格内容
  for (const pkg of pkgs) {
    const name = pkg.repository ? `[${pkg.name}](${pkg.repository})` : pkg.name;
    const version = pkg.version || '-';
    const license = pkg.license || 'Unknown';
    const description = pkg.description || '-';

    table.push(`| ${name} | ${version} | ${license} | ${description} |`);
  }
  table.push('');
  return table;
}

/**
 * 生成完整的 Markdown 文档
 *
 * @param frontendData - 前端依赖数据
 * @param backendData - 后端依赖数据
 */
function generateMarkdown(frontendData: Package[], backendData: Package[]) {
  const markdown: string[] = [];
  // 添加文档标题
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
  // 添加前端依赖表格
  markdown.push(...generateTable(frontendData, 'Frontend Dependencies'));
  // 添加后端依赖表格
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

// 1. 运行 license-checker
runLicenseChecker();
// 2. 运行 cargo license
runCargoLicense();
// 3. 解析并生成 Markdown 文件
generateMarkdown(parseFrontendJson(), parseBackendJson());
