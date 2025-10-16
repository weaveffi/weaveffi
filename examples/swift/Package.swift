// swift-tools-version:5.7
import PackageDescription

let package = Package(
    name: "CalculatorExample",
    platforms: [ .macOS(.v12) ],
    dependencies: [
        .package(name: "WeaveFFI", path: "../../generated/swift")
    ],
    targets: [
        .executableTarget(
            name: "App",
            dependencies: [ .product(name: "WeaveFFI", package: "WeaveFFI") ]
        )
    ]
)
