#!/usr/bin/env python3
"""Generate Chinese meanings for English words using the DeepL API.

Reads `references/unique_words.txt` (one English word/phrase per line) and writes
`references/unique_words_with_chinese.txt` in the format `word|中文释义`.

Requires the `DEEPL_API_KEY` environment variable.
"""

import os
import sys
from pathlib import Path

import requests

INPUT_PATH = Path("references/unique_words.txt")
OUTPUT_PATH = Path("references/unique_words_with_chinese.txt")
API_URL = "https://api-free.deepl.com/v2/translate"
BATCH_SIZE = 50
SOURCE_LANG = "EN"
TARGET_LANG = "ZH"


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

    meanings: dict[str, str] = {}

    for i in range(0, len(words), BATCH_SIZE):
        batch = words[i : i + BATCH_SIZE]
        print(f"翻译第 {i + 1} - {i + len(batch)} 个...")
        try:
            translations = translate_batch(batch, api_key)
            for word, translation in zip(batch, translations):
                cleaned = translation.strip()
                # If DeepL returns the English word unchanged, treat it as missing
                # so the app can skip it during study mode.
                meanings[word] = "" if cleaned == word else cleaned
        except Exception as exc:
            print(f"本批次失败: {exc}", file=sys.stderr)
            for word in batch:
                meanings[word] = ""

    output_lines = [f"{word}|{meanings.get(word, '')}" for word in words]
    OUTPUT_PATH.write_text("\n".join(output_lines) + "\n", encoding="utf-8")

    translated_count = sum(1 for m in meanings.values() if m)
    print(f"完成，共翻译 {translated_count} / {len(words)} 个单词")


if __name__ == "__main__":
    main()
