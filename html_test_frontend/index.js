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

function httpPost(endpoint_name, json_data, variable_context, response_handler, token) {
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            response_handler(this.responseText);
        }
    };
    let endpoint = "http://" + location.hostname + ":8080/" + endpoint_name;
    xmlHttp.open("POST", endpoint, true);
    xmlHttp.setRequestHeader('Content-type', 'application/json');
		if (token) {
				xmlHttp.setRequestHeader('Authorization', 'Bearer ' + token);
		}
    let json_string = JSON.stringify(json_data);
    xmlHttp.send(json_string);
}

function httpGetTest() {
    let variable_context = "get test: ";
    let response_handler = (response_text) => {
        // let json_data = JSON.parse(response_text);
        outputToConsole(variable_context + response_text);
    };
    httpGet("hello", variable_context, response_handler, token);
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
        outputToConsole(variable_context + response_text);
    };
    httpPost(end_point_name, json_data, variable_context, response_handler);
}

function clearConsole() {
    document.getElementById("console").innerHTML = "";
}

document.getElementById("clear_button").onclick = function() {
    clearConsole();
};

document.getElementById("get_test_button").onclick = function() {
    httpGetTest();
};

document.getElementById("login_button").onclick = function() {
    httpPostLogin();
};

outputToConsole("init successful");
