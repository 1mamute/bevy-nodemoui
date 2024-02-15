package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
	"os"

	demoinfocs "github.com/markus-wa/demoinfocs-golang/v4/pkg/demoinfocs"
	"github.com/markus-wa/demoinfocs-golang/v4/pkg/demoinfocs/msg"
)

// Run like this: go run heatmap.go -demo /path/to/demo.dem > out.jpg
func main() {
	//
	// Parsing
	//

	f, err := os.Open(DemoPathFromArgs())
	if err != nil {
		panic(err)
	}

	defer f.Close()

	p := demoinfocs.NewParser(f)
	defer p.Close()

	// Parse header (contains map-name etc.)
	header, err := p.ParseHeader()
	if err != nil {
		panic(err)
	}

	var (
		mapMetadata Map
	)

	p.RegisterNetMessageHandler(func(msg *msg.CSVCMsg_ServerInfo) {
		// Get metadata for the map that the game was played on for coordinate translations
		mapMetadata = GetMapMetadata(header.MapName, msg.GetMapCrc())
		fmt.Printf("Map metadata: %v\n", mapMetadata)
	})

	// // Register a handler for the FrameDone event which is called at the end of each tick.
	// p.RegisterEventHandler(func(e events.FrameDone) {
	// 	// Loop through all players in the game.
	// 	for _, player := range p.GameState().Participants().All() {
	// 		if player != nil && !player.IsAlive() {
	// 			continue // Skip if the player is not alive.
	// 		}
	// 		fmt.Printf("Player %s is at position %v\n", player.Name, player.Position())
	// 	}
	// })

	// Parse the whole demo
	err = p.ParseToEnd()
	if err != nil {
		panic(err)
	}

	http.HandleFunc("/echo", echoHandler)
	log.Println("WebSocket server started on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}

// DemoPathFromArgs returns the value of the -demo command line flag.
// Panics if an error occurs.
func DemoPathFromArgs() string {
	fl := new(flag.FlagSet)

	demPathPtr := fl.String("demo", "", "Demo file `path`")

	err := fl.Parse(os.Args[1:])
	if err != nil {
		panic(err)
	}

	demPath := *demPathPtr

	return demPath
}

// RedirectStdout redirects standard output to dev null.
// Panics if an error occurs.
func RedirectStdout(f func()) {
	// Redirect stdout, the resulting image is written to this
	old := os.Stdout

	r, w, err := os.Pipe()
	if err != nil {
		panic(err)
	}

	os.Stdout = w

	// Discard the output in a separate goroutine so writing to stdout can't block indefinitely
	go func() {
		for err := error(nil); err == nil; _, err = io.Copy(ioutil.Discard, r) {
		}
	}()

	f()

	os.Stdout = old
}

// Map represents a CS:GO map. It contains information required to translate
// in-game world coordinates to coordinates relative to (0, 0) on the provided map-overviews (radar images).
type Map struct {
	PosX  float64 `json:"pos_x,string"`
	PosY  float64 `json:"pos_y,string"`
	Scale float64 `json:"scale,string"`
}

// Translate translates in-game world-relative coordinates to (0, 0) relative coordinates.
func (m Map) Translate(x, y float64) (float64, float64) {
	return x - m.PosX, m.PosY - y
}

// TranslateScale translates and scales in-game world-relative coordinates to (0, 0) relative coordinates.
// The outputs are pixel coordinates for the radar images found in the maps folder.
func (m Map) TranslateScale(x, y float64) (float64, float64) {
	x, y = m.Translate(x, y)
	return x / m.Scale, y / m.Scale
}

// GetMapMetadata fetches metadata for a specific map version from
// `https://radar-overviews.csgo.saiko.tech/<map>/<crc>/info.json`.
// Panics if any error occurs.
func GetMapMetadata(name string, crc uint32) Map {
	url := fmt.Sprintf("https://radar-overviews.csgo.saiko.tech/%s/%d/info.json", name, crc)

	resp, err := http.Get(url)
	if err != nil {
		panic(err)
	}

	defer resp.Body.Close()

	var data map[string]Map

	err = json.NewDecoder(resp.Body).Decode(&data)
	if err != nil {
		panic(err)
	}

	mapInfo, ok := data[name]
	if !ok {
		panic(fmt.Sprintf("failed to get map info.json entry for %q", name))
	}

	return mapInfo
}
