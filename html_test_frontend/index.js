var token = null

function outputToConsole(text) {
    let para = document.createElement("p");
    let node = document.createTextNode(text);
    para.appendChild(node);
    document.getElementById("console").prepend(para);
    para.scrollIntoView();
}

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

function httpGetLsDirectories() {
    let variable_context = "get ls directory: ";
    let response_handler = (response_text) => {
        // let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpGet("files", variable_context, response_handler, token);
}


function httpPostLogin() {
    let end_point_name = "login";
    let variable_context = "login: ";
    let user_name = document.getElementById("user_name_input").value;
    let password = document.getElementById("password_input").value;
    let password_hash = forge_sha256(password + "salt29562");
    let json_data = {
        user_name: user_name,
        password_hash: password_hash
    };
    let param_name = "login";
    let response_handler = (response_text) => {
        let json_data = JSON.parse(response_text);
        token = json_data["token"];
        outputToConsole(variable_context + response_text);
        init_folder_structure();
    };
    let json_string = JSON.stringify(json_data);
    httpPost(end_point_name, json_string, variable_context, response_handler, null, 'json');
}

function httpPostUpload(){
    let end_point_name = "upload";
    let variable_context = "upload: ";
    let upload_file_input = document.getElementById("upload_file_input");

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

function httpPostCmdPrompt(cmd,path, handler_fct = null, base_tag = null){
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

function clearConsole() {
    document.getElementById("console").innerHTML = "";
}

function init_folder_structure() {
    // main part:
    // handle json: [{"filename":"README.md","is_folder":false,"children":null},...]
    let list_fs_handler_function = (list_fs_json,base_tag) => {
        let fs_list_tag = document.createElement("ul");
        fs_list_tag.classList.add("folder");
        for (fs_item in list_fs_json){
            let fs_item_tag = document.createElement("li");
            let fs_item_summary = document.createElement("summary");
            let fs_item_label = document.createElement("label");
            fs_item_label.innerHTML = list_fs_json[fs_item]["filename"];
            // TODO
            // let onclick_tag_function = ...
            // fs_item_label.onlick = onclick_tag_function;
            fs_item_summary.append(fs_item_label);
            fs_item_tag.append(fs_item_summary);
            fs_list_tag.append(fs_item_tag);
        }
        base_tag.append(fs_list_tag);
    };

    let base_tag = document.getElementById("folder_tree");
    httpPostCmdPrompt("ls","/",list_fs_handler_function, base_tag);
}

document.getElementById("js-form").addEventListener('submit', e => {
    e.preventDefault();
    let command_line = document.getElementById("cmd_prompts").value.split(" ", 2);
    if (command_line.length == 2) {
        httpPostCmdPrompt(command_line[0],command_line[1], null);
        } else {
            outputToConsole("error command hav to be 2 words");
        }
    return false;
});

document.getElementById("clear_button").onclick = function() {
    clearConsole();
};

document.getElementById("get_test_button").onclick = function() {
    httpGetTest();
};

document.getElementById("get_info_button").onclick = function() {
    httpGetInfo();
};

document.getElementById("get_ls_directories_button").onclick = function() {
    httpGetLsDirectories();
};

document.getElementById("login_button").onclick = function() {
    httpPostLogin();
};

document.getElementById("upload_button").onclick = function() {
    httpPostUpload();
}
outputToConsole("init successful");
