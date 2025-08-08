package main

import (
	httphandler "http-request/gen/betty-blocks/http-request/http"
	"io"
	"net/http"

	wasihttp "http-request/http-utils"

	"go.bytecodealliance.org/cm"
)

func init() {
	httphandler.Exports.Run = Run
}

const TYPICODE_URL = "https://jsonplaceholder.typicode.com/"

func Run(endpoint string) cm.Result[string, string, string] {
	if endpoint == "" {
		return cm.Err[cm.Result[string, string, string]]("endpoint cannot be empty")
	}

	url := TYPICODE_URL + "/" + endpoint
	request, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return cm.Err[cm.Result[string, string, string]](err.Error())
	}

	resp, err := wasihttp.DefaultTransport.RoundTrip(request)
	if err != nil {
		return cm.Err[cm.Result[string, string, string]](err.Error())
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return cm.Err[cm.Result[string, string, string]]("failed to fetch data: " + resp.Status)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return cm.Err[cm.Result[string, string, string]](err.Error())
	}

	return cm.OK[cm.Result[string, string, string]](string(body))

}

func main() {}
