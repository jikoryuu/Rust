# ■#[derive(Clone)]のような表現がありますが他に種類はありますか

はい、あります！Rustの `#[derive(...)]` 属性は、構造体や列挙型に対して**標準トレイトの自動実装**を付与するための便利な仕組みです。`Clone` 以外にも多くのトレイトが `derive` に対応しています。

---

### 🧰 よく使われる `#[derive(...)]` の一覧

| トレイト名       | 説明 |
|------------------|------|
| `Clone`          | 値の複製を可能にします（`.clone()`） |
| `Copy`           | ビット単位のコピーを許可（`Clone` より軽量）※ヒープを含む型には使えません |
| `Debug`          | `{:?}` でデバッグ出力できるようにします |
| `PartialEq`      | `==` や `!=` による比較を可能にします |
| `Eq`             | 完全な等価性（`PartialEq` の上位） |
| `PartialOrd`     | `<`, `>`, `<=`, `>=` の比較を可能にします（部分順序） |
| `Ord`            | 完全な順序付けを可能にします（`PartialOrd` の上位） |
| `Default`        | `::default()` による初期値生成を可能にします |
| `Hash`           | ハッシュ化を可能にします（`HashMap` のキーなどに必要） |

---

### 🧪 使用例

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pai {
    number: u8,
    red: bool,
}
```

これで `Pai` は `clone()` でき、`==` で比較でき、`{:?}` で表示できます。

---

### 🧠 補足：自動実装できないケース

- フィールドに `Clone` を実装していない型があると `#[derive(Clone)]` は使えません。
- `Copy` は `String` や `Vec<T>` のようなヒープ所有型には使えません。

---

カキツバタさんの麻雀プロジェクトでは、`Debug` や `PartialEq`、`Clone` あたりは特に重宝しそうですね。  
もし `Default` を使って初期化ロジックを簡略化したい場面があれば、そちらの活用もおすすめです！

他にも気になるトレイトがあれば、個別に解説できますよ。どれを深掘りしてみましょう？

# ■::default() による初期値生成を可能にします　とは何ですか

`::default()` は、Rustで**構造体や型に対して「標準の初期値」を生成するための仕組み**です。  
たとえば、`String::default()` は空文字列 `""` を返し、`i32::default()` は `0` を返します。

---

### 🧪 例：基本型の default()

```rust
let s: String = String::default(); // ""
let n: i32 = i32::default();       // 0
let b: bool = bool::default();     // false
```

---

### 🧱 構造体で使うには `#[derive(Default)]`

```rust
#[derive(Default)]
struct Player {
    name: String,
    score: i32,
}

fn main() {
    let p = Player::default();
    println!("{}: {}", p.name, p.score); // → 空文字と0
}
```

このように、**すべてのフィールドが `Default` を実装していれば、構造体全体にも `Default` を自動導出できます**。

---

### 🧠 カスタム初期値を定義したいとき

```rust
struct Player {
    name: String,
    score: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "カキツバタ".to_string(),
            score: 25000,
        }
    }
}
```

---

### 🌟 応用：一部だけ指定して残りはデフォルト

```rust
let p = Player {
    name: "東家".to_string(),
    ..Default::default()
};
```

この `..Default::default()` は、**指定していないフィールドにだけデフォルト値を補完**してくれる便利な構文です。

---

カキツバタさんの麻雀ゲームでも、`Board` や `Pai` の初期化に `Default` を活用すれば、テストや一時生成がぐっと楽になりますよ。  
もし `Default` を導入したい構造体があれば、一緒に実装してみましょうか？

