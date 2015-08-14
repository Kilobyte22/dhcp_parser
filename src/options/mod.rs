mod parse;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum DhcpOption {
    Pad = 0,
    SubnetMask = 1,
    TimeOffset = 2,
    Router = 3,
    TimeServer = 4,
    NameServer = 5,
    DomainNameServer = 6,
    LogServer = 7,
    CookieServer = 8,
    LPRServer = 9,
    ImpressServer = 10,
    ResourceLocationServer = 11,
    HostName = 12,
    BootFileSize = 13,
    MeritDumpFile = 14,
    DomainName = 15,
    SwapServer = 16,
    RootPath = 17,
    ExtensionsPath = 18,
    IPForwarding = 19,
    NonLocalSourceRouting = 20,
    PolicyFilter = 21,
    MaxDatagramReassemblySize = 22,
    DefaultIPTTL = 23,
    PathMTUAgingTimeout = 24,
    PathMTUPlateauTable = 25,
    InterfaceMTU = 26,
    AllSubnetsAreLocal = 27,
    BroadcastAddress = 28,
    PerformMaskDiscovery = 29,
    MaskSupplier = 30,
    PerformRouterDiscovery = 31,
    RouterSolicitationAddress = 32,
    StaticRoute = 33,
    TrailerEncapsulation = 34,
    ARPCacheTimeout = 35,
    EthernetEncapsulation = 36,
    TCPDefaultTTL = 37,
    TCPKeepaliveInterval = 38,
    TCPKeepaliveGarbage = 39,
    NISDomain = 40,
    NetworkInformationServers = 41,
    NTPServers = 42,
    VendorExtensions = 43,
    NetBIOSNameServers = 44,
    NetBIOSDatagramDistributionServer = 45,
    NetBIOSNodeType = 46,
    NetBIOSScope = 47,
    XFontServer = 48,
    XDisplayManager = 49,
    // DHCP-specific options
    RequestedIPAddress = 50,
    IPAddressLeaseTime = 51,
    OptionOverload = 52,
    MessageType = 53,
    ServerIdentifier = 54,
    ParamRequestList = 55,
    Message = 56,
    MaxMessageSize = 57,
    RenewalTimeValue = 58,
    RebindingTimeValue = 59,
    ClassIdentifier = 60,
    ClientIdentifier = 61,
    End = 255,
}
}