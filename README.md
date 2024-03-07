# PCAP Logger

## Purposes:
This tool simply takes packets capture from libpcap, pulls out the IP header from the ethernet frame, and logs data in a postgres database (though other databases can easily be added).

This tool was created as part of the federation wireless project who's aim is to develop a completely open source, wireless router similar to pfscene, but aimed for end users, rather than IT departments.

## Configuration format:

The configuration uses JSON to specify database, interface, and filters, though the filter syntax parsing is not yet implemented.

### To run
```
cargo build --release
./target/release/capture -c ./path/to/config.json
```

### Example configuration:

```
{
    "interface":"wlp2s0",
    "logger": {
        "url":"postgresql://user@localhost/database"
    }
}
```