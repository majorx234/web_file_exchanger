const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
</style>
<div>
  <button type="button" id="clear_button">clear</button>
  <div class="console" id="console"></div>
</div>
`;

class ConsoleComponent extends HTMLElement {
    static get observedAttributes() { return ['print']; }
    constructor() {
        super();
        this._token = null;
        this.root = this.attachShadow({mode: "closed"});
        this.root.appendChild(template.content.cloneNode(true));
    }

    outputToConsole(text) {
        let para = document.createElement("p");
        let node = document.createTextNode(text);
        para.appendChild(node);
        this.root.querySelector("#console").prepend(para);
    }

    connectedCallback() {
        this.root.querySelector("#clear_button").onclick = () => {
            this.root.querySelector("#console").innerHTML = "";
        };
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "print"){
            let json_data = JSON.parse(newValue);
            this.outputToConsole(json_data.msg);
        }
    }

    /* getter setter */
    get print() {
        return this.getAttribute("print");
    }

    set print(val) {
        this.setAttribute("print", val);
    }
}

customElements.define("console-component", ConsoleComponent);
