**This project is intended as a http server to run on my laptop**
# http-server
## running
**Setup the database and tables beforehand, i currently don't have it automated**
```bash
$ nix run .#release # I normally use recommend the -Lvv 
# flag while developing for ease of use 
# (it enabled build logs and verbose output)
# -------------------------------------------------------
# or 'cargo run' if you don't want to use nix
```
## Variables
**At the moment environment variable are used if they are found, else the defaults in src/environment.rs are used**
***CLI argument's haven't been implemented yet***
```bash
$MARIAHOST # is for the mariadb server hostname/ip
$MARIAUSER # is for the mariadb username
$MARIAPASS # is for the password for the username
$MARIADATABASE # is for the mariadb database to use

# IP is currently auto-assigned
```

## Current status
 - [x] a basic rate limit (src/security.rs)
 - [x] basic IP logging with unix timestamp
 - [ ] tests (src/tests.rs)
 - [ ] root and api endpoints (src/main.rs | src/endpoints/api.rs)

---
# Project is licensed under Apache License 2.0 currently
