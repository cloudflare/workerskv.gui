import fs from 'fs/promises';
import { promisify } from 'util';
import { exec } from 'child_process';
import { createRequire } from 'module';

const run = promisify(exec);
const require = createRequire(import.meta.url);

const cargo = require.resolve('../src-tauri/Cargo.toml');
const tauri = require.resolve('../src-tauri/tauri.conf.json');

function bail(msg) {
	console.error('ERROR', msg);
	process.exit(1);
}

(async function () {
	let [type] = process.argv.slice(2);
	if (!type) return bail('Missing "version" argument');
	if (!/(pre)?(major|minor|patch)/.test(type)) return bail(`Invalid version argument: "${type}"`);

	// auto-increments version
	// auto-runs `git tag v{NEXT}`
	await run(`npm version ${type}`);

	const { version } = require('../package.json');
	console.log('~> new version =', version);

	let config = require(tauri);
	config.package.version = version;
	let contents = JSON.stringify(config, null, 2);
	await fs.writeFile(tauri, contents);
	console.log('~> updated "tauri.conf.json" file');

	let toml = await fs.readFile(cargo, 'utf8');
	toml = toml.replace(
		/^version\s*=\s*"(.*)"\s*$/m,
		`version = "${version}"`
	);

	await fs.writeFile(cargo, toml);
	console.log('~> updated "Cargo.toml" file');

	await run(`git tag -d v${version}`);
	await run(`git add ${cargo} ${tauri}`);
	await run(`git commit -C HEAD --amend`);
	await run(`git tag v${version}`);
	console.log('~> DONE 🎉');
})().catch(err => {
	return bail(err.stack);
});
