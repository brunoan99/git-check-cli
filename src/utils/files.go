package utils

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

var ErrMissingPath = errors.New("path not found")

type CannotResolvePathError struct {
	Message string
}

func (e *CannotResolvePathError) Error() string {
	return e.Message
}

func ResolvePath(path string) (string, error) {

	pathSplited := strings.Split(path, "/")
	pathOutput := []string{}

	for i := 0; i < len(pathSplited); i++ {
		pathPart := pathSplited[i]
		toSearch, found := strings.CutPrefix(pathPart, "$")
		if found {
			value, ok := os.LookupEnv(toSearch)
			if ok {
				pathOutput = append(pathOutput, value)
			} else {
				return "", &CannotResolvePathError{
					Message: fmt.Sprintf("cannot find %s as env var", pathPart),
				}
			}
		} else {
			pathOutput = append(pathOutput, pathPart)
		}
	}

	resolvedPath := strings.Join(pathOutput, "/")

	return resolvedPath, nil
}

type MissingFileError struct {
	Path    string
	Message string
}

func (e *MissingFileError) Error() string {
	return e.Message
}

func LoadFile(path string) ([]byte, error) {
	resolvedPath, err := ResolvePath(path)
	if err != nil {
		return []byte{}, err
	}

	file, err := os.ReadFile(resolvedPath)
	if err != nil {
		return []byte{}, &MissingFileError{
			Path:    resolvedPath,
			Message: fmt.Sprintf("cannot find file at %s", resolvedPath),
		}
	}

	return file, nil
}
