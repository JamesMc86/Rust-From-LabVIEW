# Rust-From-LabVIEW
PoC of calling rust code from LabVIEW using the C FFI

The LabVIEW code is LabVIEW 2019 64 bit.

Note: This is very non-functional - I'm happy it does crash. I put it here for future reference.

I may flesh out more concepts over time.

## Key Concepts

### Arrays

Array ownership should remain with LabVIEW if LabVIEW created them. I saw this as when I created vectors in Rust LabVIEW crashed - probably because Rust tried to drop them.

Instead treat it as a slice. That means ownership stays with LabVIEW. If you need ownership in Rust then the simplest thing would be to make a copy as I do in this code. Obviously there are performance implications to that.

### Structs

In this case I'm not trying to share struct data. In theory it may be possible but my structures were two complicated.

I think the API style method used here is probably easier. but for simple structs the methods used in https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/ should work.

## Reading List

This was largely based off https://medium.com/jim-fleming/complex-types-with-rust-s-ffi-315d14619479 and https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/
