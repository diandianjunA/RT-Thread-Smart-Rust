#include <stdio.h>
#include <unistd.h>
#include <sys/time.h>

int main(int argc, char **argv) {
    struct timeval tv_begin, tv_end;
    gettimeofday(&tv_begin,NULL);
    for (int i = 0; i < 10; i++) {
	    printf("C program: Hello world! %d\n", i);
    }
    gettimeofday(&tv_end,NULL);
    double milisecs = (tv_end.tv_sec - tv_begin.tv_sec) * 1000.0 + (tv_end.tv_usec - tv_begin.tv_usec) / 1000.0;
    printf("Time: %fms\n", milisecs);
    return 0;
}

