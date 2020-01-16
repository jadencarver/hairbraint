/* menuitem polyfill */
(function(document) {
    var addRule = (function (style) {
        var sheet = document.head.appendChild(style).sheet;
        return function (selector, css) {
            var propText = typeof css === "string" ? css : Object.keys(css).map(function (p) {
                return p + ":" + (p === "content" ? "'" + css[p] + "'" : css[p]);
            }).join(";");
            sheet.insertRule(selector + "{" + propText + "}", sheet.cssRules.length);
        };
    })(document.createElement("style"));
    var list = document.querySelectorAll("menuitem[icon]");

    for (var i = 0; i < list.length; i++) {
        list[i].classList.add('_icon'+i);
        var url = list[i].getAttribute('icon');
        addRule('._icon'+i+'::before', {
            "background-image": "url('" + url + "')"
        });
    };

    var body = document.getElementById("body");
    var commands = document.querySelectorAll("menuitem[command]");
    for (var i = 0; i < list.length; i++) {
        var command = list[i].getAttribute('command');
        list[i].addEventListener('click', (function(command) {
            return function(event) {
                if (typeof window[command] === "function") {
                    window[command]();
                } else {
                    console.error("Invoked invalid command " + command);
                }
            };
        })(command));
    };
})(document);
