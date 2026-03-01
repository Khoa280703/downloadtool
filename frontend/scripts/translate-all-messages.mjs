import fs from 'node:fs/promises';
import path from 'node:path';

const SOURCE_LOCALE = 'en';
const SOURCE_FILE = path.resolve(process.cwd(), 'messages', `${SOURCE_LOCALE}.json`);
const MESSAGES_DIR = path.resolve(process.cwd(), 'messages');

const TARGET_LANGS = [
	'ar',
	'bg',
	'cs',
	'da',
	'de',
	'el',
	'es',
	'et',
	'fi',
	'fr',
	'hu',
	'id',
	'it',
	'ja',
	'ko',
	'lt',
	'lv',
	'nb',
	'nl',
	'pl',
	'pt',
	'pt-BR',
	'ro',
	'ru',
	'sk',
	'sl',
	'sv',
	'tr',
	'uk',
	'zh',
	'zh-TW',
	'vi'
];

function parseArg(name) {
	const prefix = `${name}=`;
	const found = process.argv.find((arg) => arg.startsWith(prefix));
	return found ? found.slice(prefix.length) : undefined;
}

function hasFlag(name) {
	return process.argv.includes(name);
}

function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function mapTargetLocale(locale) {
	if (locale === 'nb') return 'no';
	return locale;
}

function protectTokens(input) {
	const tokens = [];
	let normalized = input;

	normalized = normalized.replace(/\{[^{}]+\}/g, (match) => {
		const token = `__PARAGLIDE_PH_${tokens.length}__`;
		tokens.push({ token, value: match });
		return token;
	});

	normalized = normalized.replace(/<[^>]+>/g, (match) => {
		const token = `__PARAGLIDE_TAG_${tokens.length}__`;
		tokens.push({ token, value: match });
		return token;
	});

	return {
		normalized,
		restore(translated) {
			let restored = translated;
			for (const token of tokens) {
				restored = restored.replaceAll(token.token, token.value);
			}
			return restored;
		}
	};
}

async function translateWithGoogleWeb(text, targetLocale) {
	const target = mapTargetLocale(targetLocale);
	const endpoint = new URL('https://translate.googleapis.com/translate_a/single');

	endpoint.searchParams.set('client', 'gtx');
	endpoint.searchParams.set('sl', SOURCE_LOCALE);
	endpoint.searchParams.set('tl', target);
	endpoint.searchParams.set('dt', 't');
	endpoint.searchParams.set('q', text);

	const response = await fetch(endpoint, {
		headers: {
			'User-Agent': 'Mozilla/5.0'
		}
	});

	if (!response.ok) {
		throw new Error(`HTTP ${response.status} from Google Translate endpoint`);
	}

	const payload = await response.json();
	const chunks = Array.isArray(payload?.[0]) ? payload[0] : [];
	return chunks.map((part) => (Array.isArray(part) ? String(part[0] ?? '') : '')).join('');
}

async function translateWithRetry(text, targetLocale, retries) {
	let lastError;
	for (let attempt = 1; attempt <= retries; attempt += 1) {
		try {
			return await translateWithGoogleWeb(text, targetLocale);
		} catch (error) {
			lastError = error;
			if (attempt < retries) {
				await sleep(attempt * 1200);
			}
		}
	}
	throw lastError;
}

async function readSourceMessages() {
	const raw = await fs.readFile(SOURCE_FILE, 'utf8');
	const parsed = JSON.parse(raw);

	if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
		throw new Error(`Invalid JSON object in ${SOURCE_FILE}`);
	}

	for (const [key, value] of Object.entries(parsed)) {
		if (typeof value !== 'string') {
			throw new Error(`Only string values are supported. Key "${key}" is ${typeof value}.`);
		}
	}

	return parsed;
}

async function ensureDir() {
	await fs.mkdir(MESSAGES_DIR, { recursive: true });
}

function selectTargetLanguages() {
	const only = parseArg('--langs');
	if (!only) return TARGET_LANGS;

	const requested = only
		.split(',')
		.map((part) => part.trim())
		.filter(Boolean);

	for (const locale of requested) {
		if (!TARGET_LANGS.includes(locale)) {
			throw new Error(`Unsupported locale in --langs: ${locale}`);
		}
	}

	return requested;
}

async function translateLanguage(sourceMessages, locale, options) {
	const outFile = path.join(MESSAGES_DIR, `${locale}.json`);
	const force = options.force;
	const delayMs = options.delayMs;
	const retries = options.retries;

	if (!force) {
		try {
			await fs.access(outFile);
			console.log(`⏭  Skip ${locale} (file exists). Dùng --force để ghi đè.`);
			return;
		} catch {
			// file not found -> continue
		}
	}

	const entries = Object.entries(sourceMessages);
	const result = {};
	console.log(`🌐 Translating ${locale} (${entries.length} keys)`);

	for (let i = 0; i < entries.length; i += 1) {
		const [key, sourceText] = entries[i];
		const protectedText = protectTokens(sourceText);
		const translatedProtected = await translateWithRetry(protectedText.normalized, locale, retries);
		const translated = protectedText.restore(translatedProtected).trim();

		result[key] = translated || sourceText;
		process.stdout.write(`  ${locale} ${i + 1}/${entries.length}\r`);

		if (delayMs > 0) {
			await sleep(delayMs);
		}
	}

	process.stdout.write('\n');
	await fs.writeFile(outFile, `${JSON.stringify(result, null, 2)}\n`, 'utf8');
	console.log(`✅ Saved ${outFile}`);
}

async function main() {
	const force = hasFlag('--force');
	const delayMs = Number(parseArg('--delay-ms') ?? '120');
	const retries = Number(parseArg('--retries') ?? '3');
	const locales = selectTargetLanguages();

	if (Number.isNaN(delayMs) || delayMs < 0) {
		throw new Error('--delay-ms phải là số >= 0');
	}

	if (Number.isNaN(retries) || retries < 1) {
		throw new Error('--retries phải là số >= 1');
	}

	const sourceMessages = await readSourceMessages();
	await ensureDir();

	console.log(`Source keys: ${Object.keys(sourceMessages).length}`);
	console.log(`Locales: ${locales.join(', ')}`);
	console.log(`Mode: google-web | force=${force} | delay=${delayMs}ms | retries=${retries}`);

	for (const locale of locales) {
		await translateLanguage(sourceMessages, locale, { force, delayMs, retries });
	}

	console.log('\n🎉 Translation completed.');
}

main().catch((error) => {
	console.error('❌ Translation failed:', error instanceof Error ? error.message : error);
	process.exit(1);
});
