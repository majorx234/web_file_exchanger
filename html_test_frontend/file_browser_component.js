import {downloadFile, httpPostFetch} from "./http_operation.js";

const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
:host {
  display: grid;
  grid-template-rows: 150px auto auto 100px;
  grid-template-columns: repeat(10, 10%);
}

:host > .nav {
  grid-column:1 / 11;
  grid-row:1 / 2;
}

:host > .content {
  grid-column:4/11;
  grid-row:2 / 5;
  text-align: left;
}

:host > .aside {
  grid-column:1 / 4;
  grid-row:2 /5;
  text-align: left;
}

:host > .footer {
  grid-column: 1 / 11;
  grid-row:5 / 6;
}
</style>
<nav class="nav">
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
<aside class="aside" id="folder_tree">
</aside>
`;

class FileBrowserComponent extends HTMLElement {
    static get observedAttributes() { return ['token']; }
    constructor() {
        super();
        this._token = null;
        this.folder_path = "/";
        this.root = this.attachShadow({mode: "closed"});
        this.root.appendChild(template.content.cloneNode(true));
    }

    logEvent(log_msg) {
        this.dispatchEvent(new CustomEvent("log-event",{detail : log_msg} ));
    }

    httpPostUpload() {
        let end_point_name = "upload";
        let variable_context = "upload: ";
        let upload_file_input = this.root.querySelector("#upload_file_input");

        let fragmente = [];
        let upload_form_data = new FormData();
        for (var i = 0, f; f = upload_file_input.files[i]; i++) {
            fragmente.push('file: ' +  this.folder_path + f.name + ' type: (' + f.type + ') - size: ' + f.size + ' bytes');
            let file_name_with_path = this.folder_path + f.name;
            upload_form_data.append("file"+i, f, file_name_with_path);
        }
        this.logEvent(fragmente);
        let response_handler = (response_text) => {
            let json_data = JSON.parse(response_text);
            this.logEvent(variable_context + response_text);
        };
        httpPostFetch(end_point_name, upload_form_data, variable_context, response_handler, this._token, 'form_data');
    }

    httpPostCmdPrompt(cmd,path, handler_fct = null, base_tag = null){
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
                this.logEvent(variable_context + response_text);
            }
        };
        httpPostFetch("files", json_string, variable_context, response_handler, this._token);
    }

    init_folder_structure() {
        // main part:
        // handle json: [{"filename":"README.md","is_folder":false,"children":null},...]
        let root_ul_tag = document.createElement("ul");
        root_ul_tag.classList.add("folder");
        let root_item_tag = document.createElement("li");
        let root_item_label = document.createElement("label");
        root_item_label.innerHTML = "/";
        let root_details_tag  = document.createElement("details");
        root_item_label.onclick = (event) => {
            let new_path = "/";
            let list_fs_handler_function = (json_data, base_tag) => {
                this.createFolderDetails(json_data, base_tag, new_path);
            };
            this.httpPostCmdPrompt("ls",new_path,list_fs_handler_function, root_details_tag);
        };
        let root_item_summary = document.createElement("summary");
        root_item_summary.append(root_item_label);
        root_details_tag.append(root_item_summary);
        root_item_tag.append(root_details_tag);
        let base_tag = this.root.querySelector("#folder_tree");
        root_ul_tag.append(root_item_tag);
        base_tag.append(root_ul_tag);
    }

    createFolderDetails(json_data, base_tag, new_path) {
        let [new_fs_tree_list_tag, new_fs_details_list_tag] = this.createHtmlFromFolderStructure(json_data, new_path);
        let new_fs_list_tag2 = new_fs_details_list_tag;
        this.folder_path = new_path;
        let base_tag_children = base_tag.childNodes;
        base_tag_children.forEach(function(item){
            if(item.tagName != "SUMMARY"){
                base_tag.removeChild(item);
            }
        });
        let folder_browser_tag = this.root.querySelector("#folder_details");
        folder_browser_tag.innerHTML = "";
        folder_browser_tag.append(new_fs_list_tag2);
        base_tag.append(new_fs_tree_list_tag);
    }

    createHtmlFromFolderStructure(list_fs_json, path) {
        let fs_tree_list_tag = document.createElement("ul");
        let fs_details_list_tag = document.createElement("ul");
        fs_tree_list_tag.classList.add("folder");
        fs_details_list_tag.classList.add("folder_details");
        for (const fs_item in list_fs_json){
            let fs_item_name = list_fs_json[fs_item]["filename"];
            let fs_item_tag = document.createElement("li");
            let fs_item_label = document.createElement("label");
            fs_item_label.innerHTML = fs_item_name;

            if (list_fs_json[fs_item]["is_folder"]){
                let fs_item_label_onlick_fct = (event) => {};
                // fs_item_label.onclick = fs_item_label_onlick_fct;

                let fs_item_summary = document.createElement("summary");
                let details_tag  = document.createElement("details");
                let details_tag_onlick_fct = (event) => {
                    let new_path = path + fs_item_name + "/";
                    let list_fs_handler_function = (json_data, base_tag) => {
                        this.createFolderDetails(json_data, base_tag, new_path);
                    };
                    this.httpPostCmdPrompt("ls",new_path,list_fs_handler_function, details_tag);
                };
                // details_tag.addEventListener("toggle", details_tag_onlick_fct);
                fs_item_label.onclick = details_tag_onlick_fct;
                let fs_item_label2 = fs_item_label.cloneNode(true);
                fs_item_label2.onclick = details_tag_onlick_fct;
                fs_item_summary.append(fs_item_label);
                details_tag.append(fs_item_summary);
                fs_item_tag.append(details_tag);
                fs_tree_list_tag.append(fs_item_tag);
                let fs_details_item = document.createElement("li");
                fs_details_item.append(fs_item_label2);
                fs_details_item.classList.add("folder_item");
                fs_details_list_tag.append(fs_details_item);
            } else {
                fs_item_label.onclick = () => {
                    let new_path = 'files' + path + fs_item_name;
                    downloadFile(new_path, fs_item_name, this._token);
                };
                fs_item_tag.append(fs_item_label);
                fs_item_tag.classList.add("file_item");
                fs_details_list_tag.append(fs_item_tag);
            }
        }
        return [fs_tree_list_tag, fs_details_list_tag];
    }

    connectedCallback() {
        let token = this._token;
        this.root.querySelector("#upload_button").onclick = () => {
            this.httpPostUpload();
        };

        this.root.querySelector("#js-form").addEventListener('submit', e => {
            e.preventDefault();
            let command_line = this.root.querySelector("#cmd_prompts").value.split(" ", 2);
            if (command_line.length == 2) {
                this.httpPostCmdPrompt(command_line[0],command_line[1], null, null);
            } else {
                this.logEvent("error command hav to be 2 words");
            }
            return false;
        });
        this.init_folder_structure(this.root, token);
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "token"){
            if (oldValue !== newValue) {
                this._token = newValue;
            }
        }
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
