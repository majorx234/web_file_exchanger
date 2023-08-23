export function httpGet(endpoint_name, variable_context, response_handler, token) {
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

export function httpPost(endpoint_name, data, variable_context, response_handler, token, data_type="json") {
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
    }
    if (token) {
        xmlHttp.setRequestHeader('Authorization', 'Bearer ' + token);
    }
    xmlHttp.send(data);
}

export function httpPostfetch(endpoint_name, data, variable_context, response_handler, token, data_type="json"){
    let endpoint = "http://" + location.hostname + ":8080/" + endpoint_name;
    fetch(endpoint, {
        method: "POST",
        body: JSON.stringify(data),
        headers: {
           Accept: "application/json",
           "Content-Type": "application/json",
           Authorization: 'Bearer ' + token,
           "User-Agent": "any-name"
        }
    })
    .then(response => {
        return response_handler(response.text());
    });
}

export function downloadFile(path, fileName, token){
    let url = "http://" + location.hostname + ":8080/" + path;
    fetch(url, { headers: {Authorization: 'Bearer ' + token, method: 'get', mode: 'no-cors', referrerPolicy: 'no-referrer' }})
        .then(res => res.blob())
        .then(res => {
            const aElement = document.createElement('a');
            aElement.setAttribute('download', fileName);
            const href = URL.createObjectURL(res);
            aElement.href = href;
            // aElement.setAttribute('href', href);
            aElement.setAttribute('target', '_blank');
            aElement.click();
            URL.revokeObjectURL(href);
        });
};
