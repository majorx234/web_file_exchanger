import {httpGet} from "./http_operation.js";
import "./file_browser_component.js";
import "./login_component.js";
import "./console_component.js";

var token = null

function outputToConsole(text) {
    let json_data = { msg : text };
    document.getElementById("console").print = JSON.stringify(json_data);
}

function create_login() {
    let header_tag = document.getElementById("header");
    let login_component_tag = document.createElement("login-component");
    login_component_tag.addEventListener("log-event",(event) => {
            outputToConsole(event.detail);
        });
    login_component_tag.addEventListener("token-event", (token_event) => {
        // init_folder_structure();
        let content_tag = document.getElementById("content");
        let file_browser_tag = document.createElement("file-browser-component");
        file_browser_tag.token = token_event.detail;
        token = token_event.detail;
        content_tag.innerHTML = "";
        content_tag.append(file_browser_tag);
        file_browser_tag.addEventListener("log-event",(event) => {
            outputToConsole(event.detail);
        });

    });
    header_tag.append(login_component_tag);
}

function create_console() {
    let footer_tag = document.getElementById("footer");
    let consol_component_tag = document.createElement("console-component");
    consol_component_tag.setAttribute("id","console");
    footer_tag.append(consol_component_tag);
}

create_login();
create_console();
outputToConsole("init successful");
