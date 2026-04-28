Rust 60本ノック
===============
Zenn
- https://zenn.dev/satoshindrome/books/eaae0ff83e2c00

実行例
------
```sh
% cargo run --bin nlp60_001
```

一括クリーン
------------
```sh
% find . -name Cargo.toml -execdir cargo clean \;
```

Chapters
--------
- [Lv.1 基本構文と制御フロー（001〜010）](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/3e309b)
  - **001: Hello World**  
    お題: 画面に`Hello, Rust World!`と表示する。
  - **002: おみくじアプリ**  
    お題: `大吉`、`中吉`、`凶` のいずれかをランダムに表示する。
  - **003: 変数と計算（四則演算）**  
    お題: 縦10cm、横20cmの長方形の面積を計算して表示する。
  - **004:偶数・奇数判定**  
    お題: 定数に入れた数字が偶数か奇数かを判定する。
  - **005: FizzBuzz**  
    お題: 1から30までの数字を表示する。ただし3の倍数は`Fizz`、5の倍数は`Buzz`、両方の倍数は`FizzBuzz`と表示する。
  - **006: 九九の表**  
    お題: 1の段から9の段までの掛け算の結果を表示する。
  - **007: 温度変換ツール**  
    お題: 摂氏(C)の数値を入力させ、華氏(F)に変換して表示する。計算式:`F = C * 1.8 + 32`
  - **008: 文字列の反転**  
    お題: `Rust`という文字列を逆順にして表示する。
  - **009: 配列と合計値**  
    お題: `[10, 20, 30, 40, 50]`という配列の合計値を計算する。
  - **010: 簡易メモ帳（ファイル書き込み）**  
    お題: ユーザーが入力した文字列を`memo.txt`というファイルに保存する。
- [Lv.2 データ構造とファイル操作（011〜020）](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/6c16dd)
  - **011: 数当てゲーム (High & Low)**  
    お題: 1〜100の数字をランダムに決め、ユーザーが当てるまでループする。「もっと大きい」「もっと小さい」とヒントを出す。
  - **012: 簡易電卓**  
    お題: 2つの数字と演算子（`+`, `-`, `*`, `/`）を受け取り、計算結果を表示する。
  - **013: じゃんけんゲーム**  
    お題: グー・チョキ・パーをランダムに出し、勝敗を判定する。
  - **014: CSVデータ解析**  
    お題: カンマ区切りの文字列データを分解し、整形して表示する。
- [Lv.3 アルゴリズムと所有権の深掘り（021〜030）](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/07a3d2)
- [Lv.4 実用CLIツールと外部クレート（031〜040）](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/299ad2)
- [Lv.5 データベースとリリース工学](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/339acb)
- [Lv.6 非同期処理と並行プログラミング（051〜060）](https://zenn.dev/satoshindrome/books/eaae0ff83e2c00/viewer/f7c4da)
