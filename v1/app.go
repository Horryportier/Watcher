package v1

import (
	"fmt"
)

var (
        name string = "hide on bush"
        region string = "kr"
)

func Start() error {

        

	var p Player
	var err error
	p, err = p.GetPlayer(name, region)

	if err != nil {
		return err
	}
	fmt.Printf("%v", p)

	return err
}
