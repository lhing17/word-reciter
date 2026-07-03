# Word Reciter

一款基于 Tauri + Vue 3 的桌面端单词背诵辅助软件，目前支持 Windows、macOS 和 Linux。

## 功能

- **分类模式**：快速浏览单词并标记为“生词”、“半熟词”或“熟词”。
  - 单词按随机顺序展示。
  - 跳过的单词会被移到队尾，稍后再出现。
  - 支持键盘快捷键 `1`（生词）、`2`（半熟词）、`3`（熟词）、`Esc`（返回首页）。
- **背诵模式**：从“生词”和“半熟词”中生成题目。
  - 看英文选中文释义。
  - 根据中文释义和首尾字母填写完整单词。
  - 看英文回想中文释义。
  - 答题后可重新标记熟悉度，标记为“熟词”后不再出现。
- **首页统计**：实时显示总单词数、生词数、半熟词数和熟词数。
- **默认词库**：内置朗文 9000 词，并附带 DeepL 翻译的中文释义。

## 技术栈

- 前端：Vue 3 + TypeScript + Vite + Pinia + Vue Router
- 后端：Tauri v2 + Rust
- 数据库：SQLite（通过 `rusqlite` 访问）
- 词库生成：Python 3 + DeepL API

## 环境要求

- Node.js 与 npm
- Rust 工具链（cargo）
- Python 3（仅用于重新生成中文释义）

## 安装依赖

```bash
npm install
```

## 开发运行

```bash
npm run tauri:dev
```

这会同时启动前端开发服务器和 Tauri 桌面窗口。

## 构建生产包

```bash
npm run tauri:build
```

打包结果位于 `src-tauri/target/release/bundle/` 目录下。

## 重新生成中文释义

默认词库 `references/unique_words_with_chinese.txt` 已经包含中文释义。如需重新生成，请先设置 DeepL API Key：

```bash
export DEEPL_API_KEY="your-key-here"
python scripts/generate_chinese_meanings.py
```

脚本会读取 `references/unique_words.txt`，调用 DeepL API 翻译，并写入 `references/unique_words_with_chinese.txt`。已翻译的单词会跳过，避免重复调用 API。

## 测试

前端测试：

```bash
npm run test
```

后端测试：

```bash
cd src-tauri
cargo test
```

代码检查：

```bash
cd src-tauri
cargo clippy --all-targets -- -D warnings
```

## 项目结构

```
D:/HL/word-reciter/
├── references/
│   ├── unique_words.txt              # 默认英文词库
│   └── unique_words_with_chinese.txt # 中英对照词库
├── scripts/
│   └── generate_chinese_meanings.py  # DeepL 中文释义生成脚本
├── src/                              # 前端代码
│   ├── api/                          # Tauri 调用封装
│   ├── components/                   # 可复用组件
│   ├── stores/                       # Pinia 状态管理
│   ├── types/                        # TypeScript 类型
│   ├── views/                        # 页面视图
│   ├── App.vue
│   ├── main.ts
│   └── router.ts
├── src-tauri/                        # 后端代码
│   ├── src/
│   │   ├── commands.rs               # Tauri 命令
│   │   ├── db/                       # 数据库迁移与操作
│   │   ├── services/                 # 业务逻辑
│   │   ├── lib.rs                    # 应用入口
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/
│   └── superpowers/                  # 设计与实现文档
├── package.json
├── vite.config.ts
└── README.md
```

## 数据存储

应用数据保存在系统应用数据目录下的 SQLite 数据库中：

- Windows：`C:\Users\<用户名>\AppData\Roaming\com.wordreciter.app\word_reciter.db`
- macOS：`~/Library/Application Support/com.wordreciter.app/word_reciter.db`
- Linux：`~/.local/share/com.wordreciter.app/word_reciter.db`

删除该文件可清空本地词库和学习记录。

## 使用提示

- 首次启动时会自动导入默认词库，导入进度根据设备性能可能需要几秒到几十秒。
- 在分类模式中，标记为“熟词”的单词会直接进入背诵模式的排除列表。
- 背诵模式只从有中文释义的“生词”和“半熟词”中出题。

## 开发计划

当前版本已实现核心 MVP 功能。后续可能扩展：

- 发音支持
- 学习数据统计与图表
- 自定义词库导入
- 更多题型与复习策略

## 许可证

待定。
