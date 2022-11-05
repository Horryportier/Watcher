package v1

import "fmt"

func Start() error {
	var p Player
	var err error
	p, err = p.Parse()

	if err != nil {
		return err
	}
	fmt.Printf("%v", p)

	return err
}
