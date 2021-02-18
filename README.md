# 日めくりのRust版

_Rustで作る日めくり機能_

```markdown
cd himekuri_rust

cargo run
```

#### 結果

```markdown
時刻を表示 : 2021年02月18日 : 13時30分57秒 : 木曜日
来年の1月1日まであと : 317 日です
令和03年02月18日 : R03年02月18日
日めくり数え番号 : 1.0.0
```

_RubyからRustをffiで呼ぶ_

```markdown
gem install ffi

cd himekuri_rust

ruby himekuri.rb
```

#### 結果

```markdown
時刻を表示 : 2021年02月18日 : 14時17分31秒 : 木曜日
来年の1月1日まであと : 317 日です
令和03年02月18日 : R03年02月18日
日めくり数え番号 : 1.0.0
```
