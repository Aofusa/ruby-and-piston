Ruby と Piston を組み合わせる
---


## 事前情報
Artichoke と Piston 使います  
Rust は Stable じゃないと動かない(1.43 stable で動作確認)  


## Rubyの準備
OpenSSLは 1.1系だとよくない  
prefix はいっぱい入れないと Artichoke のテスト通らない  
```
RUBY_CONFIGURE_OPTS="--enable-pthread --enable-shared \
--with-readline-dir=`brew --prefix readline` \
--with-openssl-dir=`brew --prefix openssl` \
--with-libyaml-dir=`brew --prefix libyaml` \
--with-opt-dir=`brew --prefix gdbm`:`brew --prefix gmp`:`brew --prefix libffi`" PKG_CONFIG_PATH=/usr/local/opt/openssl/lib/pkgconfig rbenv install "2.6.3"
```

## 実行
赤の四角（Pistonでの描画）と `result 100` (ArtichokeでのRubyの実行)が表示される  
```
cargo run
```

