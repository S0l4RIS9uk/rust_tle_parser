# rust_tle_parser

A TLE fetch and parse program built in rust using data pulled from Celestrak.

## TODO
- Implement better interface with Celestrak for getting and caching TLE's
    - Allow getting individual satellite info via SupGP backup of CurrentGP
    - Implement a cache using norad currentGP to initalise and supGP to maintain/fetch more up to date data.
- Tidy up and convert to proper library
- Correct documentation etc.
