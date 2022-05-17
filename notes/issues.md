# Issues

## Docker volumes with SQLite WAL mode.

- When the target DB is SQLite, `tdog` writes to SQLite files with WAL mode enabled.
- https://sqlite.org/wal.html
- WAL mode enables better performance and supports concurrent readers and a single writer.
- WAL mode cannot be used on network filesystems.
    - It uses the OS's shared memory primitives; multiple OS's do not know about each other's state.
    - "Network filesystems" in this context also extends to filesystems that are shared from
      a [type 2 hypervisor](https://en.wikipedia.org/wiki/Hypervisor) to a guest.
        - E.g:
            - Docker Desktop for Mac [uses Hypervisor.framework](https://news.ycombinator.com/item?id=25454121) to run a
              Linux VM to host the docker container guest.
                - Sharing a filesystem path as a Docker volume from `Mac OS -> Hypervisor.framework -> Linux VM` means
                  there are two OS's that could be writing to the DB file, which breaks WAL mode and could corrupt the
                  database file.

### Testing to see if WAL mode is operating correctly.

You can see if SQLite is able to work correctly by testing to see if the `tdog` CLI pauses on DB file write lock:

```bash
# Whilst `tdog` is polling:
sqlite3 db-file.sqlite;

# This locks the file for writing; other processes must wait until this transaction is complete.
BEGIN IMMEDIATE; 

# At this point `tdog` should stop polling - check the logs to confirm.

# Release lock.
ROLLBACK;

# Confirm `tdog` continues polling from logs.
```

### Fixes

- When the target db is a SQLite file:
    - Only use Docker volumes when using type 1 hypervisors (E.g. AWS, GCP, Azure VM's).
        - Or in the case of type 2 hypervisors, only write to filesystem paths that are not shared back to the host (
          owned exclusively by the guest).
    - Use the native `tdog` CLI binary for Mac OS, Linux or Windows (no Docker container or volumes).