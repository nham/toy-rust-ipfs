use commands::HelpText;

pub const RootHelpText: HelpText = HelpText {
    tagline: "global p2p merkle-dag filesystem",
    synopsis: r#"
ipfs [<flags>] <command> [<arg>] ...
"#,
    short_desc: r#"
BASIC COMMANDS

    init          Initialize ipfs local configuration
    add <path>    Add an object to ipfs
    cat <ref>     Show ipfs object data
    get <ref>     Download ipfs objects
    ls <ref>      List links from an object
    refs <ref>    List hashes of links from an object

DATA STRUCTURE COMMANDS

    block         Interact with raw blocks in the datastore
    object        Interact with raw dag nodes
    file          Interact with Unix filesystem objects

ADVANCED COMMANDS

    daemon        Start a long-running daemon process
    mount         Mount an ipfs read-only mountpoint
    resolve       Resolve any type of name
    name          Publish or resolve IPNS names
    dns           Resolve DNS links
    pin           Pin objects to local storage
    repo gc       Garbage collect unpinned objects

NETWORK COMMANDS

    id            Show info about ipfs peers
    bootstrap     Add or remove bootstrap peers
    swarm         Manage connections to the p2p network
    dht           Query the dht for values or peers
    ping          Measure the latency of a connection
    diag          Print diagnostics

TOOL COMMANDS

    config        Manage configuration
    version       Show ipfs version information
    update        Download and apply go-ipfs updates
    commands      List all available commands

Use 'ipfs <command> --help' to learn more about each command.
"#,
};
