use std::fmt::Display;

/// A binary target.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum BinaryTarget {
    /// `linux-musl`
    ///
    /// OpenSSL: 1.1.x
    /// Distros: Alpine Linux <= 3.16
    AlpineOpenSsl1_1,
    /// `linux-musl-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Alpine Linux >= 3.17
    AlpineOpenSsl3_0,
    /// `linux-arm64-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl1_0,
    /// `linux-arm64-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl1_1,
    /// `linux-arm64-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: ARM64-based linux
    Arm64OpenSsl3_0,
    /// `darwin`
    ///
    /// macOS Intel x86
    Darwin,
    /// `darwin-arm64`
    ///
    /// macOS ARM64
    DarwinArm64,
    /// `debian-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros:
    /// * Debian 8
    /// * Ubuntu 16.04
    /// * Ubuntu 18.04
    /// * Mint 18
    DebianOpenSsl1_0,
    /// `debian-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros:
    /// * Arch 2019.09.01
    /// * Debian 9
    /// * Debian 10
    /// * Debian 11
    /// * Mint 19
    /// * Ubuntu 18.04
    /// * Ubuntu 19.04
    /// * Ubuntu 20.04
    /// * Ubuntu 21.04
    DebianOpenSsl1_1,
    /// `debian-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Ubuntu 22.04
    DebianOpenSsl3_0,
    /// `native`
    ///
    /// The native platform.
    #[default]
    Native,
    /// `rhel-openssl-1.0.x`
    ///
    /// OpenSSL: 1.0.x
    /// Distros:
    /// * CentOS 6
    /// * CentOS 7
    RhelOpenSsl1_0,
    /// `rhel-openssl-1.1.x`
    ///
    /// OpenSSL: 1.1.x
    /// Distros:
    /// * Fedora 28
    /// * Fedora 29
    /// * Fedora 30
    RhelOpenSsl1_1,
    /// `rhel-openssl-3.0.x`
    ///
    /// OpenSSL: 3.0.x
    /// Distros: Fedora 31
    RhelOpenSsl3_0,
    /// `windows`
    ///
    /// Windows
    Windows,
}

impl Display for BinaryTarget {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::AlpineOpenSsl1_1 => "linux-musl",
                Self::AlpineOpenSsl3_0 => "linux-musl-openssl-3.0.x",
                Self::Arm64OpenSsl1_0 => "linux-arm64-openssl-1.0.x",
                Self::Arm64OpenSsl1_1 => "linux-arm64-openssl-1.1.x",
                Self::Arm64OpenSsl3_0 => "linux-arm64-openssl-3.0.x",
                Self::Darwin => "darwin",
                Self::DarwinArm64 => "darwin-arm64",
                Self::DebianOpenSsl1_0 => "debian-openssl-1.0.x",
                Self::DebianOpenSsl1_1 => "debian-openssl-1.1.x",
                Self::DebianOpenSsl3_0 => "debian-openssl-3.0.x",
                Self::Native => "native",
                Self::RhelOpenSsl1_0 => "rhel-openssl-1.0.x",
                Self::RhelOpenSsl1_1 => "rhel-openssl-1.1.x",
                Self::RhelOpenSsl3_0 => "rhel-openssl-3.0.x",
                Self::Windows => "windows",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(BinaryTarget::default(), BinaryTarget::Native);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            BinaryTarget::AlpineOpenSsl1_1.to_string(),
            "\"linux-musl\""
        );

        assert_eq!(
            BinaryTarget::AlpineOpenSsl3_0.to_string(),
            "\"linux-musl-openssl-3.0.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl1_0.to_string(),
            "\"linux-arm64-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl1_1.to_string(),
            "\"linux-arm64-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::Arm64OpenSsl3_0.to_string(),
            "\"linux-arm64-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Darwin.to_string(), "\"darwin\"");

        assert_eq!(BinaryTarget::DarwinArm64.to_string(), "\"darwin-arm64\"");

        assert_eq!(
            BinaryTarget::DebianOpenSsl1_0.to_string(),
            "\"debian-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::DebianOpenSsl1_1.to_string(),
            "\"debian-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::DebianOpenSsl3_0.to_string(),
            "\"debian-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Native.to_string(), "\"native\"");

        assert_eq!(
            BinaryTarget::RhelOpenSsl1_0.to_string(),
            "\"rhel-openssl-1.0.x\""
        );

        assert_eq!(
            BinaryTarget::RhelOpenSsl1_1.to_string(),
            "\"rhel-openssl-1.1.x\""
        );

        assert_eq!(
            BinaryTarget::RhelOpenSsl3_0.to_string(),
            "\"rhel-openssl-3.0.x\""
        );

        assert_eq!(BinaryTarget::Windows.to_string(), "\"windows\"");
    }
}
