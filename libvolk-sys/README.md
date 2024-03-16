generated with: ```$ bindgen "$VOLK_PATH/include/volk/volk.h" --allowlist-function '^volk_.*' --allowlist-type '^(volk_|p_).*' --allowlist-var '^(VOLK_|volk_).*' --blocklist-function '^volk_.*(_func_desc|_manual)' --blocklist-type '^volk.*_func_desc(_t)?' --raw-line $'#![allow(non_camel_case_types)]\n#![allow(non_snake_case)
]' -o src/volk.rs -- -x c++
$ sed -i '/^extern "C" {/i\#[link(name="volk")]' src/volk.rs
$ sed -i -e 's/_M_real/real/' -e 's/_M_imag/imag/' -e 's/volk_log2to10factor/VOLK_LOG2TO10FACTOR/' src/volk.rs
```
