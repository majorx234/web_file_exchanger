# Info
- webserver to uploade/download files
- idea is a simple file exchange with user/password security
- server can provide shared folder over website
- webusers can up download files and folders
- webusers can search for files

# more documentation:
[link](documentation/README.md)

# Build
- `cargo build`
- web frontend is written in vanilla-js, no build is needed

# Usage
- in alpha stadium
- `source server_conf.sh`
- `cargo run`

# Config
- through ENV_VARS:
  - `HOST_IP` - ip
  - `PORT` - port
  - `DATABSE_URL` - not used yet
  - `FRONTEND_DIR` - directory of web application
  - `FILE_STORE_DIR` - directories of file store
  - `JWT_SECRET` - password to sign tokens
  - `JWT_EXPIRE_TIME` - expiretime of token

# Development
- rebuild backend when updating source file:
  - `cargo watch -q -c -w src/ -x run`
- run automated testing:
  - `cargo watch -q -c -w tests/ -x "test -- --nocapture"`

# ToDo
- prepare security for pentesting
  - own repo: https://github.com/majorx234/web_file_exchanger
- simplify files endpoint
- append additional folder to file index
- extra file indexer, to create file_index beforehand
  - start server afterwards with path to fileindex
- login System for users via SQL database
  - OAuth2 support
- put frontend login in own Web Component
- Add drag & drop support and MIME features
- improve searching folder
  - search for files & folders
  - interacting with result(onclick)

# License
- this software is distributed under GPLv3 ( see LICENSE)
- use this software on your own risk, no warranties is given

# History
- 2023-09-28 - v0.3.0 Alpha Version with searching
- 2023-08-06 - v0.2.0 Alpha Version with support of up-/downloading

# References
- uses forge sha256 implementation: https://github.com/brillout/forge-sha256/
- thx to Jeremey Chone for his Axum course:
  - https://www.youtube.com/watch?v=XZtlD_m59sM
  - https://github.com/jeremychone-channel/rust-axum-course
- thx to Kate Morley for her CSS Tutorial of tree views on lists
  - https://iamkate.com/code/tree-views/
- project make use of ![Alt text](http://vanilla-js.com/assets/button.png "vanilla-js")
