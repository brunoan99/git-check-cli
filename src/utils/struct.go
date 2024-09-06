package utils

import "reflect"

func IsStructEmpty(s interface{}) bool {
	// Check if the input is a struct
	if reflect.ValueOf(s).Kind() == reflect.Struct {
		// Get the value of the struct
		v := reflect.ValueOf(s)

		// Check if all fields are their zero values
		for i := 0; i < v.NumField(); i++ {
			if v.Field(i).Interface() != reflect.Zero(v.Field(i).Type()).Interface() {
				return false
			}
		}
		return true
	}
	return false
}
