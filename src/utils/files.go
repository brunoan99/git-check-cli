package utils

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
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
			if !ok {
				return "", &CannotResolvePathError{
					Message: fmt.Sprintf("cannot find %s as env var", pathPart),
				}
			}
			pathOutput = append(pathOutput, value)
		} else {
			pathOutput = append(pathOutput, pathPart)
		}
	}

	resolvedPath := strings.Join(pathOutput, "/")

	absolutePath, err := filepath.Abs(resolvedPath)
	if err != nil {
		return "", &CannotResolvePathError{
			Message: fmt.Sprint("cannot resolve path to absolute cause of error: ", err),
		}
	}

	return absolutePath, nil
}

type MissingFileError struct {
	Path    string
	Message string
}

func (e *MissingFileError) Error() string {
	return e.Message
}

func LoadFile(path string) ([]byte, error) {
	file, err := os.ReadFile(path)
	if err != nil {
		return []byte{}, &MissingFileError{
			Path:    path,
			Message: fmt.Sprintf("cannot find file at %s", path),
		}
	}

	return file, nil
}

func CheckDirExists(path string) (bool, error) {
	info, err := os.Stat(path)
	if err == nil {
		return info.IsDir(), nil
	}
	if os.IsNotExist(err) {
		return false, nil
	}
	return false, err
}
