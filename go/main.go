package main

import (
	"fmt"
)

func main() {
	PI := 3.14159
	var radius float64
	fmt.Println("Enter the radius of the sphere: ")
	_, err := fmt.Scanf("%f", &radius)
	if err != nil {
		fmt.Println(err.Error())
	}
	volume := PI * radius * radius * radius / 3
	fmt.Printf("The volume of the sphere with the radius of %f inches is %f cubic inches\n", radius, volume)
}
