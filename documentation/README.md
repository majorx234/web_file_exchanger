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

