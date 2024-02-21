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
	// 클라이언트한테 받은 데이터를 저장할 배열
	var from[ 100 ]float32

	// 추론 결과가 저장될 배열
	var to[ 2 ]float32

	// 클라이언트한테 데이터 받아서 from에 저장
	// ...

	// 추론 결과를 to 배열에 저장
	C.inference(
		(*C.float)(unsafe.Pointer(&from[ 0 ])),
		(*C.float)(unsafe.Pointer(&to[ 0 ])))

	// 추론 결과 출력
	fmt.Println(to[ 0 ], to[ 1 ])
}
