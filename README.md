# Now Playing for Slack Webhook

再生中の音楽をSlackに投稿するデスクトップアプリです。/
A desktop app that posts your currently playing music to Slack.

> [!WARNING]
> 現在はUIは日本語のみです。
> UIの多言語化へのコントリビュートを歓迎しています！
> 
> The UI is currently only available in Japanese. Sorry.
> Contributions for UI localization are welcome!

[English](#english)

## 概要
現在再生中のメディアのメタデータを取得し、選択した曲をSlack Incoming Webhookで投稿します。

また、ユーザー名を設定するとIncoming Webhookで投稿される際のユーザー名が変わります。（例: `こつ子`→「こつ子の再生中の曲」）

## 機能

- 再生中の曲をリアルタイムで一覧表示（1秒ごとに自動更新）
- 複数の曲を選択して一括投稿
- ユーザー名・Webhook URLを入力保存（次回起動時に復元）
  - Windowsの標準では`C:\Users\user_name\AppData\Roaming\NowPlayingForSlackWebhook`に保存されます。

## 使い方

1. `now_playing_for_slack_webhook.exe`を実行
2. ユーザー名、投稿先の`Slack Incoming Webhook`のURLを入力
3. 再生中のメディアがあれば表示される
4. 共有したいメディアのチェックボックスをチェック
5. 投稿ボタンを押す


## 動作環境

- Windows 10 / 11（Windows Media Session API使用）
- Linux対応は開発中（実機での実行環境の準備に苦戦している）

## ビルド
```bash
cargo build --release
```

## コントリビュート

IssueやPull Requestを歓迎します！
- Linux対応
- UIの多言語化
- バグ報告・機能提案

などなど...

## Fonts
- [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP) — SIL Open Font License 1.1 ([license](https://fonts.google.com/noto/specimen/Noto+Sans+JP/license))

## License
MIT License

---

## English

A desktop app that posts your currently playing music to Slack.

## Overview
Fetches metadata from the currently playing media and posts selected tracks to Slack via Incoming Webhook.

If you set a username, it will be reflected in the Incoming Webhook post name. (e.g., `Kotuko` → "Kotukoの再生中の曲")

## Features

- Real-time list of currently playing tracks (auto-refreshed every second)
- Select multiple tracks and post them at once
- Saves username and Webhook URL between sessions
  - On Windows, settings are stored at `C:\Users\user_name\AppData\Roaming\NowPlayingForSlackWebhook` by default.

## Usage

1. Run `now_playing_for_slack_webhook.exe`
2. Enter your username and the `Slack Incoming Webhook` URL
3. Any currently playing media will appear in the list
4. Check the checkbox next to the track(s) you want to share
5. Press the post button

## Requirements

- Windows 10 / 11 (uses Windows Media Session API)
- Linux support is in progress (still working on getting a test environment set up)

## Build
```bash
cargo build --release
```

## Contributing

Issues and pull requests are welcome!
Contributions are especially appreciated in the following areas:

- Linux support
- UI localization / multi-language support
- Bug reports and feature suggestions

etc.

## Fonts
- [Noto Sans JP](https://fonts.google.com/noto/specimen/Noto+Sans+JP) — SIL Open Font License 1.1 ([license](https://fonts.google.com/noto/specimen/Noto+Sans+JP/license))

## License
MIT License
