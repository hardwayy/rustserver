"use strict";
var _test = _interopRequireDefault(require("./test.js"));
function _interopRequireDefault(e) { return e && e.__esModule ? e : { "default": e }; }
function Index() {
  return /*#__PURE__*/React.createElement(React.Fragment, null, /*#__PURE__*/React.createElement("h1", null, "Test server"), /*#__PURE__*/React.createElement("p", null, "This local server is hosted in Rust and the pages are written in JSX"), /*#__PURE__*/React.createElement(_test["default"], null));
}
ReactDOM.render(/*#__PURE__*/React.createElement(Index, null), document.getElementById("root"));