# 🚀 開発マニフェスト（DEV_MANIFEST.md）

このマニフェストは、プロジェクトの目的・技術構成・開発ルール・自動化の仕様を明示します。  
AIによるコード生成や、開発者が設計意図を理解しやすくするために活用してください。

---

## 🧭 プロジェクト概要

- **名前**: to-webp
- **目的**: 入力画像を WebP 形式に変換する軽量・高速な CLI / API ツール
- **ライセンス**: MIT
- **開発者**: 有樹 大野
- **方針**: ステートレス / マルチアーキテクチャ対応 / Rustらしい設計

---

## 🏗 技術スタック

| 項目         | 使用技術           |
|--------------|--------------------|
| 言語         | Rust 1.86          |
| ビルド       | Cargo              |
| 静的解析     | cargo clippy       |
| 整形         | cargo fmt          |
| 脆弱性チェック | cargo deny, audit  |
| 依存チェック | cargo outdated     |
| CI           | GitHub Actions     |
| Docker対応   | Dockerfile, buildx |

---

## 🔍 品質チェックのためのコマンド

```bash
./lint.sh      # fmt, check, clippy, deny, audit, outdated を実行
```

> Rust/Cargo 未インストールでも実行可能（Dockerベース）

---

## 📦 プロジェクト初期化（Rust未インストールでもOK）

```bash
./new-rust.sh my-project
```

> `cargo new` 相当をDockerで生成し、MITライセンス付きの新プロジェクトを作成します。

---

## 📐 開発ルール・指針

- 全ソースコードは `cargo fmt` 済みであること
- 警告はすべて `cargo clippy -D warnings` で排除
- `deny.toml` に沿った依存・ライセンス制御を遵守
- mainブランチ以外ではDockerHubへの自動pushを行わない
- セキュリティ脆弱性は `cargo audit` にて管理

---

## 🤖 AIによるコード生成補助のためのガイドライン

- このマニフェストに書かれている目的・構造を尊重してください
- プロジェクトはステートレスな設計とし、データ保存は行いません
- 出力は CLI または API として自己完結するようにします
- 生成コードには適切なコメントとエラーハンドリングを含めてください

---

## 🧭 将来の展望（拡張アイデア）

- AVIFやHEIC形式の変換対応
- WebAssembly(WASM)対応
- マルチスレッド処理による高速化
- REST APIラッパー追加（Axum/FastAPI相当）

---

## 📚 関連ファイル

| ファイル名         | 役割                                         |
|--------------------|----------------------------------------------|
| `README.md`        | 一般ユーザー向けの簡易ドキュメント           |
| `DEV_MANIFEST.md`  | 開発者とAI向けの詳細設計マニフェスト         |
| `deny.toml`        | 依存ライセンス／バージョン制御               |
| `.github/workflows/ci.yml` | GitHub Actionsによる自動品質チェック  |
| `lint.sh`          | ローカル品質チェックスクリプト（Docker）     |
| `new-rust.sh`      | Rust未インストールでも新プロジェクト作成可能 |

---

> 本ファイルはAIにも読ませやすい構造で設計されています。  
> 自動生成やAIペアプログラミングに利用する際は、構造や意図を尊重してください。

---

## ✅ テンプレートへの組み込み方法

テンプレートプロジェクトのルートに次のように追加してください：

```sh

rust-template/
├── DEV_MANIFEST.md    👈 開発＆AI向けドキュメント
├── README.md          👈 ユーザー向けドキュメント
├── new-rust.sh
├── lint.sh
├── deny.toml
├── .github/workflows/ci.yml
└── ...

```

> `README.md` から `DEV_MANIFEST.md` へのリンクを追加しておくとより親切です。
