# UI

HOME 页显示仓库的基本信息，包括仓库的名字、描述、分支信息、最后几次提交历史

有几个链接是仓库层面的，剩下是分支或文件层面

## branch

进入 Home 或者 Refs 会显示当前仓库的分支信息

| Name |     Who + When    |  Action  |
|------|-------------------|----------|
| main | qaqland 2023-7-21 | tree log |
| dev  | qaqland 2023-7-22 | tree log |
| page | qaqland 2023-7-19 | tree log |

只有 Action 是可以点击跳转的

## tag

| Name |    Hash + When     |  Action  |
|------|--------------------|----------|
| v0.2 | 9555f49c 2023-7-21 |  .tar.gz |
| v0.1 | 9555f49c 2023-7-21 |  .tar.gz |

除 Action 外，Hash 可以跳转到对应的 commit 或 tag 对象

## tree

| Name |    Size + When    |  Action  |
|------|-------------------|----------|
| docs |  1.2M 2023-7-21   |      log |
| src  |  1.2M 2023-7-21   |      log |
| run* |  1.2K 2023-7-21   |  raw log |

Size 是文件夹的大小，Action 的 raw 方便直接 wget 下载

## log

|    Message     |     Who + When    |  Action  |
|----------------|-------------------|----------|
| Initial commit | qaqland 2023-7-21 |   Hash   |

这里没有放置查看 browse 的入口，可以先进 commit object 再查看 tree
