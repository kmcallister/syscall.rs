long syscall0(long n) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0");
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7) : "memory");
    return r0;
}

long syscall1(long n, long a1) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0) : "memory");
    return r0;
}

long syscall2(long n, long a1, long a2) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1) : "memory");
    return r0;
}

long syscall3(long n, long a1, long a2, long a3) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    register long r2 __asm__("r2") = a3;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2) : "memory");
    return r0;
}

long syscall4(long n, long a1, long a2, long a3, long a4) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    register long r2 __asm__("r2") = a3;
    register long r3 __asm__("r3") = a4;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2), "r"(r3) : "memory");
    return r0;
}

long syscall5(long n, long a1, long a2, long a3, long a4, long a5) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    register long r2 __asm__("r2") = a3;
    register long r3 __asm__("r3") = a4;
    register long r4 __asm__("r4") = a5;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4) : "memory");
    return r0;
}

long syscall6(long n, long a1, long a2, long a3, long a4, long a5, long a6) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    register long r2 __asm__("r2") = a3;
    register long r3 __asm__("r3") = a4;
    register long r4 __asm__("r4") = a5;
    register long r5 __asm__("r5") = a6;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5) : "memory");
    return r0;
}

long syscall7(long n, long a1, long a2, long a3, long a4, long a5, long a6, long a7) {
    register long r7 __asm__("r7") = n;
    register long r0 __asm__("r0") = a1;
    register long r1 __asm__("r1") = a2;
    register long r2 __asm__("r2") = a3;
    register long r3 __asm__("r3") = a4;
    register long r4 __asm__("r4") = a5;
    register long r5 __asm__("r5") = a6;
    register long r6 __asm__("r6") = a7;
    __asm__ __volatile__ ("swi $0" : "=r"(r0) : "r"(r7), "r"(r0), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5), "r"(r6) : "memory");
    return r0;
}
