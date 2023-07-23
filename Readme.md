# Rust bindings directly from the Linux sources

Constants only, since there is to my knowledge only one way to call functions directly from
the kernel, and that's through the `vDSO`, only constants needed to communicate through
the syscall API are generated.

## Get sources

Get from [git](https://github.com/torvalds/linux), [kernel archives](https://www.kernel.org/), or wherever.

## Make nolibc from arches

`[<Linux source root>/tools/include/nolibc] make ARCH=<arch-linux> OUTPUT=<here>/include-kernel-headers/<arch>/ headers_standalone`.  
`<Linux source root>` is where the linux sources are installed.  
`<arch-linux>` is the Linux naming for the architecture, `aarch64` is `arm64` here.  
`<here>` is this repo's root directory
`<arch>` is the rust notation of the architecture, ie 64-bit arm is `aarch64`.

Will generate the kernel headers, mind that they are licensed under `LGPL` or `MIT`.  
