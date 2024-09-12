use esp_idf_svc::sys;

macro_rules! check_constants_manually {
    ($name:expr, $const1:expr, $const2:expr) => {
        if $const1.to_string() != $const2.to_string() {
            log::error!("`{}`is different: {} != {}", $name, $const1, $const1);
        }
    };
}

macro_rules! check_constants {
    ($ident:ident) => {
        if sys::$ident.to_string() != libc::$ident.to_string() {
            log::error!(
                "`{}` is different: {} != {}",
                stringify!($ident),
                sys::$ident,
                libc::$ident
            );
        }
    };
}

macro_rules! check_types_manually {
    ($name:expr, $size1:expr, $size2:expr, $align1:expr, $align2:expr) => {
        if $size1.to_string() != $size2.to_string() {
            log::error!("`{}` size is different: {} != {}", $name, $size1, $size2);
        }
        if $align1.to_string() != $align2.to_string() {
            log::error!(
                "`{}` alignment is different: {} != {}",
                $name,
                $align1,
                $align2
            );
        }
    };
}

macro_rules! check_types {
    ($ident:ident) => {
        if std::mem::size_of::<sys::$ident>() != std::mem::size_of::<libc::$ident>() {
            log::error!(
                "`{}` size is different: {} != {}",
                stringify!($ident),
                std::mem::size_of::<sys::$ident>(),
                std::mem::size_of::<libc::$ident>()
            );
        }
        if std::mem::align_of::<sys::$ident>() != std::mem::align_of::<libc::$ident>() {
            log::error!(
                "`{}` alignment is different: {} != {}",
                stringify!($ident),
                std::mem::align_of::<sys::$ident>(),
                std::mem::align_of::<libc::$ident>()
            );
        }
    };
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // newlib/espidf module

    check_types!(clock_t);
    check_types!(cmsghdr);
    check_types!(msghdr);
    // check_types!(sockaddr_un);
    check_types!(sockaddr);
    check_types!(sockaddr_in6);
    check_types!(sockaddr_in);
    check_types!(sockaddr_storage);

    // check_constants!(AF_UNIX);
    check_constants!(AF_INET6);

    // check_constants!(FIONBIO);

    check_constants!(POLLIN);
    check_constants!(POLLRDNORM);
    check_constants!(POLLRDBAND);
    check_constants!(POLLPRI);
    check_constants!(POLLOUT);
    check_constants!(POLLWRNORM);
    check_constants!(POLLWRBAND);
    check_constants!(POLLERR);
    check_constants!(POLLHUP);

    check_constants!(SOL_SOCKET);

    check_constants!(MSG_OOB);
    check_constants!(MSG_PEEK);
    check_constants!(MSG_DONTWAIT);
    check_constants!(MSG_DONTROUTE);
    check_constants!(MSG_WAITALL);
    check_constants!(MSG_MORE);
    check_constants!(MSG_NOSIGNAL);
    check_constants!(MSG_TRUNC);
    check_constants!(MSG_CTRUNC);
    check_constants!(MSG_EOR);

    check_constants!(PTHREAD_STACK_MIN);

    check_constants!(SIGABRT);
    check_constants!(SIGFPE);
    check_constants!(SIGILL);
    check_constants!(SIGINT);
    check_constants!(SIGSEGV);
    check_constants!(SIGTERM);
    check_constants!(SIGHUP);
    check_constants!(SIGQUIT);
    check_constants!(NSIG);

    // newlib module
    check_types!(blkcnt_t);
    check_types!(blksize_t);
    check_types!(clockid_t);
    check_types!(dev_t);
    check_types!(ino_t);
    check_types!(off_t);
    check_types!(fsblkcnt_t);
    check_types!(fsfilcnt_t);
    check_types!(id_t);
    check_types!(key_t);
    check_types_manually!(
        "loff_t",
        std::mem::size_of::<esp_idf_svc::hal::sys::__loff_t>(),
        std::mem::size_of::<libc::loff_t>(),
        std::mem::align_of::<esp_idf_svc::hal::sys::__loff_t>(),
        std::mem::align_of::<libc::loff_t>()
    );
    check_types!(mode_t);
    check_types!(nfds_t);
    check_types!(nlink_t);
    check_types!(pthread_t);
    check_types!(pthread_key_t);
    // check_types!(rlim_t);
    check_types!(sa_family_t);
    check_types!(socklen_t);
    check_types!(speed_t);
    check_types!(suseconds_t);
    check_types!(tcflag_t);
    check_types!(useconds_t);
    check_types!(time_t);
    check_types!(hostent);
    check_types!(addrinfo);
    check_types!(ip_mreq);
    check_types!(linger);
    check_types!(in_addr);
    check_types!(pollfd);
    // check_types!(lconv);
    check_types!(tm);
    // check_types!(statvfs);
    check_types!(sigaction);
    check_types!(stack_t);
    check_types!(fd_set);
    // check_types!(passwd);
    check_types!(termios);
    // check_types!(sem_t);
    // check_types!(Dl_info);
    // check_types!(utsname);
    // check_types!(cpu_set_t);
    check_types!(pthread_attr_t);
    // check_types!(pthread_rwlockattr_t);

    check_constants!(NCCS);

    check_constants!(PTHREAD_MUTEX_NORMAL);
    check_constants!(PTHREAD_MUTEX_RECURSIVE);
    check_constants!(PTHREAD_MUTEX_ERRORCHECK);

    check_constants!(FD_SETSIZE);

    check_constants!(EPERM);
    check_constants!(ENOENT);
    check_constants!(ESRCH);
    check_constants!(EINTR);
    check_constants!(EIO);
    check_constants!(ENXIO);
    check_constants!(E2BIG);
    check_constants!(ENOEXEC);
    check_constants!(EBADF);
    check_constants!(ECHILD);
    check_constants!(EAGAIN);
    check_constants!(ENOMEM);
    check_constants!(EACCES);
    check_constants!(EFAULT);
    check_constants!(EBUSY);
    check_constants!(EEXIST);
    check_constants!(EXDEV);
    check_constants!(ENODEV);
    check_constants!(ENOTDIR);
    check_constants!(EISDIR);
    check_constants!(EINVAL);
    check_constants!(ENFILE);
    check_constants!(EMFILE);
    check_constants!(ENOTTY);
    check_constants!(ETXTBSY);
    check_constants!(EFBIG);
    check_constants!(ENOSPC);
    check_constants!(ESPIPE);
    check_constants!(EROFS);
    check_constants!(EMLINK);
    check_constants!(EPIPE);
    check_constants!(EDOM);
    check_constants!(ERANGE);
    check_constants!(ENOMSG);
    check_constants!(EIDRM);
    check_constants!(EDEADLK);
    check_constants!(ENOLCK);
    check_constants!(ENOSTR);
    check_constants!(ENODATA);
    check_constants!(ETIME);
    check_constants!(ENOSR);
    check_constants!(ENOLINK);
    check_constants!(EPROTO);
    check_constants!(EMULTIHOP);
    check_constants!(EBADMSG);
    check_constants!(EFTYPE);
    check_constants!(ENOSYS);
    check_constants!(ENOTEMPTY);
    check_constants!(ENAMETOOLONG);
    check_constants!(ELOOP);
    check_constants!(EOPNOTSUPP);
    check_constants!(EPFNOSUPPORT);
    check_constants!(ECONNRESET);
    check_constants!(ENOBUFS);
    check_constants!(EAFNOSUPPORT);
    check_constants!(EPROTOTYPE);
    check_constants!(ENOTSOCK);
    check_constants!(ENOPROTOOPT);
    check_constants!(ECONNREFUSED);
    check_constants!(EADDRINUSE);
    check_constants!(ECONNABORTED);
    check_constants!(ENETUNREACH);
    check_constants!(ENETDOWN);
    check_constants!(ETIMEDOUT);
    check_constants!(EHOSTDOWN);
    check_constants!(EHOSTUNREACH);
    check_constants!(EINPROGRESS);
    check_constants!(EALREADY);
    check_constants!(EDESTADDRREQ);
    check_constants!(EMSGSIZE);
    check_constants!(EPROTONOSUPPORT);
    check_constants!(EADDRNOTAVAIL);
    check_constants!(ENETRESET);
    check_constants!(EISCONN);
    check_constants!(ENOTCONN);
    check_constants!(ETOOMANYREFS);
    check_constants!(EDQUOT);
    check_constants!(ESTALE);
    check_constants!(ENOTSUP);
    check_constants!(EILSEQ);
    check_constants!(EOVERFLOW);
    check_constants!(ECANCELED);
    check_constants!(ENOTRECOVERABLE);
    check_constants!(EOWNERDEAD);
    check_constants!(EWOULDBLOCK);

    check_constants!(F_DUPFD);
    check_constants!(F_GETFD);
    check_constants!(F_SETFD);
    check_constants!(F_GETFL);
    check_constants!(F_SETFL);
    check_constants!(F_GETOWN);
    check_constants!(F_SETOWN);
    check_constants!(F_GETLK);
    check_constants!(F_SETLK);
    check_constants!(F_SETLKW);
    check_constants!(F_RGETLK);
    check_constants!(F_RSETLK);
    check_constants!(F_CNVT);
    check_constants!(F_RSETLKW);
    check_constants!(F_DUPFD_CLOEXEC);

    check_constants!(O_RDONLY);
    check_constants!(O_WRONLY);
    check_constants!(O_RDWR);
    check_constants!(O_APPEND);
    check_constants!(O_CREAT);
    check_constants!(O_TRUNC);
    check_constants!(O_EXCL);
    check_constants!(O_SYNC);
    check_constants!(O_NONBLOCK);

    // check_constants!(O_ACCMODE);
    check_constants!(O_CLOEXEC);

    // check_constants!(RTLD_LAZY);

    check_constants!(STDIN_FILENO);
    check_constants!(STDOUT_FILENO);
    check_constants!(STDERR_FILENO);

    check_constants!(SEEK_SET);
    check_constants!(SEEK_CUR);
    check_constants!(SEEK_END);

    // check_constants!(FIOCLEX);
    // check_constants!(FIONCLEX);

    check_constants!(S_BLKSIZE);
    check_constants!(S_IREAD);
    check_constants!(S_IWRITE);
    check_constants!(S_IEXEC);
    check_constants!(S_ENFMT);
    check_constants!(S_IFMT);
    check_constants!(S_IFDIR);
    check_constants!(S_IFCHR);
    check_constants!(S_IFBLK);
    check_constants!(S_IFREG);
    check_constants!(S_IFLNK);
    check_constants!(S_IFSOCK);
    check_constants!(S_IFIFO);
    check_constants!(S_IRUSR);
    check_constants!(S_IWUSR);
    check_constants!(S_IXUSR);
    check_constants!(S_IRGRP);
    check_constants!(S_IWGRP);
    check_constants!(S_IXGRP);
    check_constants!(S_IROTH);
    check_constants!(S_IWOTH);
    check_constants!(S_IXOTH);

    // check_constants!(SOL_TCP);

    check_constants!(PF_UNSPEC);
    check_constants!(PF_INET);
    check_constants!(PF_INET6);

    check_constants!(AF_UNSPEC);
    check_constants!(AF_INET);

    // check_constants!(CLOCK_REALTIME);
    // check_constants!(CLOCK_MONOTONIC);
    // check_constants!(CLOCK_BOOTTIME);

    check_constants!(SOCK_STREAM);
    check_constants!(SOCK_DGRAM);

    check_constants!(SHUT_RD);
    check_constants!(SHUT_WR);
    check_constants!(SHUT_RDWR);

    // check_constants!(SO_BINTIME);
    // check_constants!(SO_NO_OFFLOAD);
    // check_constants!(SO_NO_DDP);
    // check_constants!(SO_REUSEPORT_LB);
    // check_constants!(SO_LABEL);
    // check_constants!(SO_PEERLABEL);
    // check_constants!(SO_LISTENQLIMIT);
    // check_constants!(SO_LISTENQLEN);
    // check_constants!(SO_LISTENINCQLEN);
    // check_constants!(SO_SETFIB);
    // check_constants!(SO_USER_COOKIE);
    // check_constants!(SO_PROTOCOL);
    // check_constants!(SO_PROTOTYPE);
    // check_constants!(SO_VENDOR);
    check_constants!(SO_DEBUG);
    check_constants!(SO_ACCEPTCONN);
    check_constants!(SO_REUSEADDR);
    check_constants!(SO_KEEPALIVE);
    check_constants!(SO_DONTROUTE);
    check_constants!(SO_BROADCAST);
    check_constants!(SO_USELOOPBACK);
    check_constants!(SO_LINGER);
    check_constants!(SO_OOBINLINE);
    check_constants!(SO_REUSEPORT);
    // check_constants!(SO_TIMESTAMP);
    // check_constants!(SO_NOSIGPIPE);
    // check_constants!(SO_ACCEPTFILTER);
    check_constants!(SO_SNDBUF);
    check_constants!(SO_RCVBUF);
    check_constants!(SO_SNDLOWAT);
    check_constants!(SO_RCVLOWAT);
    check_constants!(SO_SNDTIMEO);
    check_constants!(SO_RCVTIMEO);

    check_constants!(SO_ERROR);

    check_constants!(SO_TYPE);
    check_constants_manually!("SOCK_CLOEXEC", sys::O_CLOEXEC, libc::SOCK_CLOEXEC);
    check_constants!(INET_ADDRSTRLEN);

    // check_constants!(IFF_UP);
    // check_constants!(IFF_BROADCAST);
    // check_constants!(IFF_DEBUG);
    // check_constants!(IFF_LOOPBACK);
    // check_constants!(IFF_POINTOPOINT);
    // check_constants!(IFF_NOTRAILERS);
    // check_constants!(IFF_RUNNING);
    // check_constants!(IFF_NOARP);
    // check_constants!(IFF_PROMISC);
    // check_constants!(IFF_ALLMULTI);
    // check_constants!(IFF_OACTIVE);
    // check_constants!(IFF_SIMPLEX);
    // check_constants!(IFF_LINK0);
    // check_constants!(IFF_LINK1);
    // check_constants!(IFF_LINK2);
    // check_constants!(IFF_ALTPHYS);
    // check_constants!(IFF_MULTICAST);

    check_constants!(TCP_NODELAY);
    // check_constants!(TCP_MAXSEG);

    // check_constants!(TCP_NOPUSH);
    // check_constants!(TCP_NOOPT);
    check_constants!(TCP_KEEPIDLE);
    check_constants!(TCP_KEEPINTVL);
    check_constants!(TCP_KEEPCNT);

    check_constants!(IP_TOS);

    check_constants!(IP_TTL);

    check_constants!(IP_MULTICAST_IF);
    check_constants!(IP_MULTICAST_TTL);
    check_constants!(IP_MULTICAST_LOOP);

    check_constants!(IP_ADD_MEMBERSHIP);

    check_constants!(IP_DROP_MEMBERSHIP);

    check_constants!(IPV6_UNICAST_HOPS);
    check_constants!(IPV6_MULTICAST_IF);
    check_constants!(IPV6_MULTICAST_HOPS);
    check_constants!(IPV6_MULTICAST_LOOP);
    check_constants!(IPV6_V6ONLY);
    check_constants!(IPV6_JOIN_GROUP);
    check_constants!(IPV6_LEAVE_GROUP);
    check_constants!(IPV6_ADD_MEMBERSHIP);
    check_constants!(IPV6_DROP_MEMBERSHIP);

    check_constants!(HOST_NOT_FOUND);
    check_constants!(NO_DATA);
    // check_constants!(NO_ADDRESS);
    check_constants!(NO_RECOVERY);
    check_constants!(TRY_AGAIN);

    check_constants!(AI_PASSIVE);
    check_constants!(AI_CANONNAME);
    check_constants!(AI_NUMERICHOST);
    check_constants!(AI_NUMERICSERV);
    check_constants!(AI_ADDRCONFIG);

    check_constants!(NI_MAXHOST);
    check_constants!(NI_MAXSERV);
    // check_constants!(NI_NOFQDN);
    // check_constants!(NI_NUMERICHOST);
    // check_constants!(NI_NAMEREQD);
    check_constants!(NI_NUMERICSERV);
    check_constants!(NI_DGRAM);

    check_constants!(EAI_FAMILY);
    check_constants!(EAI_MEMORY);
    check_constants!(EAI_NONAME);
    check_constants!(EAI_SOCKTYPE);

    check_constants!(EXIT_SUCCESS);
    check_constants!(EXIT_FAILURE);

    // check_constants!(PRIO_PROCESS);
    // check_constants!(PRIO_PGRP);
    // check_constants!(PRIO_USER);
}
