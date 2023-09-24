const template = document.createElement("template");
template.innerHTML = /*html*/ `
<style>
</style>
<div id="content"></div>
</div>
`;

class SearchResultComponent extends HTMLElement {
    static get observedAttributes() { return ['token']; }
    constructor() {
        super();
        this.search_result = null;
        this.folder_path = "/";
        this.root = this.attachShadow({mode: "closed"});
        this.root.appendChild(template.content.cloneNode(true));
    }

    logEvent(log_msg) {
        this.dispatchEvent(new CustomEvent("log-event",{detail : log_msg} ));
    }

    selectEvent(result_index) {
        this.dispatchEvent(new CustomEvent("select-event",{detail : {index: result_index}}));
    }

    connectedCallback() {
        let result_tag = document.createElement("div");
        result_tag.innerHTML = "nothing found";
        if (this.search_result.length > 0) {
            let result_tag = document.createElement("ul");
            for (const result_index in this.search_result) {
                let result_item = document.createElement("li");
                result_item.innerHTML = this.search_result[result_index];
                result_tag.append(result_item);
            }
        }
    }

    getSearchResults() {
        this.search_result = JSON.parse(this.getAttribute("search_result"));
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "search_result"){
            if (oldValue !== newValue) {
                this.search_result = JSON.parse(newValue);
            }
        }
    }
}

customElements.define("search-result-component", SearchResultComponent);
