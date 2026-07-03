#!/usr/bin/env python3
"""Generate Chinese meanings for English words using the DeepL API.

Reads `references/unique_words.txt` (one English word/phrase per line) and writes
`references/unique_words_with_chinese.txt` in the format `word|中文释义`.

Requires the `DEEPL_API_KEY` environment variable.
"""

import os
import re
import sys
import time
from pathlib import Path

import requests

INPUT_PATH = Path("references/unique_words.txt")
OUTPUT_PATH = Path("references/unique_words_with_chinese.txt")
API_URL = "https://api-free.deepl.com/v2/translate"
BATCH_SIZE = 50
SOURCE_LANG = "EN"
TARGET_LANG = "ZH"
SLEEP_BETWEEN_BATCHES = 0.5

# DeepL occasionally returns the source word suffixed with a locale tag
# (e.g. "apt简体中文（大陆）" or "binsimplified Chinese (大陆)").
# Strip those suffixes before deciding whether the translation is valid.
LOCALE_SUFFIX_RE = re.compile(
    r"\s*(?:"
    r"简体中文|simplified\s+chinese|"
    r"繁体中文|traditional\s+chinese"
    r")\s*[（(]\s*(?:大陆|台湾)\s*[）)]$",
    re.IGNORECASE,
)


def translate_batch(words: list[str], api_key: str) -> list[str]:
    response = requests.post(
        API_URL,
        headers={"Authorization": f"DeepL-Auth-Key {api_key}"},
        data={
            "text": words,
            "source_lang": SOURCE_LANG,
            "target_lang": TARGET_LANG,
        },
        timeout=60,
    )
    response.raise_for_status()
    return [item["text"] for item in response.json()["translations"]]


def clean_translation(translation: str, original: str) -> str:
    """Return a cleaned translation, or empty if DeepL returned the source word."""
    cleaned = translation.strip()
    cleaned = LOCALE_SUFFIX_RE.sub("", cleaned)
    return "" if cleaned.casefold() == original.casefold() else cleaned


def load_existing_meanings(path: Path) -> dict[str, str]:
    """Load previously generated output so reruns can resume without retranslating."""
    meanings: dict[str, str] = {}
    if not path.exists():
        return meanings
    for line in path.read_text(encoding="utf-8").splitlines():
        if "|" not in line:
            continue
        word, meaning = line.split("|", 1)
        meanings[word] = meaning
    return meanings


def save_meanings(path: Path, words: list[str], meanings: dict[str, str]) -> None:
    output_lines = [f"{word}|{meanings.get(word, '')}" for word in words]
    path.write_text("\n".join(output_lines) + "\n", encoding="utf-8")

def main() -> None:
    api_key = os.environ.get("DEEPL_API_KEY", "")
    if not api_key:
        raise RuntimeError("请设置环境变量 DEEPL_API_KEY")

    if not INPUT_PATH.exists():
        print(f"找不到输入文件: {INPUT_PATH}", file=sys.stderr)
        sys.exit(1)

    words = [
        line.strip()
        for line in INPUT_PATH.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    print(f"共读取 {len(words)} 个单词")

    meanings = load_existing_meanings(OUTPUT_PATH)
    if meanings:
        print(f"从已有输出中恢复 {len(meanings)} 条记录")

    missing_words = [w for w in words if not meanings.get(w)]
    print(f"需要翻译 {len(missing_words)} 个单词")

    for i in range(0, len(missing_words), BATCH_SIZE):
        batch = missing_words[i : i + BATCH_SIZE]
        print(f"翻译第 {i + 1} - {i + len(batch)} 个...")
        try:
            translations = translate_batch(batch, api_key)
            for word, translation in zip(batch, translations):
                meanings[word] = clean_translation(translation, word)
        except Exception as exc:
            print(f"本批次失败: {exc}", file=sys.stderr)
            for word in batch:
                meanings[word] = ""
        # Persist progress so reruns can resume after an interruption.
        save_meanings(OUTPUT_PATH, words, meanings)
        if i + BATCH_SIZE < len(missing_words):
            time.sleep(SLEEP_BETWEEN_BATCHES)

    save_meanings(OUTPUT_PATH, words, meanings)

    translated_count = sum(1 for m in meanings.values() if m)
    print(f"完成，共翻译 {translated_count} / {len(words)} 个单词")


if __name__ == "__main__":
    main()
