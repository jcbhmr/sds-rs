# Simple Dynamic Strings Rust crate

<table align=center><td>

```rs
#[no_mangle]
pub extern "C" fn my_data() -> c_sds {
    SdsOwned::from("He\0llo Alan\0 Turing!").into_raw()
}

#[no_mangle]
pub extern "C" fn my_sdsfree(data: c_sds) {
    let sds = unsafe { sds::SdsOwned::from_raw(data) };
    drop(sds);
}
```

<tr><td>

```c
#include <stdio.h>
#include <sds.h>
#include <my_bindings.h>

int main() {
    sds data = my_data();
    printf("printf(%%s): %s\n", data);
    printf("printf(%%.*s): %.*s\n", sdslen(data), data);
    sdsfree(data);
}
```

<tr><td>

```
printf(%s): He
printf(%.*s): Hello Alan Turing!
```

</table>
