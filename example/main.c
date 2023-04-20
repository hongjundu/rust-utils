#include "rustutils.h"
#include <stdio.h>
#include <stdlib.h>

//gcc main.c -L. -lrustutils -o main

int main() {
    message();

    char result_buf[10240];
    int32_t ret = cmd_system("git help", result_buf, sizeof(result_buf));
    if (ret == 0) {
        printf("Command executed successfully. Result:\n%s", result_buf);
    } else {
        printf("Failed to execute command.\n");
    }
    return 0;
}
