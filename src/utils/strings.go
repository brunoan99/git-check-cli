package utils

import "strings"

func GetContentUntil(s, until string) string {
	index := strings.Index(s, until)
	if index == -1 {
		return s
	}
	return s[:index]
}

func GetContentInBetween(s, start, end string) string {
	startIndex := strings.Index(s, start)
	if startIndex == -1 {
		return s
	}
	startIndex += len(start)

	endIndex := strings.Index(s[startIndex:], end)
	if endIndex == -1 {
		return s
	}
	endIndex += startIndex

	return s[startIndex:endIndex]
}

func GetContentAfter(s, start string) string {
	startIndex := strings.Index(s, start)
	if startIndex == -1 {
		return s
	}

	// Move startIndex to the end of the start string
	startIndex += len(start)

	return s[startIndex:]
}
