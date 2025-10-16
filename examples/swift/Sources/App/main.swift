import CWeaveFFI

@inline(__always)
func run() {
    var err = weaveffi_error(code: 0, message: nil)
    let sum = weaveffi_calculator_add(3, 4, &err)
    if err.code != 0 { let msg = err.message.flatMap { String(cString: $0) } ?? ""; weaveffi_error_clear(&err); fatalError(msg) }
    print("add(3,4) =", sum)

    let prod = weaveffi_calculator_mul(5, 6, &err)
    if err.code != 0 { let msg = err.message.flatMap { String(cString: $0) } ?? ""; weaveffi_error_clear(&err); fatalError(msg) }
    print("mul(5,6) =", prod)

    let q = weaveffi_calculator_div(10, 2, &err)
    if err.code != 0 { let msg = err.message.flatMap { String(cString: $0) } ?? ""; weaveffi_error_clear(&err); fatalError(msg) }
    print("div(10,2) =", q)

    let sBytes = Array("hello".utf8)
    sBytes.withUnsafeBufferPointer { buf in
        let sPtr = buf.baseAddress
        let sLen = sBytes.count
        let rv = weaveffi_calculator_echo(sPtr, sLen, &err)
        if err.code != 0 { let msg = err.message.flatMap { String(cString: $0) } ?? ""; weaveffi_error_clear(&err); fatalError(msg) }
        defer { weaveffi_free_string(rv) }
        if let rv = rv {
            print("echo(hello) =", String(cString: rv))
        }
    }

    _ = weaveffi_calculator_div(1, 0, &err)
    if err.code != 0 { let msg = err.message.flatMap { String(cString: $0) } ?? ""; print("div(1,0) error:", msg); weaveffi_error_clear(&err) }
}

run()
