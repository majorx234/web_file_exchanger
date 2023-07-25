const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
.grid {
  display: grid;
  grid-template-rows: 150px auto auto 100px;
  grid-template-columns: repeat(10, 10%);
}

.grid > .nav {
  grid-column:1 / 11;
  grid-row:1 / 2;
}

.grid > .content {
  grid-column:4/11;
  grid-row:2 / 5;
  text-align: left;
}

.grid > .aside {
  grid-column:1 / 4;
  grid-row:2 /5;
  text-align: left;
}

.grid > .footer {
  grid-column: 1 / 11;
  grid-row:5 / 6;
}
</style>
<nav class="nav">
  <button type="button" id="get_ls_directories_button">get_ls_directory</button>
  <br></br>
  <div>
    <input name="myfile" id="upload_file_input" type="file" multiple></input>
    <button type="button" id="upload_button">upload_file</button>
  </div>
  <form action="" class="js-form" id="js-form" >
    <input type="text" class="cmd_prompt" id="cmd_prompts"></input>
  </form>
</nav>
<main class="content">
  <div id="folder_details"></div>
  </div>
</main>
<aside class="aside">
  <div id="folder_tree"></div>
</aside>
`;

function httpGet(endpoint_name, variable_context, response_handler, token) {
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);
        }
    };
    let endpoint = "http://" + location.hostname + ":8080/" + endpoint_name;
    xmlHttp.open("GET", endpoint, true);
    if (token) {
        xmlHttp.setRequestHeader('Authorization', 'Bearer ' + token);
    }
    xmlHttp.send(null);
}

function httpPost(endpoint_name, data, variable_context, response_handler, token, data_type="json") {
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);
        }
    };
    let endpoint = "http://" + location.hostname + ":8080/" + endpoint_name;
    xmlHttp.open("POST", endpoint, true);
    if (data_type == "json") {
        xmlHttp.setRequestHeader('Content-type', 'application/json');
        // if (data_type == "form_data")
        //        xmlHttp.setRequestHeader('Content-type', 'multipart/form-data');
        // browser choose it by itself
        if (token) {
            xmlHttp.setRequestHeader('Authorization', 'Bearer ' + token);
        }
    }
    xmlHttp.send(data);
}

function outputToConsole(text) {
    /*
    let para = document.createElement("p");
    let node = document.createTextNode(text);
    para.appendChild(node);
    document.getElementById("console").prepend(para);
    para.scrollIntoView();
    */
}


function httpGetLsDirectories(token) {
    let variable_context = "get ls directory: ";
    let response_handler = (response_text) => {
        // let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpGet("files", variable_context, response_handler, token);
}

function httpPostUpload(root, token){
    let end_point_name = "upload";
    let variable_context = "upload: ";
    let upload_file_input = root.querySelector("#upload_file_input");

    let fragmente = [];
    let upload_form_data = new FormData();
    for (var i = 0, f; f = upload_file_input.files[i]; i++) {
        fragmente.push('file: ' , f.name, ' type: (', f.type || 'n/a', ') - size: ', f.size, ' bytes');
        upload_form_data.append("file"+i, f);
    }
    outputToConsole(fragmente);
    let response_handler = (response_text) => {
        let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpPost(end_point_name, upload_form_data, variable_context, response_handler, token, 'form_data');
}

function httpPostCmdPrompt(cmd,path, handler_fct = null, base_tag = null, token){
    let variable_context = "post cmd: " + cmd +" path: " + path +" - ";
    let json_data = {
        cmd:cmd,
        path:path,
    };
    let json_string = JSON.stringify(json_data);
    let response_handler = (response_text) => {
        let json_data = JSON.parse(response_text);
        if (handler_fct) {
            handler_fct(json_data, base_tag);
        } else {
            outputToConsole(variable_context + response_text);
        }
    };
    httpPost("files", json_string, variable_context, response_handler, token);
}

function createHtmlFromFolderStructure(list_fs_json, fs_list_tag, path, root, token) {
    fs_list_tag.classList.add("folder");
    for (const fs_item in list_fs_json){
        let fs_item_name = list_fs_json[fs_item]["filename"];
        let fs_item_tag = document.createElement("li");
        let fs_item_label = document.createElement("label");
        fs_item_label.innerHTML = fs_item_name;

         // TODO
        // let onclick_tag_function = ...
        // fs_item_label.onlick = onclick_tag_function;
        if (list_fs_json[fs_item]["is_folder"]){
            let new_fs_list_tag = document.createElement("ul");
            let fs_item_label_onlick_fct = (event) => {};
            // fs_item_label.onclick = fs_item_label_onlick_fct;

            let fs_item_summary = document.createElement("summary");
            let details_tag  = document.createElement("details");
            let details_tag_onlick_fct = (event) => {
                let new_path = path + fs_item_name + "/";
                let list_fs_handler_function = (json_data, base_tag) => {
                    let new_fs_list_tag = document.createElement("ul");
                    new_fs_list_tag = createHtmlFromFolderStructure(json_data, new_fs_list_tag, new_path, root, token);
                    // base_tag.innerHTML = '';
                    let base_tag_children = base_tag.childNodes;
                    base_tag_children.forEach(function(item){
                        if(item.tagName != "SUMMARY"){
                            base_tag.removeChild(item);
                        }
                    });
                    let folder_browser_tag = root.querySelector("#folder_details");
                    folder_browser_tag.innerHTML = "";
                    let new_fs_list_tag2 = new_fs_list_tag.cloneNode(true);
                    folder_browser_tag.append(new_fs_list_tag2);
                    base_tag.append(new_fs_list_tag);
                };
                httpPostCmdPrompt("ls",new_path,list_fs_handler_function, details_tag, token);
            };
            // details_tag.addEventListener("toggle", details_tag_onlick_fct);
            fs_item_label.onclick = details_tag_onlick_fct;
            fs_item_summary.append(fs_item_label);
            details_tag.append(fs_item_summary);
            // details_tag.append(new_fs_list_tag);
            fs_item_tag.append(details_tag);
        } else {
            fs_item_tag.append(fs_item_label);
        }
        fs_list_tag.append(fs_item_tag);
    }
    return fs_list_tag;
}

function init_folder_structure(root, token) {
    // main part:
    // handle json: [{"filename":"README.md","is_folder":false,"children":null},...]
    let list_fs_handler_function = (list_fs_json, base_tag) => {
        let fs_list_tag = document.createElement("ul");
        fs_list_tag = createHtmlFromFolderStructure(list_fs_json, fs_list_tag, "/", root, token);
        base_tag.append(fs_list_tag);
    };

    let base_tag = root.querySelector("#folder_tree");
    httpPostCmdPrompt("ls","/",list_fs_handler_function, base_tag, token);
}

class FileBrowserComponent extends HTMLElement {
    static get observedAttributes() { return ['token']; }
    constructor() {
        super();
        this._token = null;
        this.root = this.attachShadow({mode: "closed"});
        this.root.appendChild(template.content.cloneNode(true));
    }

    connectedCallback() {
        let token = this._token;
        this.root.querySelector("#upload_button").onclick = function() {
            httpPostUpload(this.root, token);
        };

        this.root.querySelector("#get_ls_directories_button").onclick = function() {
            httpGetLsDirectories(token);
        };
        this.root.querySelector("#js-form").addEventListener('submit', e => {
            e.preventDefault();
            let command_line = this.root.querySelector("#cmd_prompts").value.split(" ", 2);
            if (command_line.length == 2) {
                httpPostCmdPrompt(command_line[0],command_line[1], null, this.token);
            } else {
                outputToConsole("error command hav to be 2 words");
            }
            return false;
        });
        init_folder_structure(this.root, token);
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "token"){
            if (oldValue !== newValue) {
                this._token = newValue;
            }
        }
        console.log(name, oldValue, newValue);
    }
    /* getter setter */
    get token() {
        return this.getAttribute("token");
    }
    set token(val) {
        this.setAttribute("token", val);
        this._token = val;
    }
}

customElements.define("file-browser-component", FileBrowserComponent);
