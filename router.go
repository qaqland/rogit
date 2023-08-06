package main

import (
	// "fmt"
	// "strings"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/template/html/v2"
)

func Serve(host string, port string) {

	engine := html.New("./web/layout", ".html")
	engine.Reload(true)
	engine.Debug(true)
	app := fiber.New(fiber.Config{
		Views: engine,
	})

	// 相对于 go.mod 的路径
	app.Static("/static", "web/static")

	// home page, one repository has one home page
	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World 👋!")
	})
	// app.Get("/code")          // git clone link and XXX.md showed here
	app.Get("/refs", do_refs) // branches and tags

	// app.Get("/commit/:hash")
	// app.Get("/tag/:hash")

	// action: log, tree, blob, blame, diff, raw, zip, targz
	// ref: v0.1.0, main, hash
	app.Get("/:action/:ref", do_action_ref) //log/main
	// app.Get("/:action/:ref/*path")       //log/main/README
	// app.Get("/:action/-/*path_and_hash") //log/-/src/main.go/5e0b02d2

	app.Listen(host + ":" + port)
}

func do_refs(c *fiber.Ctx) error {
	return c.Render("refs", fiber.Map{
		"Repo": R,
	}, "base")
}

func do_action_ref(c *fiber.Ctx) error {
	action := c.Params("action")
	ref := c.Params("ref")
	// TODO 404
	switch action {
	case "tree":

		return c.Render("tree", fiber.Map{
			"Repo":   R,
			"Commit": R.Ref2Hash(ref),
			"Tree":   R.Tree(R.Ref2Hash(ref).TreeHash),
		}, "base")

	}
	return nil
}

// func do_path_and_end(c *fiber.Ctx) error {
// 	link := strings.Split(c.Params("path_and_hash"), "/")
// 	// will Get the 1st / so parts[0] is empty
// 	path := strings.Join(link[1:len(link)-1], "/")
// 	hash := link[len(link)-1]
// 	fmt.Println(http.StatusOK, path, hash)
// 	return nil
// }
