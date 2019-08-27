
pub const ENOSYS: crate::c_int = 38;

pub const EXIT_FAILURE: crate::c_int = 1;
pub const EXIT_SUCCESS: crate::c_int = 0;
pub const RAND_MAX: crate::c_int = 2147483647;
pub const EOF: crate::c_int = -1;
pub const SEEK_SET: crate::c_int = 0;
pub const SEEK_CUR: crate::c_int = 1;
pub const SEEK_END: crate::c_int = 2;
pub const _IOFBF: crate::c_int = 0;
pub const _IONBF: crate::c_int = 2;
pub const _IOLBF: crate::c_int = 1;

pub const F_DUPFD: crate::c_int = 0;
pub const F_GETFD: crate::c_int = 1;
pub const F_SETFD: crate::c_int = 2;
pub const F_GETFL: crate::c_int = 3;
pub const F_SETFL: crate::c_int = 4;

// Linux-specific fcntls
pub const F_SETLEASE: crate::c_int = 1024;
pub const F_GETLEASE: crate::c_int = 1025;
pub const F_NOTIFY: crate::c_int = 1026;
pub const F_CANCELLK: crate::c_int = 1029;
pub const F_DUPFD_CLOEXEC: crate::c_int = 1030;
pub const F_SETPIPE_SZ: crate::c_int = 1031;
pub const F_GETPIPE_SZ: crate::c_int = 1032;
pub const F_ADD_SEALS: crate::c_int = 1033;
pub const F_GET_SEALS: crate::c_int = 1034;

pub const F_SEAL_SEAL: crate::c_int = 0x0001;
pub const F_SEAL_SHRINK: crate::c_int = 0x0002;
pub const F_SEAL_GROW: crate::c_int = 0x0004;
pub const F_SEAL_WRITE: crate::c_int = 0x0008;

// TODO(#235): Include file sealing fcntls once we have a way to verify them.

pub const SIGTRAP: crate::c_int = 5;

pub const PTHREAD_CREATE_JOINABLE: crate::c_int = 0;
pub const PTHREAD_CREATE_DETACHED: crate::c_int = 1;

// pub const CLOCK_REALTIME: crate::clockid_t = 0;
// pub const CLOCK_MONOTONIC: crate::clockid_t = 1;
// pub const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2;
// pub const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3;
// pub const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4;
// pub const CLOCK_REALTIME_COARSE: crate::clockid_t = 5;
// pub const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6;
// pub const CLOCK_BOOTTIME: crate::clockid_t = 7;
// pub const CLOCK_REALTIME_ALARM: crate::clockid_t = 8;
// pub const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9;
// // TODO(#247) Someday our Travis shall have glibc 2.21 (released in Sep
// // 2014.) See also musl/mod.rs
// // pub const CLOCK_SGI_CYCLE: crate::clockid_t = 10;
// // pub const CLOCK_TAI: crate::clockid_t = 11;
// pub const TIMER_ABSTIME: crate::c_int = 1;

pub const RUSAGE_SELF: crate::c_int = 0;

pub const O_RDONLY: crate::c_int = 0;
pub const O_WRONLY: crate::c_int = 1;
pub const O_RDWR: crate::c_int = 2;

// pub const SOCK_CLOEXEC: crate::c_int = O_CLOEXEC;

pub const S_IFIFO: crate::mode_t = 4096;
pub const S_IFCHR: crate::mode_t = 8192;
pub const S_IFBLK: crate::mode_t = 24576;
pub const S_IFDIR: crate::mode_t = 16384;
pub const S_IFREG: crate::mode_t = 32768;
pub const S_IFLNK: crate::mode_t = 40960;
pub const S_IFSOCK: crate::mode_t = 49152;
// pub const S_IFMT: crate::mode_t = 61440;
// pub const S_IRWXU: crate::mode_t = 448;
// pub const S_IXUSR: crate::mode_t = 64;
// pub const S_IWUSR: crate::mode_t = 128;
// pub const S_IRUSR: crate::mode_t = 256;
// pub const S_IRWXG: crate::mode_t = 56;
// pub const S_IXGRP: crate::mode_t = 8;
// pub const S_IWGRP: crate::mode_t = 16;
// pub const S_IRGRP: crate::mode_t = 32;
// pub const S_IRWXO: crate::mode_t = 7;
// pub const S_IXOTH: crate::mode_t = 1;
// pub const S_IWOTH: crate::mode_t = 2;
// pub const S_IROTH: crate::mode_t = 4;
// pub const F_OK: crate::c_int = 0;
// pub const R_OK: crate::c_int = 4;
// pub const W_OK: crate::c_int = 2;
// pub const X_OK: crate::c_int = 1;
// pub const STDIN_FILENO: crate::c_int = 0;
// pub const STDOUT_FILENO: crate::c_int = 1;
// pub const STDERR_FILENO: crate::c_int = 2;
// pub const SIGHUP: crate::c_int = 1;
// pub const SIGINT: crate::c_int = 2;
// pub const SIGQUIT: crate::c_int = 3;
// pub const SIGILL: crate::c_int = 4;
// pub const SIGABRT: crate::c_int = 6;
// pub const SIGFPE: crate::c_int = 8;
// pub const SIGKILL: crate::c_int = 9;
// pub const SIGSEGV: crate::c_int = 11;
// pub const SIGPIPE: crate::c_int = 13;
// pub const SIGALRM: crate::c_int = 14;
// pub const SIGTERM: crate::c_int = 15;

// pub const PROT_NONE: crate::c_int = 0;
// pub const PROT_READ: crate::c_int = 1;
// pub const PROT_WRITE: crate::c_int = 2;
// pub const PROT_EXEC: crate::c_int = 4;

// pub const LC_CTYPE: crate::c_int = 0;
// pub const LC_NUMERIC: crate::c_int = 1;
// pub const LC_TIME: crate::c_int = 2;
// pub const LC_COLLATE: crate::c_int = 3;
// pub const LC_MONETARY: crate::c_int = 4;
// pub const LC_MESSAGES: crate::c_int = 5;
// pub const LC_ALL: crate::c_int = 6;
// pub const LC_CTYPE_MASK: crate::c_int = (1 << LC_CTYPE);
// pub const LC_NUMERIC_MASK: crate::c_int = (1 << LC_NUMERIC);
// pub const LC_TIME_MASK: crate::c_int = (1 << LC_TIME);
// pub const LC_COLLATE_MASK: crate::c_int = (1 << LC_COLLATE);
// pub const LC_MONETARY_MASK: crate::c_int = (1 << LC_MONETARY);
// pub const LC_MESSAGES_MASK: crate::c_int = (1 << LC_MESSAGES);
// // LC_ALL_MASK defined per platform

// pub const MAP_FILE: crate::c_int = 0x0000;
// pub const MAP_SHARED: crate::c_int = 0x0001;
// pub const MAP_PRIVATE: crate::c_int = 0x0002;
// pub const MAP_FIXED: crate::c_int = 0x0010;

// pub const MAP_FAILED: *mut crate::c_void = !0 as *mut crate::c_void;

// // MS_ flags for msync(2)
// pub const MS_ASYNC: crate::c_int = 0x0001;
// pub const MS_INVALIDATE: crate::c_int = 0x0002;
// pub const MS_SYNC: crate::c_int = 0x0004;

// // MS_ flags for mount(2)
// pub const MS_RDONLY: crate::c_ulong = 0x01;
// pub const MS_NOSUID: crate::c_ulong = 0x02;
// pub const MS_NODEV: crate::c_ulong = 0x04;
// pub const MS_NOEXEC: crate::c_ulong = 0x08;
// pub const MS_SYNCHRONOUS: crate::c_ulong = 0x10;
// pub const MS_REMOUNT: crate::c_ulong = 0x20;
// pub const MS_MANDLOCK: crate::c_ulong = 0x40;
// pub const MS_DIRSYNC: crate::c_ulong = 0x80;
// pub const MS_NOATIME: crate::c_ulong = 0x0400;
// pub const MS_NODIRATIME: crate::c_ulong = 0x0800;
// pub const MS_BIND: crate::c_ulong = 0x1000;
// pub const MS_MOVE: crate::c_ulong = 0x2000;
// pub const MS_REC: crate::c_ulong = 0x4000;
// pub const MS_SILENT: crate::c_ulong = 0x8000;
// pub const MS_POSIXACL: crate::c_ulong = 0x010000;
// pub const MS_UNBINDABLE: crate::c_ulong = 0x020000;
// pub const MS_PRIVATE: crate::c_ulong = 0x040000;
// pub const MS_SLAVE: crate::c_ulong = 0x080000;
// pub const MS_SHARED: crate::c_ulong = 0x100000;
// pub const MS_RELATIME: crate::c_ulong = 0x200000;
// pub const MS_KERNMOUNT: crate::c_ulong = 0x400000;
// pub const MS_I_VERSION: crate::c_ulong = 0x800000;
// pub const MS_STRICTATIME: crate::c_ulong = 0x1000000;
// pub const MS_ACTIVE: crate::c_ulong = 0x40000000;
// pub const MS_MGC_VAL: crate::c_ulong = 0xc0ed0000;
// pub const MS_MGC_MSK: crate::c_ulong = 0xffff0000;

// pub const EPERM: crate::c_int = 1;
pub const ENOENT: crate::c_int = 2;
// pub const ESRCH: crate::c_int = 3;
pub const EINTR: crate::c_int = 4;
pub const EIO: crate::c_int = 5;
// pub const ENXIO: crate::c_int = 6;
// pub const E2BIG: crate::c_int = 7;
// pub const ENOEXEC: crate::c_int = 8;
// pub const EBADF: crate::c_int = 9;
// pub const ECHILD: crate::c_int = 10;
pub const EAGAIN: crate::c_int = 11;
// pub const ENOMEM: crate::c_int = 12;
// pub const EACCES: crate::c_int = 13;
// pub const EFAULT: crate::c_int = 14;
// pub const ENOTBLK: crate::c_int = 15;
// pub const EBUSY: crate::c_int = 16;
// pub const EEXIST: crate::c_int = 17;
// pub const EXDEV: crate::c_int = 18;
pub const ENODEV: crate::c_int = 19;
// pub const ENOTDIR: crate::c_int = 20;
// pub const EISDIR: crate::c_int = 21;
// pub const EINVAL: crate::c_int = 22;
// pub const ENFILE: crate::c_int = 23;
// pub const EMFILE: crate::c_int = 24;
// pub const ENOTTY: crate::c_int = 25;
// pub const ETXTBSY: crate::c_int = 26;
// pub const EFBIG: crate::c_int = 27;
// pub const ENOSPC: crate::c_int = 28;
// pub const ESPIPE: crate::c_int = 29;
// pub const EROFS: crate::c_int = 30;
// pub const EMLINK: crate::c_int = 31;
// pub const EPIPE: crate::c_int = 32;
// pub const EDOM: crate::c_int = 33;
// pub const ERANGE: crate::c_int = 34;
// pub const EWOULDBLOCK: crate::c_int = EAGAIN;
pub const EPROTO: crate::c_int = 71;

pub const ENODATA: crate::c_int = 61;

// pub const SCM_RIGHTS: crate::c_int = 0x01;
// pub const SCM_CREDENTIALS: crate::c_int = 0x02;

// pub const PROT_GROWSDOWN: crate::c_int = 0x1000000;
// pub const PROT_GROWSUP: crate::c_int = 0x2000000;

// pub const MAP_TYPE: crate::c_int = 0x000f;

// pub const MADV_NORMAL: crate::c_int = 0;
// pub const MADV_RANDOM: crate::c_int = 1;
// pub const MADV_SEQUENTIAL: crate::c_int = 2;
// pub const MADV_WILLNEED: crate::c_int = 3;
// pub const MADV_DONTNEED: crate::c_int = 4;
// pub const MADV_FREE: crate::c_int = 8;
// pub const MADV_REMOVE: crate::c_int = 9;
// pub const MADV_DONTFORK: crate::c_int = 10;
// pub const MADV_DOFORK: crate::c_int = 11;
// pub const MADV_MERGEABLE: crate::c_int = 12;
// pub const MADV_UNMERGEABLE: crate::c_int = 13;
// pub const MADV_HUGEPAGE: crate::c_int = 14;
// pub const MADV_NOHUGEPAGE: crate::c_int = 15;
// pub const MADV_DONTDUMP: crate::c_int = 16;
// pub const MADV_DODUMP: crate::c_int = 17;
// pub const MADV_HWPOISON: crate::c_int = 100;

// pub const IFF_UP: crate::c_int = 0x1;
// pub const IFF_BROADCAST: crate::c_int = 0x2;
// pub const IFF_DEBUG: crate::c_int = 0x4;
// pub const IFF_LOOPBACK: crate::c_int = 0x8;
// pub const IFF_POINTOPOINT: crate::c_int = 0x10;
// pub const IFF_NOTRAILERS: crate::c_int = 0x20;
// pub const IFF_RUNNING: crate::c_int = 0x40;
// pub const IFF_NOARP: crate::c_int = 0x80;
// pub const IFF_PROMISC: crate::c_int = 0x100;
// pub const IFF_ALLMULTI: crate::c_int = 0x200;
// pub const IFF_MASTER: crate::c_int = 0x400;
// pub const IFF_SLAVE: crate::c_int = 0x800;
// pub const IFF_MULTICAST: crate::c_int = 0x1000;
// pub const IFF_PORTSEL: crate::c_int = 0x2000;
// pub const IFF_AUTOMEDIA: crate::c_int = 0x4000;
// pub const IFF_DYNAMIC: crate::c_int = 0x8000;

// pub const SOL_IP: crate::c_int = 0;
// pub const SOL_TCP: crate::c_int = 6;
// pub const SOL_UDP: crate::c_int = 17;
// pub const SOL_IPV6: crate::c_int = 41;
// pub const SOL_ICMPV6: crate::c_int = 58;
// pub const SOL_RAW: crate::c_int = 255;
// pub const SOL_DECNET: crate::c_int = 261;
// pub const SOL_X25: crate::c_int = 262;
// pub const SOL_PACKET: crate::c_int = 263;
// pub const SOL_ATM: crate::c_int = 264;
// pub const SOL_AAL: crate::c_int = 265;
// pub const SOL_IRDA: crate::c_int = 266;
// pub const SOL_NETBEUI: crate::c_int = 267;
// pub const SOL_LLC: crate::c_int = 268;
// pub const SOL_DCCP: crate::c_int = 269;
// pub const SOL_NETLINK: crate::c_int = 270;
// pub const SOL_TIPC: crate::c_int = 271;
// pub const SOL_BLUETOOTH: crate::c_int = 274;
// pub const SOL_ALG: crate::c_int = 279;

// pub const AF_UNSPEC: crate::c_int = 0;
// pub const AF_UNIX: crate::c_int = 1;
// pub const AF_LOCAL: crate::c_int = 1;
// pub const AF_INET: crate::c_int = 2;
// pub const AF_AX25: crate::c_int = 3;
// pub const AF_IPX: crate::c_int = 4;
// pub const AF_APPLETALK: crate::c_int = 5;
// pub const AF_NETROM: crate::c_int = 6;
// pub const AF_BRIDGE: crate::c_int = 7;
// pub const AF_ATMPVC: crate::c_int = 8;
// pub const AF_X25: crate::c_int = 9;
// pub const AF_INET6: crate::c_int = 10;
// pub const AF_ROSE: crate::c_int = 11;
// pub const AF_DECnet: crate::c_int = 12;
// pub const AF_NETBEUI: crate::c_int = 13;
// pub const AF_SECURITY: crate::c_int = 14;
// pub const AF_KEY: crate::c_int = 15;
// pub const AF_NETLINK: crate::c_int = 16;
// pub const AF_ROUTE: crate::c_int = AF_NETLINK;
// pub const AF_PACKET: crate::c_int = 17;
// pub const AF_ASH: crate::c_int = 18;
// pub const AF_ECONET: crate::c_int = 19;
// pub const AF_ATMSVC: crate::c_int = 20;
// pub const AF_RDS: crate::c_int = 21;
// pub const AF_SNA: crate::c_int = 22;
// pub const AF_IRDA: crate::c_int = 23;
// pub const AF_PPPOX: crate::c_int = 24;
// pub const AF_WANPIPE: crate::c_int = 25;
// pub const AF_LLC: crate::c_int = 26;
// pub const AF_CAN: crate::c_int = 29;
// pub const AF_TIPC: crate::c_int = 30;
// pub const AF_BLUETOOTH: crate::c_int = 31;
// pub const AF_IUCV: crate::c_int = 32;
// pub const AF_RXRPC: crate::c_int = 33;
// pub const AF_ISDN: crate::c_int = 34;
// pub const AF_PHONET: crate::c_int = 35;
// pub const AF_IEEE802154: crate::c_int = 36;
// pub const AF_CAIF: crate::c_int = 37;
// pub const AF_ALG: crate::c_int = 38;

// pub const PF_UNSPEC: crate::c_int = AF_UNSPEC;
// pub const PF_UNIX: crate::c_int = AF_UNIX;
// pub const PF_LOCAL: crate::c_int = AF_LOCAL;
// pub const PF_INET: crate::c_int = AF_INET;
// pub const PF_AX25: crate::c_int = AF_AX25;
// pub const PF_IPX: crate::c_int = AF_IPX;
// pub const PF_APPLETALK: crate::c_int = AF_APPLETALK;
// pub const PF_NETROM: crate::c_int = AF_NETROM;
// pub const PF_BRIDGE: crate::c_int = AF_BRIDGE;
// pub const PF_ATMPVC: crate::c_int = AF_ATMPVC;
// pub const PF_X25: crate::c_int = AF_X25;
// pub const PF_INET6: crate::c_int = AF_INET6;
// pub const PF_ROSE: crate::c_int = AF_ROSE;
// pub const PF_DECnet: crate::c_int = AF_DECnet;
// pub const PF_NETBEUI: crate::c_int = AF_NETBEUI;
// pub const PF_SECURITY: crate::c_int = AF_SECURITY;
// pub const PF_KEY: crate::c_int = AF_KEY;
// pub const PF_NETLINK: crate::c_int = AF_NETLINK;
// pub const PF_ROUTE: crate::c_int = AF_ROUTE;
// pub const PF_PACKET: crate::c_int = AF_PACKET;
// pub const PF_ASH: crate::c_int = AF_ASH;
// pub const PF_ECONET: crate::c_int = AF_ECONET;
// pub const PF_ATMSVC: crate::c_int = AF_ATMSVC;
// pub const PF_RDS: crate::c_int = AF_RDS;
// pub const PF_SNA: crate::c_int = AF_SNA;
// pub const PF_IRDA: crate::c_int = AF_IRDA;
// pub const PF_PPPOX: crate::c_int = AF_PPPOX;
// pub const PF_WANPIPE: crate::c_int = AF_WANPIPE;
// pub const PF_LLC: crate::c_int = AF_LLC;
// pub const PF_CAN: crate::c_int = AF_CAN;
// pub const PF_TIPC: crate::c_int = AF_TIPC;
// pub const PF_BLUETOOTH: crate::c_int = AF_BLUETOOTH;
// pub const PF_IUCV: crate::c_int = AF_IUCV;
// pub const PF_RXRPC: crate::c_int = AF_RXRPC;
// pub const PF_ISDN: crate::c_int = AF_ISDN;
// pub const PF_PHONET: crate::c_int = AF_PHONET;
// pub const PF_IEEE802154: crate::c_int = AF_IEEE802154;
// pub const PF_CAIF: crate::c_int = AF_CAIF;
// pub const PF_ALG: crate::c_int = AF_ALG;

// pub const SOMAXCONN: crate::c_int = 128;

// pub const MSG_OOB: crate::c_int = 1;
// pub const MSG_PEEK: crate::c_int = 2;
// pub const MSG_DONTROUTE: crate::c_int = 4;
// pub const MSG_CTRUNC: crate::c_int = 8;
// pub const MSG_TRUNC: crate::c_int = 0x20;
// pub const MSG_DONTWAIT: crate::c_int = 0x40;
// pub const MSG_EOR: crate::c_int = 0x80;
// pub const MSG_WAITALL: crate::c_int = 0x100;
// pub const MSG_FIN: crate::c_int = 0x200;
// pub const MSG_SYN: crate::c_int = 0x400;
// pub const MSG_CONFIRM: crate::c_int = 0x800;
// pub const MSG_RST: crate::c_int = 0x1000;
// pub const MSG_ERRQUEUE: crate::c_int = 0x2000;
// pub const MSG_NOSIGNAL: crate::c_int = 0x4000;
// pub const MSG_MORE: crate::c_int = 0x8000;
// pub const MSG_WAITFORONE: crate::c_int = 0x10000;
// pub const MSG_FASTOPEN: crate::c_int = 0x20000000;
// pub const MSG_CMSG_CLOEXEC: crate::c_int = 0x40000000;

// pub const SCM_TIMESTAMP: crate::c_int = SO_TIMESTAMP;

// pub const SOCK_RAW: crate::c_int = 3;
// pub const SOCK_RDM: crate::c_int = 4;
// pub const IP_MULTICAST_IF: crate::c_int = 32;
// pub const IP_MULTICAST_TTL: crate::c_int = 33;
// pub const IP_MULTICAST_LOOP: crate::c_int = 34;
// pub const IP_TOS: crate::c_int = 1;
// pub const IP_TTL: crate::c_int = 2;
// pub const IP_HDRINCL: crate::c_int = 3;
// pub const IP_PKTINFO: crate::c_int = 8;
// pub const IP_RECVTOS: crate::c_int = 13;
// pub const IP_RECVERR: crate::c_int = 11;
// pub const IP_ADD_MEMBERSHIP: crate::c_int = 35;
// pub const IP_DROP_MEMBERSHIP: crate::c_int = 36;
// pub const IP_TRANSPARENT: crate::c_int = 19;
// pub const IPV6_UNICAST_HOPS: crate::c_int = 16;
// pub const IPV6_MULTICAST_IF: crate::c_int = 17;
// pub const IPV6_MULTICAST_HOPS: crate::c_int = 18;
// pub const IPV6_MULTICAST_LOOP: crate::c_int = 19;
// pub const IPV6_ADD_MEMBERSHIP: crate::c_int = 20;
// pub const IPV6_DROP_MEMBERSHIP: crate::c_int = 21;
// pub const IPV6_V6ONLY: crate::c_int = 26;
// pub const IPV6_RECVPKTINFO: crate::c_int = 49;
// pub const IPV6_PKTINFO: crate::c_int = 50;
// pub const IPV6_RECVTCLASS: crate::c_int = 66;
// pub const IPV6_TCLASS: crate::c_int = 67;

// pub const TCP_NODELAY: crate::c_int = 1;
// pub const TCP_MAXSEG: crate::c_int = 2;
// pub const TCP_CORK: crate::c_int = 3;
// pub const TCP_KEEPIDLE: crate::c_int = 4;
// pub const TCP_KEEPINTVL: crate::c_int = 5;
// pub const TCP_KEEPCNT: crate::c_int = 6;
// pub const TCP_SYNCNT: crate::c_int = 7;
// pub const TCP_LINGER2: crate::c_int = 8;
// pub const TCP_DEFER_ACCEPT: crate::c_int = 9;
// pub const TCP_WINDOW_CLAMP: crate::c_int = 10;
// pub const TCP_INFO: crate::c_int = 11;
// pub const TCP_QUICKACK: crate::c_int = 12;
// pub const TCP_CONGESTION: crate::c_int = 13;

// pub const SO_DEBUG: crate::c_int = 1;

// pub const SHUT_RD: crate::c_int = 0;
// pub const SHUT_WR: crate::c_int = 1;
// pub const SHUT_RDWR: crate::c_int = 2;

// pub const LOCK_SH: crate::c_int = 1;
// pub const LOCK_EX: crate::c_int = 2;
// pub const LOCK_NB: crate::c_int = 4;
// pub const LOCK_UN: crate::c_int = 8;

// pub const SS_ONSTACK: crate::c_int = 1;
// pub const SS_DISABLE: crate::c_int = 2;

// pub const PATH_MAX: crate::c_int = 4096;

// pub const FD_SETSIZE: usize = 1024;

// pub const EPOLLIN: crate::c_int = 0x1;
// pub const EPOLLPRI: crate::c_int = 0x2;
// pub const EPOLLOUT: crate::c_int = 0x4;
// pub const EPOLLRDNORM: crate::c_int = 0x40;
// pub const EPOLLRDBAND: crate::c_int = 0x80;
// pub const EPOLLWRNORM: crate::c_int = 0x100;
// pub const EPOLLWRBAND: crate::c_int = 0x200;
// pub const EPOLLMSG: crate::c_int = 0x400;
// pub const EPOLLERR: crate::c_int = 0x8;
// pub const EPOLLHUP: crate::c_int = 0x10;
// pub const EPOLLET: crate::c_int = 0x80000000;

// pub const EPOLL_CTL_ADD: crate::c_int = 1;
// pub const EPOLL_CTL_MOD: crate::c_int = 3;
// pub const EPOLL_CTL_DEL: crate::c_int = 2;

// pub const MNT_DETACH: crate::c_int = 0x2;
// pub const MNT_EXPIRE: crate::c_int = 0x4;

// pub const Q_GETFMT: crate::c_int = 0x800004;
// pub const Q_GETINFO: crate::c_int = 0x800005;
// pub const Q_SETINFO: crate::c_int = 0x800006;
// pub const QIF_BLIMITS: u32 = 1;
// pub const QIF_SPACE: u32 = 2;
// pub const QIF_ILIMITS: u32 = 4;
// pub const QIF_INODES: u32 = 8;
// pub const QIF_BTIME: u32 = 16;
// pub const QIF_ITIME: u32 = 32;
// pub const QIF_LIMITS: u32 = 5;
// pub const QIF_USAGE: u32 = 10;
// pub const QIF_TIMES: u32 = 48;
// pub const QIF_ALL: u32 = 63;

// pub const MNT_FORCE: crate::c_int = 0x1;

// pub const Q_SYNC: crate::c_int = 0x800001;
// pub const Q_QUOTAON: crate::c_int = 0x800002;
// pub const Q_QUOTAOFF: crate::c_int = 0x800003;
// pub const Q_GETQUOTA: crate::c_int = 0x800007;
// pub const Q_SETQUOTA: crate::c_int = 0x800008;

// pub const TCIOFF: crate::c_int = 2;
// pub const TCION: crate::c_int = 3;
// pub const TCOOFF: crate::c_int = 0;
// pub const TCOON: crate::c_int = 1;
// pub const TCIFLUSH: crate::c_int = 0;
// pub const TCOFLUSH: crate::c_int = 1;
// pub const TCIOFLUSH: crate::c_int = 2;
// pub const NL0: crate::tcflag_t = 0x00000000;
// pub const NL1: crate::tcflag_t = 0x00000100;
// pub const TAB0: crate::tcflag_t = 0x00000000;
// pub const CR0: crate::tcflag_t = 0x00000000;
// pub const FF0: crate::tcflag_t = 0x00000000;
// pub const BS0: crate::tcflag_t = 0x00000000;
// pub const VT0: crate::tcflag_t = 0x00000000;
// pub const VERASE: usize = 2;
// pub const VKILL: usize = 3;
// pub const VINTR: usize = 0;
// pub const VQUIT: usize = 1;
// pub const VLNEXT: usize = 15;
// pub const IGNBRK: crate::tcflag_t = 0x00000001;
// pub const BRKINT: crate::tcflag_t = 0x00000002;
// pub const IGNPAR: crate::tcflag_t = 0x00000004;
// pub const PARMRK: crate::tcflag_t = 0x00000008;
// pub const INPCK: crate::tcflag_t = 0x00000010;
// pub const ISTRIP: crate::tcflag_t = 0x00000020;
// pub const INLCR: crate::tcflag_t = 0x00000040;
// pub const IGNCR: crate::tcflag_t = 0x00000080;
// pub const ICRNL: crate::tcflag_t = 0x00000100;
// pub const IXANY: crate::tcflag_t = 0x00000800;
// pub const IMAXBEL: crate::tcflag_t = 0x00002000;
// pub const OPOST: crate::tcflag_t = 0x1;
// pub const CS5: crate::tcflag_t = 0x00000000;
// pub const CRTSCTS: crate::tcflag_t = 0x80000000;
// pub const ECHO: crate::tcflag_t = 0x00000008;
// pub const OCRNL:  crate::tcflag_t = 0o000010;
// pub const ONOCR:  crate::tcflag_t = 0o000020;
// pub const ONLRET: crate::tcflag_t = 0o000040;
// pub const OFILL:  crate::tcflag_t = 0o000100;
// pub const OFDEL:  crate::tcflag_t = 0o000200;

// pub const CLONE_VM: crate::c_int = 0x100;
// pub const CLONE_FS: crate::c_int = 0x200;
// pub const CLONE_FILES: crate::c_int = 0x400;
// pub const CLONE_SIGHAND: crate::c_int = 0x800;
// pub const CLONE_PTRACE: crate::c_int = 0x2000;
// pub const CLONE_VFORK: crate::c_int = 0x4000;
// pub const CLONE_PARENT: crate::c_int = 0x8000;
// pub const CLONE_THREAD: crate::c_int = 0x10000;
// pub const CLONE_NEWNS: crate::c_int = 0x20000;
// pub const CLONE_SYSVSEM: crate::c_int = 0x40000;
// pub const CLONE_SETTLS: crate::c_int = 0x80000;
// pub const CLONE_PARENT_SETTID: crate::c_int = 0x100000;
// pub const CLONE_CHILD_CLEARTID: crate::c_int = 0x200000;
// pub const CLONE_DETACHED: crate::c_int = 0x400000;
// pub const CLONE_UNTRACED: crate::c_int = 0x800000;
// pub const CLONE_CHILD_SETTID: crate::c_int = 0x01000000;
// pub const CLONE_NEWUTS: crate::c_int = 0x04000000;
// pub const CLONE_NEWIPC: crate::c_int = 0x08000000;
// pub const CLONE_NEWUSER: crate::c_int = 0x10000000;
// pub const CLONE_NEWPID: crate::c_int = 0x20000000;
// pub const CLONE_NEWNET: crate::c_int = 0x40000000;
// pub const CLONE_IO: crate::c_int = 0x80000000;
// pub const CLONE_NEWCGROUP: crate::c_int = 0x02000000;

// pub const WNOHANG: crate::c_int = 0x00000001;
// pub const WUNTRACED: crate::c_int = 0x00000002;
// pub const WSTOPPED: crate::c_int = WUNTRACED;
// pub const WEXITED: crate::c_int = 0x00000004;
// pub const WCONTINUED: crate::c_int = 0x00000008;
// pub const WNOWAIT: crate::c_int = 0x01000000;

// // Options set using PTRACE_SETOPTIONS.
// pub const PTRACE_O_TRACESYSGOOD: crate::c_int = 0x00000001;
// pub const PTRACE_O_TRACEFORK: crate::c_int = 0x00000002;
// pub const PTRACE_O_TRACEVFORK: crate::c_int = 0x00000004;
// pub const PTRACE_O_TRACECLONE: crate::c_int = 0x00000008;
// pub const PTRACE_O_TRACEEXEC: crate::c_int = 0x00000010;
// pub const PTRACE_O_TRACEVFORKDONE: crate::c_int = 0x00000020;
// pub const PTRACE_O_TRACEEXIT: crate::c_int = 0x00000040;
// pub const PTRACE_O_TRACESECCOMP: crate::c_int = 0x00000080;
// pub const PTRACE_O_EXITKILL: crate::c_int = 0x00100000;
// pub const PTRACE_O_SUSPEND_SECCOMP: crate::c_int = 0x00200000;
// pub const PTRACE_O_MASK: crate::c_int = 0x003000ff;

// // Wait extended result codes for the above trace options.
// pub const PTRACE_EVENT_FORK: crate::c_int = 1;
// pub const PTRACE_EVENT_VFORK: crate::c_int = 2;
// pub const PTRACE_EVENT_CLONE: crate::c_int = 3;
// pub const PTRACE_EVENT_EXEC: crate::c_int = 4;
// pub const PTRACE_EVENT_VFORK_DONE: crate::c_int = 5;
// pub const PTRACE_EVENT_EXIT: crate::c_int = 6;
// pub const PTRACE_EVENT_SECCOMP: crate::c_int = 7;
// // PTRACE_EVENT_STOP was added to glibc in 2.26
// // pub const PTRACE_EVENT_STOP: crate::c_int = 128;

// pub const __WNOTHREAD: crate::c_int = 0x20000000;
// pub const __WALL: crate::c_int = 0x40000000;
// pub const __WCLONE: crate::c_int = 0x80000000;

// pub const SPLICE_F_MOVE: crate::c_uint = 0x01;
// pub const SPLICE_F_NONBLOCK: crate::c_uint = 0x02;
// pub const SPLICE_F_MORE: crate::c_uint = 0x04;
// pub const SPLICE_F_GIFT: crate::c_uint = 0x08;

// pub const RTLD_LOCAL: crate::c_int = 0;
// pub const RTLD_LAZY: crate::c_int = 1;

// pub const POSIX_FADV_NORMAL: crate::c_int = 0;
// pub const POSIX_FADV_RANDOM: crate::c_int = 1;
// pub const POSIX_FADV_SEQUENTIAL: crate::c_int = 2;
// pub const POSIX_FADV_WILLNEED: crate::c_int = 3;

// pub const AT_FDCWD: crate::c_int = -100;
// pub const AT_SYMLINK_NOFOLLOW: crate::c_int = 0x100;
// pub const AT_REMOVEDIR: crate::c_int = 0x200;
// pub const AT_SYMLINK_FOLLOW: crate::c_int = 0x400;
// pub const AT_NO_AUTOMOUNT: crate::c_int = 0x800;
// pub const AT_EMPTY_PATH: crate::c_int = 0x1000;

// pub const LOG_CRON: crate::c_int = 9 << 3;
// pub const LOG_AUTHPRIV: crate::c_int = 10 << 3;
// pub const LOG_FTP: crate::c_int = 11 << 3;
// pub const LOG_PERROR: crate::c_int = 0x20;

// pub const PIPE_BUF: usize = 4096;

// pub const SI_LOAD_SHIFT: crate::c_uint = 16;

// pub const SIGEV_SIGNAL: crate::c_int = 0;
// pub const SIGEV_NONE: crate::c_int = 1;
// pub const SIGEV_THREAD: crate::c_int = 2;

// pub const P_ALL: crate::idtype_t = 0;
// pub const P_PID: crate::idtype_t = 1;
// pub const P_PGID: crate::idtype_t = 2;

// pub const UTIME_OMIT: crate::c_long = 1073741822;
// pub const UTIME_NOW: crate::c_long = 1073741823;

// pub const POLLIN: crate::c_short = 0x1;
// pub const POLLPRI: crate::c_short = 0x2;
// pub const POLLOUT: crate::c_short = 0x4;
// pub const POLLERR: crate::c_short = 0x8;
// pub const POLLHUP: crate::c_short = 0x10;
// pub const POLLNVAL: crate::c_short = 0x20;
// pub const POLLRDNORM: crate::c_short = 0x040;
// pub const POLLRDBAND: crate::c_short = 0x080;

// pub const IPTOS_LOWDELAY: u8 = 0x10;
// pub const IPTOS_THROUGHPUT: u8 = 0x08;
// pub const IPTOS_RELIABILITY: u8 = 0x04;
// pub const IPTOS_MINCOST: u8 = 0x02;

// pub const IPTOS_PREC_NETCONTROL: u8 = 0xe0;
// pub const IPTOS_PREC_INTERNETCONTROL: u8 = 0xc0;
// pub const IPTOS_PREC_CRITIC_ECP: u8 = 0xa0;
// pub const IPTOS_PREC_FLASHOVERRIDE: u8 = 0x80;
// pub const IPTOS_PREC_FLASH: u8 = 0x60;
// pub const IPTOS_PREC_IMMEDIATE: u8 = 0x40;
// pub const IPTOS_PREC_PRIORITY: u8 = 0x20;
// pub const IPTOS_PREC_ROUTINE: u8 = 0x00;

// pub const IPTOS_ECN_MASK: u8 = 0x03;
// pub const IPTOS_ECN_ECT1: u8 = 0x01;
// pub const IPTOS_ECN_ECT0: u8 = 0x02;
// pub const IPTOS_ECN_CE: u8 = 0x03;

// pub const IPOPT_COPY: u8 = 0x80;
// pub const IPOPT_CLASS_MASK: u8 = 0x60;
// pub const IPOPT_NUMBER_MASK: u8 = 0x1f;

// pub const IPOPT_CONTROL: u8 = 0x00;
// pub const IPOPT_RESERVED1: u8 = 0x20;
// pub const IPOPT_MEASUREMENT: u8 = 0x40;
// pub const IPOPT_RESERVED2: u8 = 0x60;
// pub const IPOPT_END: u8 = (0 |IPOPT_CONTROL);
// pub const IPOPT_NOOP: u8 = (1 |IPOPT_CONTROL);
// pub const IPOPT_SEC: u8 = (2 |IPOPT_CONTROL|IPOPT_COPY);
// pub const IPOPT_LSRR: u8 = (3 |IPOPT_CONTROL|IPOPT_COPY);
// pub const IPOPT_TIMESTAMP: u8 = (4 |IPOPT_MEASUREMENT);
// pub const IPOPT_RR: u8 = (7 |IPOPT_CONTROL);
// pub const IPOPT_SID: u8 = (8 |IPOPT_CONTROL|IPOPT_COPY);
// pub const IPOPT_SSRR: u8 = (9 |IPOPT_CONTROL|IPOPT_COPY);
// pub const IPOPT_RA: u8 = (20|IPOPT_CONTROL|IPOPT_COPY);
// pub const IPVERSION: u8 = 4;
// pub const MAXTTL: u8 = 255;
// pub const IPDEFTTL: u8 = 64;
// pub const IPOPT_OPTVAL: u8 = 0;
// pub const IPOPT_OLEN: u8 = 1;
// pub const IPOPT_OFFSET: u8 = 2;
// pub const IPOPT_MINOFF: u8 = 4;
// pub const MAX_IPOPTLEN: u8 = 40;
// pub const IPOPT_NOP: u8 = IPOPT_NOOP;
// pub const IPOPT_EOL: u8 = IPOPT_END;
// pub const IPOPT_TS: u8 = IPOPT_TIMESTAMP;
// pub const IPOPT_TS_TSONLY: u8 = 0;
// pub const IPOPT_TS_TSANDADDR: u8 = 1;
// pub const IPOPT_TS_PRESPEC: u8 = 3;

// pub const ARPOP_RREQUEST: u16 = 3;
// pub const ARPOP_RREPLY: u16 = 4;
// pub const ARPOP_InREQUEST: u16 = 8;
// pub const ARPOP_InREPLY: u16 = 9;
// pub const ARPOP_NAK: u16 = 10;

// pub const ATF_NETMASK: crate::c_int = 0x20;
// pub const ATF_DONTPUB: crate::c_int = 0x40;

// pub const ARPHRD_NETROM: u16 = 0;
// pub const ARPHRD_ETHER: u16 = 1;
// pub const ARPHRD_EETHER: u16 = 2;
// pub const ARPHRD_AX25: u16 = 3;
// pub const ARPHRD_PRONET: u16 = 4;
// pub const ARPHRD_CHAOS: u16 = 5;
// pub const ARPHRD_IEEE802: u16 = 6;
// pub const ARPHRD_ARCNET: u16 = 7;
// pub const ARPHRD_APPLETLK: u16 = 8;
// pub const ARPHRD_DLCI: u16 = 15;
// pub const ARPHRD_ATM: u16 = 19;
// pub const ARPHRD_METRICOM: u16 = 23;
// pub const ARPHRD_IEEE1394: u16 = 24;
// pub const ARPHRD_EUI64: u16 = 27;
// pub const ARPHRD_INFINIBAND: u16 = 32;

// pub const ARPHRD_SLIP: u16 = 256;
// pub const ARPHRD_CSLIP: u16 = 257;
// pub const ARPHRD_SLIP6: u16 = 258;
// pub const ARPHRD_CSLIP6: u16 = 259;
// pub const ARPHRD_RSRVD: u16 = 260;
// pub const ARPHRD_ADAPT: u16 = 264;
// pub const ARPHRD_ROSE: u16 = 270;
// pub const ARPHRD_X25: u16 = 271;
// pub const ARPHRD_HWX25: u16 = 272;
// pub const ARPHRD_PPP: u16 = 512;
// pub const ARPHRD_CISCO: u16 = 513;
// pub const ARPHRD_HDLC: u16 = ARPHRD_CISCO;
// pub const ARPHRD_LAPB: u16 = 516;
// pub const ARPHRD_DDCMP: u16 = 517;
// pub const ARPHRD_RAWHDLC: u16 = 518;

// pub const ARPHRD_TUNNEL: u16 = 768;
// pub const ARPHRD_TUNNEL6: u16 = 769;
// pub const ARPHRD_FRAD: u16 = 770;
// pub const ARPHRD_SKIP: u16 = 771;
// pub const ARPHRD_LOOPBACK: u16 = 772;
// pub const ARPHRD_LOCALTLK: u16 = 773;
// pub const ARPHRD_FDDI: u16 = 774;
// pub const ARPHRD_BIF: u16 = 775;
// pub const ARPHRD_SIT: u16 = 776;
// pub const ARPHRD_IPDDP: u16 = 777;
// pub const ARPHRD_IPGRE: u16 = 778;
// pub const ARPHRD_PIMREG: u16 = 779;
// pub const ARPHRD_HIPPI: u16 = 780;
// pub const ARPHRD_ASH: u16 = 781;
// pub const ARPHRD_ECONET: u16 = 782;
// pub const ARPHRD_IRDA: u16 = 783;
// pub const ARPHRD_FCPP: u16 = 784;
// pub const ARPHRD_FCAL: u16 = 785;
// pub const ARPHRD_FCPL: u16 = 786;
// pub const ARPHRD_FCFABRIC: u16 = 787;
// pub const ARPHRD_IEEE802_TR: u16 = 800;
// pub const ARPHRD_IEEE80211: u16 = 801;
// pub const ARPHRD_IEEE80211_PRISM: u16 = 802;
// pub const ARPHRD_IEEE80211_RADIOTAP: u16 = 803;
// pub const ARPHRD_IEEE802154: u16 = 804;

// pub const ARPHRD_VOID: u16 = 0xFFFF;
// pub const ARPHRD_NONE: u16 = 0xFFFE;