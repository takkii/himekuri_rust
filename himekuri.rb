require "ffi"

module Hello
  extend FFI::Library
  ffi_lib "./lib/libhimekuri_rust.dylib"
  attach_function :himekuri, [], :void
end

Hello.himekuri
