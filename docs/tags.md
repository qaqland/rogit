# tags

在 Git 中，有两种类型的标签：轻量标签（lightweight tag）和附注标签（annotated tag）。

轻量标签只是一个指向特定提交的引用，它不包含任何其他信息。它们非常轻量，因为它们只是一个指针，所以它们通常用于临时或私有的标记。

附注标签是一个完整的 Git 对象，它包含标签的名称、标签的创建者、标签的创建日期、标签的注释等信息。它们通常用于发布版本或重要的里程碑。

因此，如果想要发布一个版本或重要的里程碑，建议使用附注标签。

---

```bash
# 列出所有 tag 的记录哈希
git show-ref --tag

# 查看 tag 类型 tag/commit
git cat-file -t bd38425

# 查看哈希内容
git cat-file -p 736622fb
```

两种用的都有，比如 gcc 用的是轻量标签发布版本，注意哈希值是 commit 的

- https://gcc.gnu.org/git/?p=gcc.git;a=tag;h=3585d89e56b9581657e02bff18254f1c3712fd8f
- https://gcc.gnu.org/git/?p=gcc.git;a=commit;h=d04fe5541c53cb16d1ca5c80da044b4c7633dbc6

而 Linux kernel 使用附注标签

- https://git.kernel.org/pub/scm/linux/kernel/git/abelloni/linux.git/tag/?h=rtc-6.5
