use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;

static SOCKET_TABLE: Mutex<SocketTable> = Mutex::new(SocketTable::new());
static EPOLL_TABLE: Mutex<BTreeMap<i32, EpollInstance>> = Mutex::new(BTreeMap::new());

struct SocketTable {
    sockets: Vec<Socket>,
    next_fd: i32,
}

impl SocketTable {
    const fn new() -> Self {
        Self {
            sockets: Vec::new(),
            next_fd: 1024,
        }
    }
}

struct Socket {
    fd: i32,
    domain: i32,
    sock_type: i32,
    state: SocketState,
    local_addr: [u8; 128],
    remote_addr: [u8; 128],
    buffer: Vec<u8>,
}

enum SocketState {
    Created,
    Bound,
    Listening,
    Connected,
    Closed,
}

struct EpollInstance {
    events: Vec<EpollEvent>,
}

struct EpollEvent {
    fd: i32,
    events: u32,
}

pub fn create_socket(domain: i32, sock_type: i32, protocol: i32) -> isize {
    let mut table = SOCKET_TABLE.lock();
    let fd = table.next_fd;
    table.next_fd += 1;
    
    table.sockets.push(Socket {
        fd,
        domain,
        sock_type,
        state: SocketState::Created,
        local_addr: [0; 128],
        remote_addr: [0; 128],
        buffer: Vec::new(),
    });
    
    fd as isize
}

pub fn bind_socket(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    let mut table = SOCKET_TABLE.lock();
    
    if let Some(sock) = table.sockets.iter_mut().find(|s| s.fd == sockfd) {
        let copy_len = addrlen.min(128);
        unsafe {
            core::ptr::copy_nonoverlapping(
                addr as *const u8,
                sock.local_addr.as_mut_ptr(),
                copy_len
            );
        }
        sock.state = SocketState::Bound;
        return 0;
    }
    
    -9
}

pub fn listen_socket(sockfd: i32, backlog: i32) -> isize {
    let mut table = SOCKET_TABLE.lock();
    
    if let Some(sock) = table.sockets.iter_mut().find(|s| s.fd == sockfd) {
        sock.state = SocketState::Listening;
        return 0;
    }
    
    -9
}

pub fn connect_socket(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    let mut table = SOCKET_TABLE.lock();
    
    if let Some(sock) = table.sockets.iter_mut().find(|s| s.fd == sockfd) {
        let copy_len = addrlen.min(128);
        unsafe {
            core::ptr::copy_nonoverlapping(
                addr as *const u8,
                sock.remote_addr.as_mut_ptr(),
                copy_len
            );
        }
        sock.state = SocketState::Connected;
        return 0;
    }
    
    -9
}

pub fn accept_socket(sockfd: i32, addr: usize, addrlen: usize) -> isize {
    let mut table = SOCKET_TABLE.lock();
    
    if table.sockets.iter().any(|s| s.fd == sockfd && matches!(s.state, SocketState::Listening)) {
        let new_fd = table.next_fd;
        table.next_fd += 1;
        
        table.sockets.push(Socket {
            fd: new_fd,
            domain: 0,
            sock_type: 0,
            state: SocketState::Connected,
            local_addr: [0; 128],
            remote_addr: [0; 128],
            buffer: Vec::new(),
        });
        
        return new_fd as isize;
    }
    
    -9
}

pub fn send_to(sockfd: i32, buf: usize, len: usize, flags: i32, dest_addr: usize, addrlen: usize) -> isize {
    let table = SOCKET_TABLE.lock();
    
    if table.sockets.iter().any(|s| s.fd == sockfd) {
        return len as isize;
    }
    
    -9
}

pub fn recv_from(sockfd: i32, buf: usize, len: usize, flags: i32, src_addr: usize, addrlen: usize) -> isize {
    let mut table = SOCKET_TABLE.lock();
    
    if let Some(sock) = table.sockets.iter_mut().find(|s| s.fd == sockfd) {
        if sock.buffer.is_empty() {
            return if (flags & 0x40) != 0 { -11 } else { 0 };
        }
        
        let copy_len = len.min(sock.buffer.len());
        unsafe {
            core::ptr::copy_nonoverlapping(
                sock.buffer.as_ptr(),
                buf as *mut u8,
                copy_len
            );
        }
        
        sock.buffer.drain(..copy_len);
        return copy_len as isize;
    }
    
    -9
}

pub fn epoll_wait(epfd: i32, events: usize, maxevents: i32, timeout: i32) -> isize {
    let table = EPOLL_TABLE.lock();
    
    if let Some(_instance) = table.get(&epfd) {
        if timeout > 0 {
            for _ in 0..timeout {
                core::hint::spin_loop();
            }
        }
        return 0;
    }
    
    -9
}
