package main

import (
	"context"
	"log"
	"net/http"

	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wsjson"
)

func echoHandler(w http.ResponseWriter, r *http.Request) {
	// Upgrade the HTTP server connection to a WebSocket connection.
	c, err := websocket.Accept(w, r, &websocket.AcceptOptions{
		InsecureSkipVerify: true,
		// Inserir opções conforme necessário, por exemplo, Subprotocols: []string{"echo"},
	})
	if err != nil {
		log.Println("Error accepting connection:", err)
		return
	}
	defer c.Close(websocket.StatusInternalError, "the sky is falling")

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	// Read message from the client
	var v interface{}
	err = wsjson.Read(ctx, c, &v)
	if err != nil {
		log.Println("Error reading message:", err)
		return
	}

	// Write "hello world" as a response.
	err = wsjson.Write(ctx, c, "hello world")
	if err != nil {
		log.Println("Error writing message:", err)
		return
	}

	// Close the connection normally.
	c.Close(websocket.StatusNormalClosure, "")
}
