# Simple Dynamic Strings

<table align=center><td>

```rs
#[no_mangle]
pub extern "C" fn my_data() -> c_sds {
    SdString::from("He\0llo Alan\0 Turing!").into_raw()
}

#[no_mangle]
pub extern "C" fn my_free(data: *const std::ffi::c_char) {
    let sds = unsafe { sds::Sds::from_raw(data) };
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
