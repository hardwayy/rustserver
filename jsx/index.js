import Test from './components/test.js';
import Component from './components/component.js';
function Index() {
    return (
        <>
            <h1>Test server</h1>
            <p>This local server is hosted in Rust and the pages are written in JSX</p>
            <Test />
            <Component/>
        </>
    );
}

ReactDOM.render(<Index />, document.getElementById("root"));
