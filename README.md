# Info
- Webserver to help exchanging files
- WIP

# Usage
- not usable yet
- `source server_conf.sh`
- `cargo run`

# Config
- through ENVVARS:
  - `HOST_IP` - ip
  - `PORT` - port
  - `DATABSE_URL` - not used
  - `FRONTEND_DIR` - directory of web application
  - `FILE_STORE_DIR` - directories of file store
  - `JWT_SECRET` - password to sign tokens
  - `JWT_EXPIRE_TIME` - expiretime of token

# ToDo
- login System for users
- add Endpoints for:
  - list dir
  - dir navigation
  - file download
  - file upload

# References
- uses forge sha256 implementation: https://github.com/brillout/forge-sha256/
