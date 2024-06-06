package handle

import (
	"math/rand"
	"time"

	"github.com/jequi_go"
)

func init() {
	rand.NewSource(time.Now().Unix())
}

func HandleRequest(req jequi_go.Request, resp jequi_go.Response) {
}

func HandleProxyRequest(req jequi_go.Request, resp jequi_go.Response) *string {

	address := []string{"furi-1:3000", "furi-2:3000"}
	addr := address[rand.Intn(len(address))]

	return &addr
}
