#include <zephyr/kernel.h>
#include <rust_lib_1/rust_lib_1.h>
#include <rust_lib_2/rust_lib_2.h>

void main(void)
{
    printk("rust_lib_1_add(7, 19) returned %d\n", rust_lib_1_add(7, 19));
    printk("rust_lib_2_mul(7, 19) returned %d\n", rust_lib_2_mul(7, 19));
}
