import {httpPost, httpGet} from "./http_operation.js";
//import {forge_sha256} from "./forge-sha256.min.js"

const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
</style>
<div class="login">
  <input type="text" id="user_name_input">
  <input type="text" id="password_input">
  <button type="button" id="login_button">login</button>
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
        httpPost(end_point_name, json_string, variable_context, response_handler, null, 'json');
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
