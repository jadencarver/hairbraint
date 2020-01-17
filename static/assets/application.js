var ws = new WebSocket("ws://localhost:9002");
var body = document.getElementById('body');
window.onhashchange = push_state;

ws.onerror = function(error) {
    console.log('ws error',error);
};
ws.onmessage = function(message) {
    var parser = new DOMParser();
    var xml = parser.parseFromString(message.data, "text/xml");
    document.getElementById("date").value = xml.children[0].getElementsByTagName("date")[0].textContent

    var xslt = new XSLTProcessor();
    var xhttp = new XMLHttpRequest();
    xhttp.open("GET", "/xslt/schedule.xml", false);
    xhttp.send(null);
    var scheduleXSLT = xhttp.responseXML;
    xslt.importStylesheet(scheduleXSLT);

    var result = xslt.transformToFragment(xml, document);
    body.replaceWith(result);
    window.body = result;
};
ws.onclose = function(event) {
    console.log('close',event);
};

function generate_state() {
    var parser = new DOMParser();
    var xml = parser.parseFromString("<state></state>", "text/xml");
    var state = xml.children[0];

    var date = xml.createElement("date");
    date.appendChild(xml.createTextNode(document.getElementById("date").value));
    state.appendChild(date);

    if (document.location.hash) {
        var anchor = xml.createElement("anchor");
        anchor.appendChild(xml.createTextNode(document.location.hash));
    }

    return state;
}

function push_state() {
    console.log("Pushing State");
    var serializer = new XMLSerializer();
    var state = generate_state();
    ws.send(serializer.serializeToString(state));
}
