pub const IPV4_MULTICAST: &str = "224.0.0.0/4";
pub const IPV6_MULTICAST: &str = "ff00::/8";

pub const IPV4_PRIVATE_NETWORKS: [&str; 4] = [
    "169.254.0.0/16",
    "192.168.0.0/16",
    "172.16.0.0/12",
    "10.0.0.0/8",
];
pub const IPV6_PRIVATE_NETWORKS: [&str; 2] = ["fe80::/10", "fc00::/7"];

pub const IPV4_NOT_ROUTABLE_MULTICASTS: [&str; 1] = ["224.0.0.0/24"];
pub const IPV6_NOT_ROUTABLE_MULTICASTS: [&str; 4] =
    ["ff01::/16", "ff11::/16", "ff02::/16", "ff12::/16"];

#[cfg(unix)]
pub const DEFAULT_CONF_DIR: &str = "~/.config/me.lucky.netlock/";
pub const FIREWALL_CONF_FILE_NAME: &str = "firewall.conf";
pub const SETTINGS_CONF_FILE_NAME: &str = "settings.conf";
