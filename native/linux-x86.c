long syscall0(long n)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n) : "memory");
  return __ret;
}

long syscall1(long n, long a1)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n), "b"(a1) : "memory");
  return __ret;
}

long syscall2(long n, long a1, long a2)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n), "b"(a1), "c"(a2) : "memory");
  return __ret;
}

long syscall3(long n, long a1, long a2, long a3)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n), "b"(a1), "c"(a2), "d"(a3) : "memory");
  return __ret;
}

long syscall4(long n, long a1, long a2, long a3, long a4)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4) : "memory");
  return __ret;
}

long syscall5(long n, long a1, long a2, long a3, long a4, long a5)
{
  unsigned long __ret;
  __asm__ __volatile__ ("int $128" : "=a"(__ret) : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4), "D"(a5) : "memory");
  return __ret;
}

long syscall6(long n, long a1, long a2, long a3, long a4, long a5, long a6)
{
  unsigned long __ret;
  __asm__ __volatile__ ("pushl %7 ; push %%ebp ; mov 4(%%esp),%%ebp ; int $128 ; pop %%ebp ; add $4,%%esp" : "=a"(__ret) : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4), "D"(a5), "g"(a6) : "memory");
  return __ret;
}
