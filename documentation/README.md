# Usecase:
- ![Alt text](usecase.png?raw=true "user want to exchange files")
# Sequencediagram:
- ![Alt text](sequence_diagram_user_interaction.png?raw=true "user login and access root directory")
# Frontend view:
- ![Alt text](frontend_view.png?raw=true "html frontend with login area and file browser")
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

