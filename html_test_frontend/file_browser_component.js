import {downloadFile, httpPostFetch} from "./http_operation.js";
import * as bootstrap from "./bootstrap-5.3.1-dist/bootstrap.min.js"

const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
:host {
  display: grid;
  grid-template-rows: 100px auto auto auto auto auto 100px;
  grid-template-columns: repeat(10, 10%);
}

:host > .nav {
  grid-column:1 / 11;
  grid-row:1 / 2;
}

:host > .content {
  grid-column:4/11;
  grid-row:2 / 7;
  text-align: left;
}

:host > .aside {
  grid-column:1 / 4;
  grid-row:2 /7;
  text-align: left;
}

:host > .footer {
  grid-column: 1 / 11;
  grid-row:7 / 8;
}

.tree{
  --spacing : 1.5rem;
  --radius  : 10px;
}

.tree li{
  display      : block;
  position     : relative;
  padding-left : calc(2 * var(--spacing) - var(--radius) - 2px);
}

.tree ul{
  margin-left  : calc(var(--radius) - var(--spacing));
  padding-left : 0;
}

.tree ul li{
  border-left : 2px solid #ddd;
}

.tree ul li:last-child{
  border-color : transparent;
}

.tree ul li::before{
  content      : '';
  display      : block;
  position     : absolute;
  top          : calc(var(--spacing) / -2);
  left         : -2px;
  width        : calc(var(--spacing) + 2px);
  height       : calc(var(--spacing) + 1px);
  border       : solid #ddd;
  border-width : 0 0 2px 2px;
}

.tree summary{
  display : block;
  cursor  : pointer;
}

.tree summary::marker,
.tree summary::-webkit-details-marker{
  display : none;
}

.tree summary:focus{
  outline : none;
}

.tree summary:focus-visible{
  outline : 1px dotted #000;
}

.tree li::after,
.tree summary::before{
  content       : '';
  display       : block;
  position      : absolute;
  top           : calc(var(--spacing) / 2 - var(--radius));
  left          : calc(var(--spacing) - var(--radius) - 1px);
  width         : calc(2 * var(--radius));
  height        : calc(2 * var(--radius));
  border-radius : 50%;
  background    : #ddd;
}

.tree summary::before{
  z-index    : 1;
  background : #0D6EFD url('expand-collapse.svg') 0 0;
}

.tree details[open] > summary::before{
  background-position : calc(-2 * var(--radius)) 0;
}

#folder_details ul li {
    list-style-type: none;
}

</style>
<meta name="viewport" content="width=device-width, initial-scale=1">
<link href="./bootstrap-5.3.1-dist/bootstrap.min.css" rel="stylesheet">
<nav class="nav">
  <br></br>
  <div class="d-flex flex-row align-items-center" data-bs-theme="light">
    <label for="upload_file_input" class="form-label">Multiple files input example</label>
    <input class="form-control form-control-sm" type="file" id="upload_file_input" multiple />
    <button type="button" class="btn btn-primary" id="upload_button">upload_file</button>
  </div>
</nav>
<main class="content" id="content">
  <div id="folder_details"></div>
  </div>
</main>
<aside class="aside tree" id="folder_tree">
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
        fs_details_list_tag.classList.add("folder_items");
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

        this.init_folder_structure(this.root, token);

        let dropContainer = this.root.querySelector("#content");
        // dragover and dragenter events need to have 'preventDefault' called
        // in order for the 'drop' event to register.
        // See: https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Drag_operations#droptargets
        dropContainer.ondragover = dropContainer.ondragenter = (evt) => {
            evt.preventDefault();
        };

        dropContainer.ondrop = (evt) => {
            let fileInput = this.root.querySelector("#upload_file_input");
            fileInput.files = evt.dataTransfer.files;

            // If you want to use some of the dropped files
            const dT = new DataTransfer();
            for( let file_idx = 0; file_idx < evt.dataTransfer.files.length;file_idx++) {
                dT.items.add(evt.dataTransfer.files[file_idx]);
            }
            fileInput.files = dT.files;

            evt.preventDefault();
        };
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
