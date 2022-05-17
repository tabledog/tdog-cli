

# Connecting to a local Docker Postgre sever with SSL enabled

- The Debian docker image comes installed with:
    - `/etc/ssl/certs/ssl-cert-snakeoil.pem`
    - `/etc/ssl/private/ssl-cert-snakeoil.key`
    - Version = `06d6f5bd.0 -> ssl-cert-snakeoil.pem`
    - Copy these to the client and add to the Postgres connection as a root CA (do not add to system wide).
    
- `.pem` can be converted to `.crt`:
    - `openssl crl2pkcs7 -nocrl -certfile cert.pem | openssl pkcs7 -print_certs -out cert.crt`
    - `.crt` is the format Supabase uses.
        - Note: In some cases its just the file extension that differs?
    - See:
        - https://stackoverflow.com/questions/13732826/convert-pem-to-crt-and-key
        - https://gist.github.com/mrw34/c97bb03ea1054afb551886ffc8b63c3b#gistcomment-3602247 
          
    

# Connecting to Postgres server as a client via TLS

- `PGPASSWORD=postgres psql "port=5432 host=localhost user=postgres sslrootcert=./ssl-cert-snakeoil.crt sslmode=require"`
    - Supabase (and other SSL enabled Postgres cloud databases) will provide a `root-ca.crt` file.
    - This is a "Root Certificate Authority (CA)" that can be installed to the OS **to avoid certificate-invalid errors when connecting**.
        - This enables the DB provider to self-sign a unique server certificate per database instance.
            - The `root-ca.crt` itself is not signed/invalid.
            - This is the **server Root Certificate Authority**.
                - Not to be mixed up with the **client** private/pub keys which some PG clients optionally take.
    

# Issues with the `postgres` Rust crate with TLS connections: 


- https://github.com/sfackler/rust-native-tls/issues/210
    - Issue about using `openssl` vs `native-tls` Rust API's with self signed `ca.crt`
        - Summary:
            - `native-tls` on Mac OS has stricter rules the `openssl` alternative.
                - Must add to keychain with "Always Trust".
                - `.danger_accept_invalid_hostnames(true)` must be set for some certs (`SSLSetPeerDomainName` is the function that fails if this is false: https://developer.apple.com/documentation/security/1393047-sslsetpeerdomainname?language=objc).
                - Certs must be valid for less than 825 days.
                - macOS Console app can show specific reason for cert rejection.
                - A fix for ignoring cert errors is to add both the CA and the server cert to keychain with Always Trust.
                    - Server cert can be downloaded via `openssl s_client` and copy/pasted to a `.pem` file.
            - `openssl` seems easier to use as:
                - It mimics the logic used in other clients (CLI, GUI).
                - It works the same across different OS's.
                - It is far less strict when verifying certificates than macOS.
                - `native-tls` falls back to using it anyway, why not just start with that and only have one set of logic for all builds.


- https://wiki.openssl.org/index.php/Hostname_validation

- https://serverfault.com/questions/79876/connecting-to-postgresql-with-ssl-using-openssl-s-client
    - `echo "" | /opt/homebrew/opt/openssl@1.1/bin/openssl s_client -starttls postgres -connect db.otfzdikncxtokgnkhsut.supabase.co:5432 -showcerts -CAfile /Users/enzo/Downloads/prod-ca-2021.crt`
    
    - `echo "" | /opt/homebrew/opt/openssl@3/bin/openssl s_client -starttls postgres -connect db.otfzdikncxtokgnkhsut.supabase.co:5432 -servername db.otfzdikncxtokgnkhsut.supabase.co -showcerts -CAfile /Users/enzo/Downloads/prod-ca-2021.crt -verify_hostname db.otfzdikncxtokgnkhsut.supabase.co -policy_check -policy_print -verify_return_error`
    - `openssl version` must be > 1.1.1
    - Args: `/opt/homebrew/opt/openssl@3/bin/openssl s_client -help`
    
    -  PGPASSWORD=test-db-a-pass psql "port=5432 host=db.otfzdikncxtokgnkhsut.supabase.co user=postgres sslrootcert=/Users/enzo/Downloads/prod-ca-2021.crt sslmode=verify-full"
    

- https://unix.stackexchange.com/questions/128370/linux-openssl-cn-hostname-verification-against-ssl-certificate
- https://github.com/sfackler/rust-native-tls/commit/98f459c1dbc85cc933cf126b327cdab60cf05f3a
- https://github.com/sfackler/rust-native-tls/issues/13
- https://github.com/sfackler/rust-native-tls/pull/15
- https://developer.apple.com/documentation/security/1393047-sslsetpeerdomainname?language=objc


## Issue: Error `IP address mismatch`.
- This happens when the server IP does not match the CA cert IP.
    - E.g. when using the `snakeoil` certs.
    - The cert DNS name is stored here: `Extension: Subject Alternative Name, DNS Name: X`

- To replicate this error:
    - echo "" | /opt/homebrew/opt/openssl@1.1/bin/openssl s_client -starttls postgres -connect 127.0.0.1:5432 -showcerts -CAfile ./tls/ssl-cert-snakeoil.pem -verify_ip 127.0.0.1

- Fix:
    - 1. Add `127.0.0.1 08e59019d71f` to `/etc/hosts`.
    - 2. Set `08e59019d71f` as the IP in the Postgres connection.
    
- I think the Supabase CA cert fails because the Common Name is not a domain name (incorrectly filled out).
    - This fails on macOS, but openssl seems to ignore this.
