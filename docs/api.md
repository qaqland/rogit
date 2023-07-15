# api

前缀较为简单，自行组织

* `/repo`
* `/user/repo`

## 首页 

	/

展示 README 等其它说明信息，即

	/blob/main/README.md

## 引用（分支与标签）

	/refs

分支跳转对应的 tree，标签跳转对应的 commit

## 历史

	/log

由于 tag 与 barnch 的名称一般不相同（前者会带个v），因此对 tag、branch、hash 不做区分。

	/log/main
	/log/v0.1.0

查看某一文件的提交历史

	/log/main/src/main.rs
	/log/v0.1.0/src/main.rs

为了方便 hash 的缩略，在此处使用 `-` 占位符，而把 hash 放在最后

	/log/-/src/main.rs/5e0b02d2
	/log/-/src/main.rs/5e0b02d248653b1434c3317f1654cb5c6f011320

## 提交

	/commit/5e0b02d2
	/commit/5e0b02d248653b1434c3317f1654cb5c6f011320

## 提交时的文件树

	/tree

没人拿 tree 本身的 oid 当索引，所以此处的 hash 是对应 commit 的 hash

	/tree/5e0b02d2
	/tree/5e0b02d248653b1434c3317f1654cb5c6f011320

会有分支名称恰好等于 hash 吗？

## 文本

	/blob

检测到目录但使用 blob 时会自动跳转到相应的 tree

	/blob/main/book.toml
	/blob/v0.1.0/book.toml
	/blob/-/book.toml/5e0b02d248653b1434c3317f1654cb5c6f011320

日后可能会做的：raw/plain、download、blame 等

## 目录 

	/tree

检测到文本但使用 tree 时会自动跳转到相应的 blob

	/tree/main/src
	/tree/v0.1.0/src
	/tree/-/src/5e0b02d248653b1434c3317f1654cb5c6f011320
