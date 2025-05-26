# 🧭 Kairo – A Zettelkasten-Oriented Personal Knowledge CLI

**Kairo** は、Zettelkastenの原則に基づき、知識やタスク、プロジェクト、リンクをCLIで一元管理できるツールです。
ノートは `.md` 形式で保存され、メタデータはSQLiteで管理。CLIから高速に作成・取得・リンク付けが可能です。

---

## 🚀 Features

- Zettelkasten ノートの作成・管理（Fleeting / Permanent ノート）
- Project / Task / Tag の関連付けとフィルター機能
- ノート同士のリンク機能（双方向リンク管理）
- Markdown 出力（Frontmatter + Body）
- CLIベースの素早い操作性と柔軟なフィルタリング
- SQLiteによる軽量な永続化

---

## 🛠️ Installation

```bash
git clone https://github.com/kyotalab/kairo.git
cd kairo
cargo install --path .
```

---

## ⚙️ Configuration

`kairo` は以下の順に設定ファイル (`config.toml`) を探索します：

`$HOME/.config/kairo/config.toml`

```toml
[paths]
markdown_dir = "~/zettelkasten/notes"
```

---

## 🧑‍💻 Usage

```bash
kairo <ENTITY> <SUBCOMMAND> [OPTIONS]
```

例：

```bash
kairo note create --title "Rustとメモリ管理" --note-type fleeting --sub-type log --tag rust,memory
kairo note list --archived false --deleted false --tag rust
kairo project create --title "個人プロダクト" --description "CLIベースのTUIツール"
```

---

## 🗂️ Supported Entities & Commands

### 📝 `kairo note`

| Command      | Description                                 |
|--------------|---------------------------------------------|
| `create`     | Create a new note with metadata              |
| `list`       | List notes with filters (archived, tags etc.)|
| `get`        | Show a specific note                         |
| `update`     | Update title, type, or associations          |
| `archive`    | Archive a note                               |
| `delete`     | Soft-delete a note                           |
| `purge`      | Permanently delete a note                    |
| `unarchive`  | Unarchive a note                             |
| `restore`    | Restore a soft-deleted note                  |

---

### 📁 `kairo project` / `kairo task`

Project と Task も同様のコマンド群をサポートしています：

- `create`, `list`, `get`, `update`, `archive`, `delete`, `purge`, `unarchive`, `restore`

---

### 🔗 `kairo link`

ノート間のリンクを管理します。

| Command   | Description                         |
|-----------|-------------------------------------|
| `create`  | Link two notes by ID                |
| `list`    | List existing links                 |
| `get`     | Show a specific link                |
| `delete`  | Delete a link                       |

---

### 🏷️ `kairo tag`

タグの作成や削除が可能です。

- `create`, `list`, `get`, `update`, `delete`

---

## 📄 Markdown Output

ノートを作成すると、指定ディレクトリに自動で `.md` ファイルが生成されます。例：

```markdown
---
id: n-202405261005
title: "Rustとメモリ管理"
note_type: "fleeting"
sub_type: "log"
tags: ["rust", "memory"]
project_id: "p-001"
task_id: "t-002"
created_at: "2025-05-26T10:05:00"
updated_at: "2025-05-26T10:05:00"
---

## Rustにおけるメモリ管理の要点

...
```

---

## 💡 今後の構想

- TUI (Compass View + Note Editor)
- ノートのバッファ切替機能
- フルテキスト検索の実装
- バックアップ／同期（S3など）

---

## 📝 License

MIT

---

## 🤝 Contributing

Issues や Pull Requests は大歓迎です！
あなたのZettelkastenライフを支えるCLIを一緒に育てていきましょう。