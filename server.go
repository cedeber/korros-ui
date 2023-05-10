package main

import (
	"flag"
	"log"
	"net/http"
	"os"
	"strings"
)

// FileSystem custom file system handler
type FileSystem struct {
	fs http.FileSystem
}

// Open opens file
func (fs FileSystem) Open(path string) (http.File, error) {
	f, err := fs.fs.Open(path)
	if err != nil {
		return nil, err
	}

	s, err := f.Stat()
	if s.IsDir() {
		index := strings.TrimSuffix(path, "/") + "/index.html"
		if _, err := fs.fs.Open(index); err != nil {
			return nil, err
		}
	}

	return f, nil
}

func addHeaders(fs http.Handler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Add("X-Frame-Options", "DENY")
		// w.Header().Add("Access-Control-Allow-Origin", "*")
		w.Header().Add("Cross-Origin-Opener-Policy", "same-origin")
		w.Header().Add("Cross-Origin-Embedder-Policy", "require-corp")
		w.Header().Add("Cache-Control", "no-store, must-revalidate")
		w.Header().Add("Expires", "0")
		fs.ServeHTTP(w, r)
	}
}

func main() {
	port := "8080"

	if env, exists := os.LookupEnv("PORT"); exists {
		port = env
	}

	directory := flag.String("d", ".", "the directory of static file to host")
	flag.Parse()

	fileServer := http.FileServer(FileSystem{http.Dir(*directory)})
	http.Handle("/", http.StripPrefix(strings.TrimRight("/", "/"), addHeaders(fileServer)))

	log.Printf("Serving %s on HTTP port: %s\n", *directory, port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
