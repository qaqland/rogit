package main

import (
	"fmt"
	"log"
	"strings"

	// "reflect"
	"sort"
	"time"

	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/plumbing/filemode"
	"github.com/go-git/go-git/v5/plumbing/object"
)

type RefType uint

const (
	Branch RefType = iota // branch
	TagLWT                // lightweight tag, no tag object but commit
	TagATD                // annotated tag, has tag object
)

type Ref struct {
	Name   string
	Type   RefType
	When   time.Time
	Tag    *object.Tag
	Commit *object.Commit
}

type Repo struct {
	*git.Repository

	Head     *plumbing.Reference
	Refs     map[string]Ref
	SortTags []string
	SortBras []string
}

type RTree struct {
	Object *object.TreeEntry // ??
	Commit *object.Commit
	Dirs   []object.TreeEntry
	Files  []object.TreeEntry
}

func Init(path string) *Repo {
	repo, err := git.PlainOpen(path)
	if err != nil {
		log.Panic(err)
	}
	head, err := repo.Head()
	if err != nil {
		log.Panic(err)
	}
	r := &Repo{
		Repository: repo,
		Head:       head,
		Refs:       make(map[string]Ref, 16),
	}
	r.GetRefs()
	r.Logs("master")
	// r.Logs("main")
	return r
}

// Get all tags in the repository.
// tag name | tag hash | tag time | download
func (r *Repo) tagsAll() {
	tag_refs, _ := r.Tags()
	err := tag_refs.ForEach(func(t *plumbing.Reference) error {
		tag_name := t.Name().Short()
		tag_obj, err := r.TagObject(t.Hash())
		switch err {
		case nil:
			commit, _ := r.CommitObject(tag_obj.Target)
			r.Refs[tag_name] = Ref{
				Name:   tag_name,
				Type:   TagATD,
				When:   tag_obj.Tagger.When,
				Tag:    tag_obj,
				Commit: commit,
			}
			r.SortTags = append(r.SortTags, tag_name)
		case plumbing.ErrObjectNotFound:
			commit, _ := r.CommitObject(t.Hash())
			r.Refs[tag_name] = Ref{
				Name:   tag_name,
				Type:   TagLWT,
				When:   commit.Committer.When,
				Commit: commit,
			}
			r.SortTags = append(r.SortTags, tag_name)
		default:
			return err
		}
		return nil
	})
	if err != nil {
		log.Panic(err)
	}
}

func (r *Repo) brasAll() {
	bra_refs, _ := r.Branches()
	err := bra_refs.ForEach(func(b *plumbing.Reference) error {
		bra_name := b.Name().Short()
		if b.Type() != plumbing.HashReference {
			// TODO 处理其它类型的分支
			return plumbing.ErrInvalidType
		}
		commit, _ := r.CommitObject(b.Hash())
		r.Refs[bra_name] = Ref{
			Name:   bra_name,
			Type:   Branch,
			When:   commit.Committer.When,
			Commit: commit,
		}
		r.SortBras = append(r.SortBras, bra_name)
		return nil
	})
	if err != nil {
		log.Panic(err)
	}
}

func (r *Repo) GetRefs() {
	r.tagsAll()
	r.brasAll()
	// sort both tags and branches by time
	for _, v := range [][]string{r.SortTags, r.SortBras} {
		sort.Slice(v, func(i, j int) bool {
			return r.Refs[v[i]].When.After(r.Refs[v[j]].When)
		})
	}
	fmt.Println("Get Refs Done")
}

func (r *Repo) Logs(s string) {

	commit := r.Ref2Hash(s)
	// tree, _ := r.TreeObject(commit.TreeHash)

	// m := reflect.ValueOf(&tree).FieldByName("m")
	// fmt.Println(m.Len())
	// var commits []object.Commit
	// path := "README.md"
	logs, err := r.Log(&git.LogOptions{
		From: commit.Hash,
		// Order: git.LogOrderBSF,
		PathFilter: func(s string) bool {
			// k, err := tree.FindEntry(s)
			// // if err != nil{
			// fmt.Println(s, k, err)
			// fmt.Println(s)
			// // }
			// return s == "docs"
			return strings.HasPrefix(s, "main")
		},
		// FileName: &path,
	})
	if err != nil {
		log.Panicln(err)
	}
	i := 0
	fmt.Println(time.Now())
	err = logs.ForEach(func(c *object.Commit) error {
		if i > 9 {
			fmt.Println("object.ErrCanceled")
			return object.ErrCanceled
		}
		// commits = append(commits, *c)
		fmt.Println(time.Now(), c.Committer.When, c.Message)
		i++
		return nil
	})
	if err != nil && err != object.ErrCanceled {
		log.Panicln(err)
	}
}

func (r *Repo) Tree(tree_hash plumbing.Hash) RTree {
	tree, _ := r.TreeObject(tree_hash)
	var files RTree
	// TODO 最后一次 commit
	for _, v := range tree.Entries {
		switch v.Mode {
		case filemode.Dir:
			files.Dirs = append(files.Dirs, v)
		case filemode.Regular:
			files.Files = append(files.Files, v)
		}
	}
	for _, v := range [][]object.TreeEntry{files.Dirs, files.Files} {
		sort.Slice(v, func(i, j int) bool {
			return v[i].Name < v[j].Name
		})
	}
	// for _, v := range files.Dirs {
	// 	fmt.Println("DIR", v.Name)
	// }
	// for _, v := range files.Files {
	// 	fmt.Println("FILE", v.Name)
	// }
	return files
}

func (r *Repo) Ref2Hash(ref string) *object.Commit {

	fmt.Println("Get Tree from:", ref)

	// branch or tag first
	if key, ok := r.Refs[ref]; ok {
		return key.Commit
	}

	// here hash is the commit object hash
	hash, err := r.ResolveRevision(plumbing.Revision(ref))
	if err != nil {
		log.Panic(err)
	}

	commit, err := r.CommitObject(*hash)
	if err != nil {
		log.Panic(err)
	}

	return commit
}
