# ROGit(WIP)

Mininal Read Only Git web interface.

Goal: Smaller but ~~faster~~ than GitLab and Gitea.

## Usage

```bash
Usage: rogit [OPTIONS] [ROGIT-PATH]

  Options:
        --bind <BIND>  bind to interface [default: 127.0.0.1]
        --port <PORT>  port to listen on [default: 8080]
        --update       update git repositories to rogit.db
    -h, --help         Print help
    -V, --version      Print version
```

1. Create working directory `[ROGIT-PATH]`
2. Add repositories with softlinks
3. Update `rogit.db`
4. Server...

Example:

```bash
$ ls -lh
total 33M
drwxr-xr-x    2 qaq      qaq           50 Jan 20 14:51 .
drwxr-xr-x    6 qaq      qaq          279 Jan 20 15:51 ..
lrwxrwxrwx    1 qaq      qaq           32 Jan 20 14:45 apk-tools -> /home/qaq/mirrors/apk-tools.git/
lrwxrwxrwx    1 qaq      qaq           26 Jan 20 14:51 gcc -> /home/qaq/mirrors/gcc.git/
-rw-r--r--    1 qaq      qaq        32.5M Jan 20 15:16 rogit.db
```

## TODO

- New `git diff` based on SQLite cache
- Commit detail like `git branch --contain <commit-hash>`
- Server and HTML pages

## Limitation

- Only show tags on Commit Object
- No `git-blame` support
- No `.mailmap` support
- No `git-note` support

