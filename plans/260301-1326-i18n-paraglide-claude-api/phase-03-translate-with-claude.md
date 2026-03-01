# Phase 3: Claude API Translation Script

**Status:** completed | **Priority:** critical | **Effort:** medium (parallel sub-agent run)

## Overview

Phase này đã được thực hiện bằng **parallel sub-agent translation** (không dùng Claude API trong app runtime):
- Spawn song song nhiều sub-agent, mỗi agent dịch 1 locale.
- Giữ nguyên placeholder `{name}`.
- Ghi output vào `frontend/messages/<locale>.json`.

Kết quả hiện tại: 32 locale translated files + `en.json` source = 33 files.
Vòng polish cuối đã đổi source string thành `Hello, {name}!` và loại bỏ cụm `from en` trong các bản dịch locale.
Sau khi Phase 2 bổ sung key mới (privacy/theme/download), đã chạy thêm 1 vòng dịch song song cho **96 key mới** trên toàn bộ 32 locale non-en và merge trực tiếp vào từng file locale.

> Số lượng file locale đúng theo cấu hình hiện tại là 33 locales total (`en` + 32 non-en), theo `frontend/project.inlang/settings.json`.

## Script Design

### Input/Output
```
Input:  messages/en.json         (~180 keys)
Output: messages/ar.json
        messages/bg.json
        ... (33 files total)
```

### Target Languages (33, excluding source `en`)
```ts
const TARGET_LANGS = [
  'ar', 'bg', 'cs', 'da', 'de', 'el', 'es', 'et', 'fi',
  'fr', 'hu', 'id', 'it', 'ja', 'ko', 'lt', 'lv', 'nb', 'nl',
  'pl', 'pt', 'pt-BR', 'ro', 'ru', 'sk', 'sl', 'sv', 'tr', 'uk',
  'zh', 'zh-TW', 'vi'
];
```

> **Note:** Add `vi` (Vietnamese) to the list even though DeepL doesn't support it — Claude handles it well.

### scripts/translate-all-messages.ts

```ts
import Anthropic from '@anthropic-ai/sdk';
import fs from 'fs';
import path from 'path';

const client = new Anthropic(); // uses ANTHROPIC_API_KEY env var

const SOURCE = path.resolve('messages/en.json');
const MESSAGES_DIR = path.resolve('messages');

// Batch size: translate N keys per API call to avoid context limits
const BATCH_SIZE = 50;

const LANG_NAMES: Record<string, string> = {
  ar: 'Arabic', bg: 'Bulgarian', cs: 'Czech', da: 'Danish',
  de: 'German', el: 'Greek', es: 'Spanish', et: 'Estonian',
  fi: 'Finnish', fr: 'French', hu: 'Hungarian', id: 'Indonesian',
  it: 'Italian', ja: 'Japanese', ko: 'Korean', lt: 'Lithuanian',
  lv: 'Latvian', nb: 'Norwegian Bokmål', nl: 'Dutch', pl: 'Polish',
  pt: 'Portuguese', 'pt-BR': 'Brazilian Portuguese', ro: 'Romanian',
  ru: 'Russian', sk: 'Slovak', sl: 'Slovenian', sv: 'Swedish',
  tr: 'Turkish', uk: 'Ukrainian', zh: 'Chinese (Simplified)',
  'zh-TW': 'Chinese (Traditional)', vi: 'Vietnamese',
};

// Note: vi (Vietnamese) is included even though DeepL doesn't support it — Claude translates it well.

async function translateBatch(
  keys: Record<string, string>,
  langCode: string,
  langName: string,
  attempt = 1
): Promise<Record<string, string>> {
  const prompt = `Translate the following JSON values from English to ${langName} (${langCode}).

Rules:
- Keep JSON keys exactly as-is
- Preserve {placeholders} like {name}, {count} unchanged
- Keep HTML tags unchanged if any
- Use natural, native-sounding ${langName}
- For UI strings, keep them concise (same length as English roughly)
- Return ONLY valid JSON, no explanation

Input JSON:
${JSON.stringify(keys, null, 2)}`;

  try {
    const response = await client.messages.create({
      model: 'claude-haiku-4-5',  // haiku: fast + cheap for translation
      max_tokens: 4096,
      messages: [{ role: 'user', content: prompt }],
    });

    const text = (response.content[0] as { text: string }).text.trim();
    // Strip markdown code fences if present
    const json = text.replace(/^```json\n?/, '').replace(/\n?```$/, '');
    return JSON.parse(json);
  } catch (err) {
    if (attempt < 3) {
      const delay = attempt * 2000; // exponential backoff: 2s, 4s
      console.warn(`  Attempt ${attempt} failed for ${langCode}, retrying in ${delay}ms...`);
      await new Promise((r) => setTimeout(r, delay));
      return translateBatch(keys, langCode, langName, attempt + 1);
    }
    throw err;
  }
}

async function translateLanguage(langCode: string): Promise<void> {
  const langName = LANG_NAMES[langCode];
  const outPath = path.join(MESSAGES_DIR, `${langCode}.json`);

  // Skip if already exists (resume support)
  if (fs.existsSync(outPath)) {
    console.log(`⏭  Skipping ${langCode} (already exists)`);
    return;
  }

  console.log(`🌐 Translating → ${langName} (${langCode})...`);
  const source: Record<string, string> = JSON.parse(fs.readFileSync(SOURCE, 'utf-8'));
  const entries = Object.entries(source);
  const result: Record<string, string> = {};

  // Process in batches
  for (let i = 0; i < entries.length; i += BATCH_SIZE) {
    const batch = Object.fromEntries(entries.slice(i, i + BATCH_SIZE));
    const translated = await translateBatch(batch, langCode, langName);
    Object.assign(result, translated);
    process.stdout.write(`  batch ${Math.ceil((i + BATCH_SIZE) / BATCH_SIZE)}/${Math.ceil(entries.length / BATCH_SIZE)} ✓\n`);
  }

  // Key parity check — fail hard rather than write a partial file
  const sourceKeys = Object.keys(source);
  const resultKeys = Object.keys(result);
  if (resultKeys.length !== sourceKeys.length) {
    const missing = sourceKeys.filter((k) => !resultKeys.includes(k));
    const extra = resultKeys.filter((k) => !sourceKeys.includes(k));
    throw new Error(
      `Key mismatch for ${langCode}: expected ${sourceKeys.length} keys, got ${resultKeys.length}. ` +
      `Missing: [${missing.join(', ')}]. Extra: [${extra.join(', ')}]`
    );
  }

  fs.writeFileSync(outPath, JSON.stringify(result, null, 2) + '\n');
  console.log(`  ✅ Saved ${outPath} (${resultKeys.length} keys)`);
}

async function main() {
  const TARGET_LANGS = Object.keys(LANG_NAMES);
  console.log(`Translating ${Object.keys(JSON.parse(fs.readFileSync(SOURCE, 'utf-8'))).length} keys to ${TARGET_LANGS.length} languages\n`);

  for (const lang of TARGET_LANGS) {
    await translateLanguage(lang);
  }

  console.log('\n✅ All translations complete!');
}

main().catch(console.error);
```

## Runbook (đã chạy)

```bash
# 1) Tạo CSV locale
cat > /tmp/phase3-locales.csv

# 2) Spawn parallel agents trên từng locale
spawn_agents_on_csv ...

# 3) Parse CSV output và ghi messages/<locale>.json
node <<'NODE'
// parse CSV -> write locale json files
NODE
```

## Notes

- Không cần API key Claude trong app flow.
- Batch translation dùng output schema + key-preservation rule để tránh drift key.
- Key parity sau merge: không thiếu key so với `en.json`.

## Files Created/Updated

- `frontend/messages/ar.json`, `frontend/messages/bg.json`, ... (32 translated files)
- `frontend/messages/en.json` (source)

## Dependencies

- Không thêm dependency mới cho translation run này.

## Success Criteria

- [x] Parallel sub-agent run completed without failed items
- [x] 32 translated JSON files created in `frontend/messages/` (excluding `en.json`)
- [x] Placeholder `{name}` preserved in generated strings
- [x] `pnpm --filter frontend check` passes
- [x] `pnpm --filter frontend build` passes
- [x] Newly added Phase 2 keys translated for non-en locales
