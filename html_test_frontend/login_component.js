import {httpPost, httpGet, httpPostFetch} from "./http_operation.js";
import "./forge-sha256.min.js"
import * as bootstrap from "./bootstrap-5.3.1-dist/bootstrap.min.js"

const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
</style>
<meta name="viewport" content="width=device-width, initial-scale=1">
<link href="./bootstrap-5.3.1-dist/bootstrap.min.css" rel="stylesheet">

<div class="d-flex flex-row align-items-center" data-bs-theme="light">
    <label class="form-label" for="user">User Name:</label>
    <input type="text" id="user_name_input" class="form-control form-control-sm" placeholder="User">
    <label class="form-label" for="password">Password:</label>
    <input type="password" class="form-control form-control-sm" id="password_input" placeholder="Enter password">
    <button type="button" class="btn btn-primary" id="login_button">login</button>
</div>
`;

class LoginComponent extends HTMLElement {
    static get observedAttributes() { return ['token']; }
    constructor() {
        super();
        this._token = null;
        this.root = this.attachShadow({mode: "closed"});
        this.root.appendChild(template.content.cloneNode(true));
        this.root.querySelector("#login_button").onclick = () => {
            this.httpPostLogin();
        };
    }

    logEvent(log_msg) {
        this.dispatchEvent(new CustomEvent("log-event",{detail : log_msg} ));
    }

    tokenEvent(token_msg) {
        this.dispatchEvent(new CustomEvent("token-event",{detail : token_msg} ));
    }

    httpPostLogin() {
        let end_point_name = "login";
        let variable_context = "login: ";
        let user_name = this.root.querySelector("#user_name_input").value;
        let password = this.root.querySelector("#password_input").value;
        let password_hash = forge_sha256(password + "salt29562");
        let json_data = {
            user_name: user_name,
            password_hash: password_hash
        };
        let response_handler = (response_text) => {
            let json_data = JSON.parse(response_text);
            let token = json_data["token"];
            this.tokenEvent(token);
            this.logEvent(variable_context + response_text);
        };
        let json_string = JSON.stringify(json_data);
        httpPostFetch(end_point_name, json_string, variable_context, response_handler, null, 'json');
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

customElements.define("login-component", LoginComponent);
