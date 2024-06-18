# rust_tle_parser

A TLE fetch and parse utility built in rust. Data pulled from Celestrak.

## TODO
- Use serde to implement json serialisation method on TLE struct.
- Implement better interface with Celestrak for getting and caching TLE's
    - Allow getting individual satellite info via SupGP backup of CurrentGP
- Tidy up and convert to proper library
- Correct documentation etc.