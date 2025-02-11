# IME toggle

IMEの半角全角切り替えを行います。

## 前提

Windowsの設定で変換キー無変換キーをIMEオンオフに指定する必要があります。

![image](https://github.com/user-attachments/assets/633de57d-04fb-4e0a-bd38-7ef3e3053e6e)

## 使い方

```sh
cargo build --release
```

ビルドしたらパスの通ったディレクトリに置けば動作します。

```sh
# 全角入力
ime_toggle.exe ja
# 半角入力
ime_toggle.exe en
```

## 備考

ImmSetConversionStatus APIでやろうとしたところ、うまくいきませんでした。
わかる方はIssueいただけるとありがたいです。
