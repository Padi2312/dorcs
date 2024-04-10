//go:build ignore

//go:generate bash prebuild.sh
//go:generate go run gen.go

package main

import (
	"encoding/json"
	"os"
	"runtime"

	"github.com/invopop/jsonschema"
	"github.com/padi2312/dorcs/internal"
)

func main() {

	if runtime.GOOS == "windows" || runtime.GOOS == "unix" || runtime.GOOS == "linux" {
		println("Prebuild script executed successfully.")
		println("Generating JSON schema for config file.")

		reflector := jsonschema.Reflector{ExpandedStruct: true}
		schema := reflector.Reflect(&internal.Config{})
		schemaJSON, err := json.MarshalIndent(schema, "", "  ")
		if err != nil {
			panic(err)
		}
		// Write the schema to a file named dorcs.config.schema.json
		err = os.WriteFile("../dorcs.config.schema.json", schemaJSON, 0644)
		if err != nil {
			panic(err)
		}
		println("JSON schema generated successfully.")
	} else {
		println("Unsupported OS.")
	}
}
