# refs

在 Git 中，引用（Reference）是指向 Git 对象的指针，可以是分支、标签、远程引用等。Git 引用有两种类型：符号引用（Symbolic Reference）和哈希引用（Hash Reference）。

符号引用是指向另一个引用的指针，而不是直接指向 Git 对象的指针。当你在 Git 仓库中切换分支时，Git 会更新 HEAD 引用来指向新的分支。例如，当你从 `master` 分支切换到 `dev` 分支时，Git 会将 HEAD 引用更新为指向 `dev` 分支。这样，当你在工作目录中进行更改时，Git 就会将这些更改提交到 `dev` 分支上。

哈希引用主要用于指向 Git 对象，例如提交对象、树对象或 Blob 对象。在 Git 中，每个 Git 对象都有一个唯一的 SHA-1 哈希值，这个哈希值可以用来唯一标识 Git 对象，哈希引用包含 Git 对象的 SHA-1 哈希值，因此可以用来指向 Git 对象。

在 Git 中，分支、标签和远程引用等引用类型都可以使用哈希引用来指向 Git 对象。例如，分支引用指向最新的提交对象，标签引用指向特定的提交对象，远程引用指向远程 Git 仓库的引用。这些引用类型都使用哈希引用来指向 Git 对象。

哈希引用还可以用于 Git 命令中的参数，例如 `git log <commit>` 命令中的 `<commit>` 参数就可以是一个哈希引用，用于指定要查看的提交对象。

---

这里选择哈希引用