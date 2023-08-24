import {httpGet} from "./http_operation.js";
import "./file_browser_component.js";
import "./login_component.js";

var token = null

function outputToConsole(text) {
    let para = document.createElement("p");
    let node = document.createTextNode(text);
    para.appendChild(node);
    document.getElementById("console").prepend(para);
    para.scrollIntoView();
}

function httpGetTest() {
    let variable_context = "get test: ";
    let response_handler = (response_text) => {
        // let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpGet("hello", variable_context, response_handler, token);
}

function httpGetInfo() {
    let variable_context = "get info: ";
    let response_handler = (response_text) => {
        // let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpGet("info", variable_context, response_handler, token);
}

function clearConsole() {
    document.getElementById("console").innerHTML = "";
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

document.getElementById("clear_button").onclick = function() {
    clearConsole();
};

document.getElementById("get_test_button").onclick = function() {
    httpGetTest();
};

document.getElementById("get_info_button").onclick = function() {
    httpGetInfo();
};

create_login();
outputToConsole("init successful");
