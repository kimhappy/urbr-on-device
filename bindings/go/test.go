package main

/*
#cgo LDFLAGS: -L./target/release -lurbr_go
#include <stdlib.h>
void inference(float const* from, float* to);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	var from [ 100 ]float32
	var to   [   2 ]float32

	from[ 0 ] = 100
	from[ 1 ] = 200

	C.inference((*C.float)(unsafe.Pointer(&from[ 0 ])), (*C.float)(unsafe.Pointer(&to[ 0 ])))

	fmt.Println(to[ 0 ], to[ 1 ])
}
