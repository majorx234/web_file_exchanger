# Usecase:
- ![Alt text](usecase.png?raw=true "user want to exchange files")
- file server with Webfrontend
- users can search, upload and download files over webfrontend
- users need login
- folder shares are configurated by server admin
- Configuration via ENV-Vars
- ToDO: need configuration file for Users and shared folder (still hardcoded)

# Sequencediagram:
- ![Alt text](sequence_diagram_user_interaction.png?raw=true "user login and access root directory")
- JWT Token Based authorization Process
- after login via username/password a JsonWebToken is given out to frontend
  - in all further requests (for file handling) these token is submitted in http-header
  - thus further requests are checked for authorization
- ToDo: need refresh token mechanism
# Frontend view:
- ![Alt text](frontend_view_explained.png?raw=true "html frontend with login area and file browser")
# endpoints
- login
  - request:
    - user_name
    - password_hash
  - response:
    - msg
    - token
- files
  - request:
    - cmd
    - path
  - response:
    - list of files/folders: `{ filename: "test1", is_folder: true, children: null }`

