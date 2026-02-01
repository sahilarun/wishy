#define _GNU_SOURCE
#include <stdint.h>
#include <stddef.h>

extern long linux_syscall(long num, long arg1, long arg2, long arg3, long arg4, long arg5, long arg6);

long syscall(long num, ...) {
    long args[6] = {0};
    
    __builtin_va_list ap;
    __builtin_va_start(ap, num);
    
    for (int i = 0; i < 6; i++) {
        args[i] = __builtin_va_arg(ap, long);
    }
    
    __builtin_va_end(ap);
    
    return linux_syscall(num, args[0], args[1], args[2], args[3], args[4], args[5]);
}

int open(const char *pathname, int flags, ...) {
    mode_t mode = 0;
    
    if (flags & 0x40) {
        __builtin_va_list ap;
        __builtin_va_start(ap, flags);
        mode = __builtin_va_arg(ap, mode_t);
        __builtin_va_end(ap);
    }
    
    return (int)syscall(2, (long)pathname, (long)flags, (long)mode);
}

ssize_t read(int fd, void *buf, size_t count) {
    return syscall(0, (long)fd, (long)buf, (long)count);
}

ssize_t write(int fd, const void *buf, size_t count) {
    return syscall(1, (long)fd, (long)buf, (long)count);
}

int close(int fd) {
    return (int)syscall(3, (long)fd);
}

void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset) {
    return (void *)syscall(9, (long)addr, (long)length, (long)prot, (long)flags, (long)fd, (long)offset);
}

int munmap(void *addr, size_t length) {
    return (int)syscall(11, (long)addr, (long)length);
}

pid_t fork(void) {
    return (pid_t)syscall(57);
}

pid_t getpid(void) {
    return (pid_t)syscall(39);
}

pid_t getppid(void) {
    return (pid_t)syscall(110);
}

int socket(int domain, int type, int protocol) {
    return (int)syscall(41, (long)domain, (long)type, (long)protocol);
}

int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    return (int)syscall(42, (long)sockfd, (long)addr, (long)addrlen);
}

int ioctl(int fd, unsigned long request, ...) {
    __builtin_va_list ap;
    __builtin_va_start(ap, request);
    void *arg = __builtin_va_arg(ap, void *);
    __builtin_va_end(ap);
    
    extern int drm_ioctl_handler(int fd, unsigned long request, void *arg);
    return drm_ioctl_handler(fd, request, arg);
}

void *dlsym(void *handle, const char *symbol) {
    if (__builtin_strcmp(symbol, "eglGetDisplay") == 0) {
        extern void *egl_get_display(void *);
        return egl_get_display;
    }
    if (__builtin_strcmp(symbol, "eglInitialize") == 0) {
        extern int egl_initialize(void *, int *, int *);
        return egl_initialize;
    }
    if (__builtin_strcmp(symbol, "eglSwapBuffers") == 0) {
        extern int egl_swap_buffers(void *, void *);
        return egl_swap_buffers;
    }
    
    return NULL;
}

void *dlopen(const char *filename, int flag) {
    return (void *)1;
}

int dlclose(void *handle) {
    return 0;
}
