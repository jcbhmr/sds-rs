# Simple Dynamic Strings for Rust

🦀 Rust FFI interop for the SDS C library

<table align=center><td>

```rs
#[no_mangle]
pub extern "C" fn count_nuls(s: sds::c_sds) -> c_int {
    let s = unsafe { sds::SdStr::from_ptr(s) };
    s.iter().filter(|c| *c == b'\0').count() as c_int
}
```

```c
sds message = sdsnewlen("He\0llo Al\0an Tur\0ing!", 22);
int nuls = count_nuls(message);
printf("There are %d NUL bytes in '%.*s'\n", nuls, sdslen(message), message);
sdsfree(message);
```

<tr><td>

```
There are 3 NUL bytes in 'Hello Alan Turing!'
```

</table>

- **[sds](crates/sds):** TODO

- **[sds-sys](crates/sds-sys):** TODO

## Installation

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)

You're reading the root monorepo readme. 😉 Click the links above 👆 to check out subproject-specific installation instructions.

If you're just looking for the main SDS Rust crate:

```sh
cargo add sds
```

## Usage

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![C](https://img.shields.io/static/v1?style=for-the-badge&message=C&color=222222&logo=C&logoColor=A8B9CC&label=)

TODO

<!-- ```rs
// Takes ownership of `s`. Mutates it. Returns an owned SDS.
#[no_mangle]
pub extern "C" fn to_spongebob_case(s: sds::c_sds) -> sds::c_sds {
    let s = unsafe { sds::Sds::from_raw(s) };
    for (i, c) in s.iter().enumerate() {
        if i % 2 == 0 {
            s[i] = c.to_ascii_uppercase();
        } else {
            s[i] = c.to_ascii_lowercase();
        }
    }
    s.into_raw()
}
```

```c
#include <stdio.h>
#include <sds.h>
#include <bindings.h>

int main() {
    sds message = sdsnew("Hello Alan Turing!");
    message = to_spongebob_case(message);
    puts(message);
    sdsfree(message);
    // Output: "HeLlO AlAn tUrInG!"
}
``` -->
